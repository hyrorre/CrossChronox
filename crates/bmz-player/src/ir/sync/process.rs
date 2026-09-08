use super::*;

pub(super) async fn sync_pending_ir_jobs_with_filter(
    network_db: &mut NetworkDatabase,
    score_db_path: &Path,
    profile_root: &Path,
    logs_dir: &Path,
    ir_config: &IrConfig,
    now: i64,
    limit: u32,
    ignore_retry_backoff: bool,
    throttle: IrSyncThrottle,
    filter: Option<IrSyncJobFilter<'_>>,
) -> Result<IrSyncReport> {
    let mut report = IrSyncReport::default();
    let jobs = match filter {
        Some(IrSyncJobFilter {
            provider_key,
            account_id,
            kind,
            local_score_id: Some(local_score_id),
        }) => network_db.claim_pending_ir_score_job_for_local_score(
            provider_key,
            account_id,
            kind,
            local_score_id,
            now,
            ignore_retry_backoff,
        )?,
        Some(IrSyncJobFilter { provider_key, account_id, kind, local_score_id: None }) => {
            network_db.claim_pending_ir_score_jobs_for_kind(
                provider_key,
                account_id,
                kind,
                now,
                limit,
                ignore_retry_backoff,
            )?
        }
        None => network_db.claim_pending_ir_score_jobs(now, limit, ignore_retry_backoff)?,
    };
    let job_count = jobs.len();
    let replay_paths = match replay_paths_for_jobs(score_db_path, &jobs) {
        Ok(paths) => paths,
        Err(error) => {
            let message = format!("failed to resolve replay paths: {error:#}");
            for job in &jobs {
                network_db.mark_ir_score_job_failed(job.id, now, &message, None)?;
            }
            return Err(error);
        }
    };
    let batch_started = std::time::Instant::now();
    for (index, job) in jobs.into_iter().enumerate() {
        let job_now = now.saturating_add(batch_started.elapsed().as_secs() as i64);
        let Some(provider) = provider_config(ir_config, &job.provider) else {
            network_db.mark_ir_score_job_failed(
                job.id,
                job_now,
                "provider is not configured",
                None,
            )?;
            report.failed += 1;
            report
                .messages
                .push(format!("job {}: provider '{}' not configured", job.id, job.provider));
            continue;
        };
        // Keep the checked token for this job: loading it again during submission
        // could pick up a different account after a concurrent login.
        let credentials =
            match credentials_for_job(profile_root, provider, &job.account_id, job_now).await {
                Ok(credentials) => credentials,
                Err(error) => {
                    let message = format!("{error:#}");
                    network_db.mark_ir_score_job_failed(job.id, job_now, &message, None)?;
                    report.failed += 1;
                    report.messages.push(format!("job {}: {message}", job.id));
                    continue;
                }
            };
        match job.kind {
            IrJobKind::Replay => {
                let replay_result = submit_replay_job(
                    profile_root,
                    provider,
                    &job.payload_json,
                    replay_paths.get(&job.id).and_then(Option::as_deref),
                    job.local_score_id,
                    credentials,
                )
                .await;
                match replay_result {
                    Ok(()) => {
                        network_db.mark_ir_score_job_status(
                            job.id,
                            IrScoreJobStatus::Succeeded,
                            job_now,
                            "",
                        )?;
                        report.submitted += 1;
                    }
                    Err(error) => {
                        let message = format!("replay upload failed: {error:#}");
                        let _ = write_ir_submission_log(
                            logs_dir,
                            &job,
                            "failed",
                            "",
                            job_now,
                            &job.payload_json,
                            "",
                            &message,
                        );
                        network_db.mark_ir_score_job_failed(
                            job.id,
                            job_now,
                            &message,
                            retry_after_seconds_from_error(&error),
                        )?;
                        report.failed += 1;
                        report.messages.push(format!("job {}: {message}", job.id));
                        tracing::warn!(job_id = job.id, provider = job.provider, %message, "IR replay upload failed");
                    }
                }
            }
            IrJobKind::Attestation => {
                let attestation_result = submit_score_attestation_job(
                    profile_root,
                    provider,
                    &job.payload_json,
                    credentials,
                )
                .await;
                match attestation_result {
                    Ok((remote_score_id, request_json, response_json)) => {
                        let _ = write_ir_submission_log(
                            logs_dir,
                            &job,
                            "succeeded",
                            &remote_score_id,
                            job_now,
                            &request_json,
                            &response_json,
                            "",
                        );
                        network_db.mark_ir_score_job_status(
                            job.id,
                            IrScoreJobStatus::Succeeded,
                            job_now,
                            "",
                        )?;
                        report.submitted += 1;
                    }
                    Err(error) => {
                        let message = format!("score attestation failed: {error:#}");
                        let _ = write_ir_submission_log(
                            logs_dir,
                            &job,
                            "failed",
                            "",
                            job_now,
                            &job.payload_json,
                            "",
                            &message,
                        );
                        network_db.mark_ir_score_job_failed(
                            job.id,
                            job_now,
                            &message,
                            retry_after_seconds_from_error(&error),
                        )?;
                        report.failed += 1;
                        report.messages.push(format!("job {}: {message}", job.id));
                        tracing::warn!(job_id = job.id, provider = job.provider, %message, "IR score attestation failed");
                    }
                }
            }
            IrJobKind::Score | IrJobKind::Course => {
                let include_ranking = job.kind == IrJobKind::Score
                    && score_submission_includes_ranking(ir_config, &job.provider);
                let submit_result = match job.kind {
                    IrJobKind::Score => {
                        submit_job_payload(
                            profile_root,
                            provider,
                            &job.payload_json,
                            credentials,
                            include_ranking,
                        )
                        .await
                    }
                    IrJobKind::Course => {
                        submit_course_job_payload(
                            profile_root,
                            provider,
                            &job.payload_json,
                            credentials,
                        )
                        .await
                    }
                    IrJobKind::Replay | IrJobKind::Attestation => unreachable!(),
                };
                let Ok((request_json, response_json)) = submit_result else {
                    let error = submit_result.unwrap_err();
                    let message = format!("{error:#}");
                    let _ = write_ir_submission_log(
                        logs_dir,
                        &job,
                        "failed",
                        "",
                        job_now,
                        &job.payload_json,
                        "",
                        &message,
                    );
                    network_db.mark_ir_score_job_failed(
                        job.id,
                        job_now,
                        &message,
                        retry_after_seconds_from_error(&error),
                    )?;
                    report.failed += 1;
                    report.messages.push(format!("job {}: {message}", job.id));
                    tracing::warn!(job_id = job.id, provider = job.provider, %message, "IR score submission failed");
                    if index + 1 < job_count
                        && let Some(delay) = throttle.job_delay()
                    {
                        tokio::time::sleep(delay).await;
                    }
                    continue;
                };
                let parsed_response =
                    serde_json::from_str::<crate::ir::types::IrSubmitResponse>(&response_json).ok();
                if include_ranking
                    && let Some(ranking_response) = parsed_response
                        .as_ref()
                        .and_then(|response| response.rankings.get(&IrRankingScope::Global))
                        .filter(|ranking| ranking.succeeded)
                    && let Some(ranking) = ranking_response.data.clone()
                {
                    report.included_rankings.push(IrIncludedRanking {
                        provider: job.provider.clone(),
                        account_id: job.account_id.clone(),
                        kind: job.kind,
                        local_score_id: job.local_score_id,
                        previous_rank: ranking_response.previous_rank,
                        ranking,
                    });
                }
                let remote_score_id = parsed_response
                    .as_ref()
                    .and_then(|response| response.score_id.clone())
                    .or_else(|| {
                        serde_json::from_str::<serde_json::Value>(&response_json).ok().and_then(
                            |value| value.get("course_score_id")?.as_str().map(str::to_string),
                        )
                    })
                    .unwrap_or_default();
                let completion =
                    replay_job_for_score(&job, &remote_score_id, job_now).and_then(|replay_job| {
                        let log_path = write_ir_submission_log(
                            logs_dir,
                            &job,
                            "succeeded",
                            &remote_score_id,
                            job_now,
                            &request_json,
                            &response_json,
                            "",
                        );
                        network_db.complete_ir_score_job(
                            &NewIrScoreSubmission {
                                job_id: job.id,
                                provider: job.provider.clone(),
                                account_id: job.account_id.clone(),
                                kind: job.kind,
                                local_score_id: job.local_score_id,
                                remote_score_id: remote_score_id.clone(),
                                status: "succeeded".to_string(),
                                submitted_at: job_now,
                                log_path,
                                error: String::new(),
                            },
                            replay_job.as_ref(),
                            &response_json,
                        )?;
                        Ok(())
                    });
                match completion {
                    Ok(()) => {
                        report.submitted += 1;
                    }
                    Err(error) => {
                        let message = format!("failed to complete IR score job: {error:#}");
                        let _ = write_ir_submission_log(
                            logs_dir,
                            &job,
                            "failed",
                            &remote_score_id,
                            job_now,
                            &request_json,
                            &response_json,
                            &message,
                        );
                        network_db.mark_ir_score_job_failed(
                            job.id,
                            job_now,
                            &message,
                            retry_after_seconds_from_error(&error),
                        )?;
                        report.failed += 1;
                        report.messages.push(format!("job {}: {message}", job.id));
                        tracing::warn!(job_id = job.id, provider = job.provider, %message, "IR score completion failed");
                    }
                }
            }
        }
        if index + 1 < job_count
            && let Some(delay) = throttle.job_delay()
        {
            tokio::time::sleep(delay).await;
        }
    }
    let finished_at = now.saturating_add(batch_started.elapsed().as_secs() as i64);
    let pruned = network_db.prune_succeeded_ir_score_jobs(finished_at)?;
    if pruned > 0 {
        tracing::debug!(pruned, "pruned succeeded IR score jobs");
    }
    Ok(report)
}
