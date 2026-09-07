use rusqlite::Connection;

use super::*;
use crate::storage::common::configure_connection;
use crate::storage::migration::{NETWORK_MIGRATIONS, run_migrations};

fn open_network_db() -> NetworkDatabase {
    let mut conn = Connection::open_in_memory().unwrap();
    configure_connection(&conn).unwrap();
    run_migrations(&mut conn, NETWORK_MIGRATIONS).unwrap();
    NetworkDatabase::from_connection(conn)
}

fn enqueue_test_job(db: &mut NetworkDatabase, local_score_id: i64, now: i64) -> i64 {
    db.enqueue_ir_score_job(&NewIrScoreJob {
        provider: "bmz-official".to_string(),
        account_id: "account-1".to_string(),
        kind: IrJobKind::Score,
        local_score_id,
        chart_sha256: [local_score_id as u8; 32],
        ln_policy: LnScorePolicy::ForceLn,
        payload_json: "{}".to_string(),
        now,
    })
    .unwrap()
}

#[test]
fn uuid_submission_survives_failure_reopen_and_local_id_reuse() {
    let path = std::env::temp_dir()
        .join(format!("{}.db", crate::ir::payload::new_score_idempotency_key().unwrap()));
    crate::storage::migration::migrate_network_db(&path).unwrap();
    let mut db = NetworkDatabase::open(&path).unwrap();
    let key_a = crate::ir::payload::new_score_idempotency_key().unwrap();
    let key_b = crate::ir::payload::new_score_idempotency_key().unwrap();
    assert_ne!(key_a, key_b);
    for key in [&key_a, &key_b] {
        let uuid = uuid::Uuid::parse_str(key.strip_prefix("bmz-score-v2-").unwrap()).unwrap();
        assert_eq!(uuid.get_version_num(), 4);
    }
    let job = NewIrScoreJob {
        provider: "bmz".into(),
        account_id: "player".into(),
        kind: IrJobKind::Score,
        local_score_id: 95,
        chart_sha256: [7; 32],
        ln_policy: LnScorePolicy::ForceLn,
        payload_json: serde_json::json!({"idempotency_key": key_a, "ex_score": 1234}).to_string(),
        now: 100,
    };
    let first = db.enqueue_ir_score_job(&job).unwrap();
    assert_eq!(db.enqueue_ir_score_job(&job).unwrap(), first);
    let claimed = db.claim_pending_ir_score_jobs(100, 10, false).unwrap();
    assert_eq!(claimed[0].payload_json, job.payload_json);
    db.mark_ir_score_job_failed(first, 101, "network failure", None).unwrap();
    drop(db);
    let mut db = NetworkDatabase::open(&path).unwrap();
    let retry = db.claim_pending_ir_score_jobs(162, 10, false).unwrap();
    assert_eq!(retry.len(), 1);
    assert_eq!(retry[0].payload_json, job.payload_json);
    let response = r#"{"accepted":true,"score_id":"old"}"#;
    db.complete_ir_score_job(
        &NewIrScoreSubmission {
            job_id: first,
            provider: job.provider.clone(),
            account_id: job.account_id.clone(),
            kind: job.kind,
            local_score_id: 95,
            remote_score_id: "old".into(),
            status: "succeeded".into(),
            submitted_at: 163,
            log_path: String::new(),
            error: String::new(),
        },
        None,
        response,
    )
    .unwrap();
    // A recreated score DB assigns 95 to a different play.
    let second = db
        .enqueue_ir_score_job(&NewIrScoreJob {
            chart_sha256: [8; 32],
            payload_json: serde_json::json!({"idempotency_key": key_b, "ex_score": 2776})
                .to_string(),
            now: 200,
            ..job
        })
        .unwrap();
    assert_ne!(first, second);
    let current = db.ir_score_jobs_for_local_score(IrJobKind::Score, 95).unwrap();
    assert_eq!(current.len(), 1);
    assert_eq!(current[0].id, second);
    assert_eq!(current[0].chart_sha256, [8; 32]);
    assert!(
        db.latest_ir_score_submission_response("bmz", "player", IrJobKind::Score, 95)
            .unwrap()
            .is_none()
    );
    db.mark_ir_score_job_failed(second, 201, "409 Conflict: idempotency key collision", None)
        .unwrap();
    assert_eq!(db.ir_score_jobs_for_local_score(IrJobKind::Score, 95).unwrap()[0].status, "failed");
    let old_status: String = db
        .conn()
        .query_row("SELECT status FROM ir_score_jobs WHERE id = ?1", [first], |row| row.get(0))
        .unwrap();
    assert_eq!(old_status, "succeeded");
    drop(db);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn submission_identity_migration_preserves_jobs_and_submission_history() {
    let mut conn = Connection::open_in_memory().unwrap();
    configure_connection(&conn).unwrap();
    run_migrations(&mut conn, &NETWORK_MIGRATIONS[..3]).unwrap();
    conn.execute_batch(
        "INSERT INTO ir_score_jobs
        (id, provider, account_id, kind, local_score_id, chart_sha256, ln_policy, payload_json,
         status, attempt_count, next_attempt_at, last_error, created_at, updated_at)
        VALUES (5, 'bmz', 'player', 'score', 95, 'hash', 'ForceLn',
                '{\"idempotency_key\":\"bmz-score-95\"}', 'failed', 3, 999, 'offline', 1, 2);
        INSERT INTO ir_score_submissions (job_id, provider, local_score_id, status, submitted_at)
        VALUES (5, 'bmz', 95, 'failed', 2);",
    )
    .unwrap();
    run_migrations(&mut conn, NETWORK_MIGRATIONS).unwrap();
    let row: (String, String, i64, i64) = conn.query_row(
        "SELECT submission_key, status, attempt_count, next_attempt_at FROM ir_score_jobs WHERE id = 5",
        [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))).unwrap();
    assert_eq!(row, ("bmz-score-95".into(), "failed".into(), 3, 999));
    assert_eq!(
        conn.query_row("SELECT job_id FROM ir_score_submissions", [], |row| row.get::<_, i64>(0))
            .unwrap(),
        5
    );
    assert!(
        conn.prepare("PRAGMA foreign_key_check")
            .unwrap()
            .query([])
            .unwrap()
            .next()
            .unwrap()
            .is_none()
    );
    conn.execute("DELETE FROM ir_score_jobs WHERE id = 5", []).unwrap();
    assert_eq!(
        conn.query_row("SELECT COUNT(*) FROM ir_score_submissions", [], |row| row.get::<_, i64>(0))
            .unwrap(),
        0
    );
}

#[test]
fn reused_local_id_keeps_unfinished_jobs_separate_for_each_provider() {
    let mut db = open_network_db();
    for provider in ["bmz", "rian-ir"] {
        let mut job = NewIrScoreJob {
            provider: provider.into(), account_id: "player".into(), kind: IrJobKind::Score,
            local_score_id: 95, chart_sha256: [7; 32], ln_policy: LnScorePolicy::ForceLn,
            payload_json: serde_json::json!({"idempotency_key": crate::ir::payload::new_score_idempotency_key().unwrap()}).to_string(),
            now: 100,
        };
        let first = db.enqueue_ir_score_job(&job).unwrap();
        db.mark_ir_score_job_failed(first, 101, "offline", None).unwrap();
        let old_payload = job.payload_json.clone();
        job.chart_sha256 = [8; 32];
        assert!(db.enqueue_ir_score_job(&job).is_err());
        job.payload_json = serde_json::json!({"idempotency_key": crate::ir::payload::new_score_idempotency_key().unwrap()}).to_string();
        let second = db.enqueue_ir_score_job(&job).unwrap();
        assert_ne!(first, second);
        let current = db
            .claim_pending_ir_score_job_for_local_score(
                provider,
                "player",
                IrJobKind::Score,
                95,
                200,
                true,
            )
            .unwrap();
        assert_eq!(current.len(), 1);
        assert_eq!(current[0].id, second);
        let old: (String, String) = db
            .conn()
            .query_row(
                "SELECT payload_json, status FROM ir_score_jobs WHERE id = ?1",
                [first],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(old, (old_payload, "failed".into()));
    }
}

#[test]
fn rival_score_cache_replaces_one_rival_snapshot() {
    let mut db = open_network_db();
    let first = IrRivalScoreRecord {
        chart_sha256: [7; 32],
        ln_mode: 1,
        ex_score: 1234,
        clear_type: 5,
        max_combo: 500,
        min_bp: 12,
        play_option: 0,
        arrange_1p: "f-random".to_string(),
        arrange_2p: "normal".to_string(),
        double_option: "off".to_string(),
        play_seed: Some(42),
    };
    db.replace_rival_scores("rian-ir", "42", "beatoraja", std::slice::from_ref(&first), "v1", 10)
        .unwrap();
    assert_eq!(db.rival_scores("rian-ir", "42", "beatoraja").unwrap(), vec![first]);
    assert_eq!(
        db.rival_score_cache_state("rian-ir", "42", "beatoraja").unwrap(),
        IrRivalScoreCacheState { etag: "v1".to_string(), fetched_at: 10 }
    );

    db.replace_rival_scores("rian-ir", "42", "beatoraja", &[], "v2", 20).unwrap();
    assert!(db.rival_scores("rian-ir", "42", "beatoraja").unwrap().is_empty());
}

#[test]
fn rival_score_cache_keeps_scores_and_etag_when_snapshot_commit_fails() {
    let mut db = open_network_db();
    let original = IrRivalScoreRecord {
        chart_sha256: [7; 32],
        ln_mode: 1,
        ex_score: 1234,
        clear_type: 5,
        max_combo: 500,
        min_bp: 12,
        play_option: 0,
        arrange_1p: "normal".to_string(),
        arrange_2p: "normal".to_string(),
        double_option: "off".to_string(),
        play_seed: None,
    };
    db.replace_rival_scores(
        "bms-ir",
        "42",
        "beatoraja",
        std::slice::from_ref(&original),
        "old-etag",
        10,
    )
    .unwrap();

    let duplicate = IrRivalScoreRecord { chart_sha256: [9; 32], ..original.clone() };
    assert!(
        db.replace_rival_scores(
            "bms-ir",
            "42",
            "beatoraja",
            &[duplicate.clone(), duplicate],
            "new-etag",
            20,
        )
        .is_err()
    );

    assert_eq!(db.rival_scores("bms-ir", "42", "beatoraja").unwrap(), vec![original]);
    assert_eq!(
        db.rival_score_cache_state("bms-ir", "42", "beatoraja").unwrap(),
        IrRivalScoreCacheState { etag: "old-etag".to_string(), fetched_at: 10 }
    );
}

#[test]
fn ir_score_jobs_round_trip_and_dedupe_by_provider_account_kind_score() {
    let mut db = open_network_db();
    let job = NewIrScoreJob {
        provider: "bmz-official".to_string(),
        account_id: "account-1".to_string(),
        kind: IrJobKind::Score,
        local_score_id: 42,
        chart_sha256: [7; 32],
        ln_policy: LnScorePolicy::ForceLn,
        payload_json: "{\"score\":1}".to_string(),
        now: 100,
    };
    let first_id = db.enqueue_ir_score_job(&job).unwrap();
    let mut updated = job.clone();
    updated.payload_json = "{\"score\":2}".to_string();
    updated.now = 200;
    let second_id = db.enqueue_ir_score_job(&updated).unwrap();

    assert_eq!(first_id, second_id);
    let pending = db.pending_ir_score_jobs(200, 10).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].payload_json, "{\"score\":2}");
    assert_eq!(pending[0].ln_policy, LnScorePolicy::ForceLn);

    let log_path = "ir-submissions.jsonl".to_string();
    let submission_id = db
        .insert_ir_score_submission(&NewIrScoreSubmission {
            job_id: first_id,
            provider: "bmz-official".to_string(),
            account_id: "account-1".to_string(),
            kind: IrJobKind::Score,
            local_score_id: 42,
            remote_score_id: "sc_remote".to_string(),
            status: "succeeded".to_string(),
            submitted_at: 220,
            log_path: log_path.clone(),
            error: String::new(),
        })
        .unwrap();
    assert!(submission_id > 0);
    let stored_log_path: String = db
        .conn()
        .query_row(
            "SELECT log_path FROM ir_score_submissions WHERE id = ?1",
            [submission_id],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(stored_log_path, log_path);

    db.mark_ir_score_job_status(first_id, IrScoreJobStatus::Succeeded, 230, "").unwrap();
    assert!(db.pending_ir_score_jobs(300, 10).unwrap().is_empty());
    let payload_json: String = db
        .conn()
        .query_row("SELECT payload_json FROM ir_score_jobs WHERE id = ?1", [first_id], |row| {
            row.get(0)
        })
        .unwrap();
    assert!(payload_json.is_empty());
}

#[test]
fn cleanup_removes_selected_provider_records_for_removed_local_scores() {
    let mut db = open_network_db();
    let score_job = db
        .enqueue_ir_score_job(&NewIrScoreJob {
            provider: "bmz".to_string(),
            account_id: "account-1".to_string(),
            kind: IrJobKind::Score,
            local_score_id: 42,
            chart_sha256: [1; 32],
            ln_policy: LnScorePolicy::AutoLn,
            payload_json: "{}".to_string(),
            now: 100,
        })
        .unwrap();
    db.enqueue_ir_score_job(&NewIrScoreJob {
        provider: "bmz".to_string(),
        account_id: "account-1".to_string(),
        kind: IrJobKind::Attestation,
        local_score_id: 42,
        chart_sha256: [0; 32],
        ln_policy: LnScorePolicy::AutoLn,
        payload_json: "{}".to_string(),
        now: 100,
    })
    .unwrap();
    db.insert_ir_score_submission(&NewIrScoreSubmission {
        job_id: score_job,
        provider: "bmz".to_string(),
        account_id: "account-1".to_string(),
        kind: IrJobKind::Score,
        local_score_id: 42,
        remote_score_id: "remote-42".to_string(),
        status: "succeeded".to_string(),
        submitted_at: 101,
        log_path: String::new(),
        error: String::new(),
    })
    .unwrap();
    db.enqueue_ir_score_job(&NewIrScoreJob {
        provider: "other".to_string(),
        account_id: "account-2".to_string(),
        kind: IrJobKind::Score,
        local_score_id: 42,
        chart_sha256: [2; 32],
        ln_policy: LnScorePolicy::AutoLn,
        payload_json: "{}".to_string(),
        now: 100,
    })
    .unwrap();

    assert_eq!(
        db.successful_ir_score_submissions_for_local_scores(&[42]).unwrap(),
        vec![IrSubmittedScoreLink {
            provider: "bmz".to_string(),
            account_id: "account-1".to_string(),
            local_score_id: 42,
            remote_score_id: "remote-42".to_string(),
        }]
    );
    assert_eq!(
        db.purge_ir_records_for_local_scores("bmz", "account-1", &[42]).unwrap(),
        IrLocalScoreCleanupReport { removed_jobs: 2, removed_submissions: 1 }
    );
    let remaining: Vec<(String, String)> = db
        .conn()
        .prepare("SELECT provider, account_id FROM ir_score_jobs ORDER BY id")
        .unwrap()
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    assert_eq!(remaining, vec![("other".to_string(), "account-2".to_string())]);
}

#[test]
fn attempt_job_lookup_keeps_kind_and_local_score_isolated() {
    let mut db = open_network_db();
    for (kind, local_score_id) in
        [(IrJobKind::Score, 42), (IrJobKind::Course, 42), (IrJobKind::Score, 43)]
    {
        db.enqueue_ir_score_job(&NewIrScoreJob {
            provider: format!("provider-{local_score_id}-{}", kind.as_str()),
            account_id: "account".to_string(),
            kind,
            local_score_id,
            chart_sha256: [local_score_id as u8; 32],
            ln_policy: LnScorePolicy::AutoLn,
            payload_json: "{}".to_string(),
            now: 100,
        })
        .unwrap();
    }

    let jobs = db.ir_score_jobs_for_local_score(IrJobKind::Score, 42).unwrap();

    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].kind, IrJobKind::Score);
    assert_eq!(jobs[0].local_score_id, 42);
}

#[test]
fn completing_score_job_atomically_enqueues_replay_job() {
    let mut db = open_network_db();
    let score_job_id = enqueue_test_job(&mut db, 42, 100);
    let replay_job = NewIrScoreJob {
        provider: "bmz-official".to_string(),
        account_id: "account-1".to_string(),
        kind: IrJobKind::Replay,
        local_score_id: 42,
        chart_sha256: [42; 32],
        ln_policy: LnScorePolicy::ForceLn,
        payload_json: r#"{"remote_score_id":"remote-42"}"#.to_string(),
        now: 200,
    };

    db.complete_ir_score_job(
        &NewIrScoreSubmission {
            job_id: score_job_id,
            provider: "bmz-official".to_string(),
            account_id: "account-1".to_string(),
            kind: IrJobKind::Score,
            local_score_id: 42,
            remote_score_id: "remote-42".to_string(),
            status: "succeeded".to_string(),
            submitted_at: 200,
            log_path: "ir-submissions.jsonl".to_string(),
            error: String::new(),
        },
        Some(&replay_job),
        r#"{"accepted":true,"score_id":"remote-42","best_updated":true,"rankings":{}}"#,
    )
    .unwrap();

    assert_eq!(
        db.latest_ir_score_submission_response("bmz-official", "account-1", IrJobKind::Score, 42,)
            .unwrap()
            .as_deref(),
        Some(r#"{"accepted":true,"score_id":"remote-42","best_updated":true,"rankings":{}}"#),
    );

    let (score_status, score_payload): (String, String) = db
        .conn()
        .query_row(
            "SELECT status, payload_json FROM ir_score_jobs WHERE id = ?1",
            [score_job_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    assert_eq!(score_status, "succeeded");
    assert!(score_payload.is_empty());

    let replay_jobs = db.pending_ir_score_jobs(200, 10).unwrap();
    assert_eq!(replay_jobs.len(), 1);
    assert_eq!(replay_jobs[0].kind, IrJobKind::Replay);
    assert_eq!(replay_jobs[0].payload_json, replay_job.payload_json);

    let submissions: i64 = db
        .conn()
        .query_row("SELECT COUNT(*) FROM ir_score_submissions", [], |row| row.get(0))
        .unwrap();
    assert_eq!(submissions, 1);
}

#[test]
fn ir_score_job_failures_back_off_progressively() {
    let mut db = open_network_db();
    let job_id = db
        .enqueue_ir_score_job(&NewIrScoreJob {
            provider: "bmz-official".to_string(),
            account_id: "account-1".to_string(),
            kind: IrJobKind::Score,
            local_score_id: 42,
            chart_sha256: [7; 32],
            ln_policy: LnScorePolicy::ForceLn,
            payload_json: "{}".to_string(),
            now: 0,
        })
        .unwrap();

    // docs/ir.md: 1分 → 5分 → 30分 → 2時間 → 24時間 (以降は 24時間維持)。
    let expected_delays = [60, 300, 1800, 7200, 86_400, 86_400];
    for (attempt, delay) in expected_delays.into_iter().enumerate() {
        let now = (attempt as i64 + 1) * 1_000_000;
        db.mark_ir_score_job_status(job_id, IrScoreJobStatus::Failed, now, "boom").unwrap();
        let (attempt_count, next_attempt_at): (u32, i64) = db
            .conn()
            .query_row(
                "SELECT attempt_count, next_attempt_at FROM ir_score_jobs WHERE id = ?1",
                [job_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(attempt_count, attempt as u32 + 1);
        assert_eq!(next_attempt_at, now + delay, "attempt {attempt}");
    }
}

#[test]
fn ir_score_job_failure_honors_retry_after() {
    let mut db = open_network_db();
    let job_id = enqueue_test_job(&mut db, 42, 100);

    db.mark_ir_score_job_failed(job_id, 200, "rate limited", Some(777)).unwrap();

    let (status, attempt_count, next_attempt_at): (String, u32, i64) = db
        .conn()
        .query_row(
            "SELECT status, attempt_count, next_attempt_at
                 FROM ir_score_jobs
                 WHERE id = ?1",
            [job_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    assert_eq!(status, "failed");
    assert_eq!(attempt_count, 1);
    assert_eq!(next_attempt_at, 977);
}

#[test]
fn claiming_ir_jobs_marks_the_whole_batch_sending() {
    let mut db = open_network_db();
    let first = enqueue_test_job(&mut db, 1, 100);
    let second = enqueue_test_job(&mut db, 2, 100);

    let claimed = db.claim_pending_ir_score_jobs(100, 20, false).unwrap();
    assert_eq!(claimed.iter().map(|job| job.id).collect::<Vec<_>>(), vec![first, second]);
    assert!(db.claim_pending_ir_score_jobs(100, 20, false).unwrap().is_empty());

    let statuses = db
        .conn()
        .prepare("SELECT status FROM ir_score_jobs ORDER BY id")
        .unwrap()
        .query_map([], |row| row.get::<_, String>(0))
        .unwrap()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();
    assert_eq!(statuses, vec!["sending", "sending"]);
}

#[test]
fn claiming_ir_jobs_for_kind_keeps_other_jobs_pending() {
    let mut db = open_network_db();
    let score = enqueue_test_job(&mut db, 1, 100);
    let attestation = db
        .enqueue_ir_score_job(&NewIrScoreJob {
            provider: "bmz-official".to_string(),
            account_id: "account-1".to_string(),
            kind: IrJobKind::Attestation,
            local_score_id: 2,
            chart_sha256: [0; 32],
            ln_policy: LnScorePolicy::AutoLn,
            payload_json: r#"{"remote_score_id":"remote-2"}"#.to_string(),
            now: 100,
        })
        .unwrap();
    db.enqueue_ir_score_job(&NewIrScoreJob {
        provider: "bmz-official".to_string(),
        account_id: "account-2".to_string(),
        kind: IrJobKind::Attestation,
        local_score_id: 3,
        chart_sha256: [0; 32],
        ln_policy: LnScorePolicy::AutoLn,
        payload_json: r#"{"remote_score_id":"remote-3"}"#.to_string(),
        now: 100,
    })
    .unwrap();

    let pending = db
        .pending_ir_score_jobs_for_kind(
            "bmz-official",
            "account-1",
            IrJobKind::Attestation,
            100,
            10,
            true,
        )
        .unwrap();
    assert_eq!(pending.iter().map(|job| job.id).collect::<Vec<_>>(), vec![attestation]);

    let claimed = db
        .claim_pending_ir_score_jobs_for_kind(
            "bmz-official",
            "account-1",
            IrJobKind::Attestation,
            100,
            10,
            true,
        )
        .unwrap();
    assert_eq!(claimed.iter().map(|job| job.id).collect::<Vec<_>>(), vec![attestation]);

    let score_status: String = db
        .conn()
        .query_row("SELECT status FROM ir_score_jobs WHERE id = ?1", [score], |row| row.get(0))
        .unwrap();
    assert_eq!(score_status, "pending");
}

#[test]
fn claiming_ir_job_for_local_score_targets_only_current_attempt() {
    let mut db = open_network_db();
    let previous = enqueue_test_job(&mut db, 1, 100);
    let current = enqueue_test_job(&mut db, 2, 100);
    let other_provider = db
        .enqueue_ir_score_job(&NewIrScoreJob {
            provider: "rian-ir".to_string(),
            account_id: "account-2".to_string(),
            kind: IrJobKind::Score,
            local_score_id: 2,
            chart_sha256: [2; 32],
            ln_policy: LnScorePolicy::ForceLn,
            payload_json: "{}".to_string(),
            now: 100,
        })
        .unwrap();

    let claimed = db
        .claim_pending_ir_score_job_for_local_score(
            "bmz-official",
            "account-1",
            IrJobKind::Score,
            2,
            100,
            false,
        )
        .unwrap();

    assert_eq!(claimed.iter().map(|job| job.id).collect::<Vec<_>>(), vec![current]);
    let statuses = db
        .conn()
        .prepare("SELECT id, status FROM ir_score_jobs ORDER BY id")
        .unwrap()
        .query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)))
        .unwrap()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();
    assert_eq!(
        statuses,
        vec![
            (previous, "pending".to_string()),
            (current, "sending".to_string()),
            (other_provider, "pending".to_string()),
        ]
    );
}

#[test]
fn existing_ir_job_is_detected_before_backfill_reenqueue() {
    let mut db = open_network_db();
    enqueue_test_job(&mut db, 42, 100);

    assert!(db.has_ir_score_job("bmz-official", "account-1", IrJobKind::Score, 42).unwrap());
    assert!(!db.has_ir_score_job("bmz-official", "account-2", IrJobKind::Score, 42).unwrap());
    assert_eq!(
        db.unfinished_ir_score_job_count_for_kind("bmz-official", "account-1", IrJobKind::Score,)
            .unwrap(),
        1
    );

    let job_id = db
        .conn()
        .query_row("SELECT id FROM ir_score_jobs WHERE local_score_id = 42", [], |row| row.get(0))
        .unwrap();
    db.mark_ir_score_job_status(job_id, IrScoreJobStatus::Succeeded, 200, "").unwrap();
    assert_eq!(
        db.unfinished_ir_score_job_count_for_kind("bmz-official", "account-1", IrJobKind::Score,)
            .unwrap(),
        0
    );
}

#[test]
fn submitted_scores_enqueue_one_attestation_job() {
    let mut db = open_network_db();
    let score_job_id = enqueue_test_job(&mut db, 42, 100);
    db.insert_ir_score_submission(&NewIrScoreSubmission {
        job_id: score_job_id,
        provider: "bmz-official".to_string(),
        account_id: "account-1".to_string(),
        kind: IrJobKind::Score,
        local_score_id: 42,
        remote_score_id: "remote-42".to_string(),
        status: "succeeded".to_string(),
        submitted_at: 200,
        log_path: "ir-submissions.jsonl".to_string(),
        error: String::new(),
    })
    .unwrap();

    assert_eq!(db.enqueue_ir_score_attestation_jobs("bmz-official", "account-1", 300).unwrap(), 1);
    assert_eq!(db.enqueue_ir_score_attestation_jobs("bmz-official", "account-1", 301).unwrap(), 0);

    let jobs = db.pending_ir_score_jobs(300, 10).unwrap();
    let job = jobs.iter().find(|job| job.kind == IrJobKind::Attestation).unwrap();
    let payload: serde_json::Value = serde_json::from_str(&job.payload_json).unwrap();
    assert_eq!(job.local_score_id, 42);
    assert_eq!(payload["remote_score_id"], "remote-42");
}

#[test]
fn manual_ir_sync_can_ignore_retry_backoff() {
    let mut db = open_network_db();
    let job_id = db
        .enqueue_ir_score_job(&NewIrScoreJob {
            provider: "bmz-official".to_string(),
            account_id: "account-1".to_string(),
            kind: IrJobKind::Score,
            local_score_id: 42,
            chart_sha256: [7; 32],
            ln_policy: LnScorePolicy::ForceLn,
            payload_json: "{}".to_string(),
            now: 100,
        })
        .unwrap();

    db.mark_ir_score_job_status(job_id, IrScoreJobStatus::Failed, 200, "boom").unwrap();

    assert!(db.pending_ir_score_jobs(201, 10).unwrap().is_empty());
    let pending = db.pending_ir_score_jobs_ignoring_backoff(201, 10).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, job_id);
    assert_eq!(pending[0].status, "failed");
}

#[test]
fn stale_sending_ir_score_jobs_are_retried() {
    let mut db = open_network_db();
    let job_id = db
        .enqueue_ir_score_job(&NewIrScoreJob {
            provider: "bmz-official".to_string(),
            account_id: "account-1".to_string(),
            kind: IrJobKind::Score,
            local_score_id: 42,
            chart_sha256: [7; 32],
            ln_policy: LnScorePolicy::ForceLn,
            payload_json: "{}".to_string(),
            now: 100,
        })
        .unwrap();

    db.mark_ir_score_job_status(job_id, IrScoreJobStatus::Sending, 200, "").unwrap();

    assert!(db.pending_ir_score_jobs(499, 10).unwrap().is_empty());
    let pending = db.pending_ir_score_jobs(500, 10).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, job_id);
    assert_eq!(pending[0].status, "sending");
}

#[test]
fn prune_succeeded_ir_score_jobs_keeps_recent_and_unfinished_jobs() {
    let mut db = open_network_db();
    let stale_a = enqueue_test_job(&mut db, 1, 0);
    let stale_b = enqueue_test_job(&mut db, 2, 0);
    let retained_by_count = enqueue_test_job(&mut db, 3, 0);
    let retained_by_age = enqueue_test_job(&mut db, 4, 9_500);
    let failed = enqueue_test_job(&mut db, 5, 0);

    db.mark_ir_score_job_status(stale_a, IrScoreJobStatus::Succeeded, 100, "").unwrap();
    db.mark_ir_score_job_status(stale_b, IrScoreJobStatus::Succeeded, 200, "").unwrap();
    db.mark_ir_score_job_status(retained_by_count, IrScoreJobStatus::Succeeded, 300, "").unwrap();
    db.mark_ir_score_job_status(retained_by_age, IrScoreJobStatus::Succeeded, 9_500, "").unwrap();
    db.mark_ir_score_job_status(failed, IrScoreJobStatus::Failed, 100, "boom").unwrap();

    db.insert_ir_score_submission(&NewIrScoreSubmission {
        job_id: stale_a,
        provider: "bmz-official".to_string(),
        account_id: "account-1".to_string(),
        kind: IrJobKind::Score,
        local_score_id: 1,
        remote_score_id: "remote-a".to_string(),
        status: "succeeded".to_string(),
        submitted_at: 100,
        log_path: "ir-submissions.jsonl".to_string(),
        error: String::new(),
    })
    .unwrap();

    let deleted = db.prune_succeeded_ir_score_jobs_with_policy(10_000, 1_000, 2).unwrap();
    assert_eq!(deleted, 2);

    let remaining: Vec<i64> = db
        .conn()
        .prepare("SELECT id FROM ir_score_jobs ORDER BY id")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    assert_eq!(remaining, vec![retained_by_count, retained_by_age, failed]);

    let stale_submission_count: i64 = db
        .conn()
        .query_row(
            "SELECT COUNT(*) FROM ir_score_submissions WHERE job_id = ?1",
            [stale_a],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(stale_submission_count, 0);
}
