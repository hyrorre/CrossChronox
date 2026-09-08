use super::*;

impl SelectIrRanking {
    /// 選択中譜面のスキン用 snapshot。IR 未設定なら Offline。
    pub fn snapshot_for(
        &self,
        ir_config: &IrConfig,
        selected: Option<[u8; 32]>,
    ) -> ResultIrSnapshot {
        self.snapshot_for_scope(ir_config, selected, SelectIrRankingScope::Global)
    }

    /// 対応 Select スキン用の、現在選択されている scope の snapshot。
    pub fn active_snapshot_for(
        &self,
        ir_config: &IrConfig,
        selected: Option<[u8; 32]>,
    ) -> ResultIrSnapshot {
        self.snapshot_for_scope(ir_config, selected, self.active_scope)
    }

    /// Select スキンの scope binding に従う snapshot を返す。
    ///
    /// `Global` は既存スキンとの互換性を保ち、常に全体ランキングを返す。
    pub fn snapshot_for_binding(
        &self,
        ir_config: &IrConfig,
        selected: Option<[u8; 32]>,
        binding: bmz_render::skin::IrScopeBinding,
    ) -> ResultIrSnapshot {
        match binding {
            bmz_render::skin::IrScopeBinding::Global => self.snapshot_for(ir_config, selected),
            bmz_render::skin::IrScopeBinding::Active => {
                self.active_snapshot_for(ir_config, selected)
            }
        }
    }

    /// 指定 scope の snapshot。Rival scope が未取得・非対応なら Global へ戻す。
    pub fn snapshot_for_scope(
        &self,
        ir_config: &IrConfig,
        selected: Option<[u8; 32]>,
        requested_scope: SelectIrRankingScope,
    ) -> ResultIrSnapshot {
        if enabled_provider(ir_config).is_none() {
            return ResultIrSnapshot::default();
        }
        let Some(sha256) = selected else {
            return snapshot_with_provider(ResultIrSnapshot::default(), ir_config);
        };
        let scope = if requested_scope == SelectIrRankingScope::SelfAndRivals
            && self.supports_scope(ir_config, Some(sha256), requested_scope)
        {
            requested_scope
        } else {
            SelectIrRankingScope::Global
        };
        let mut snapshot = self
            .cache
            .get(&sha256)
            .and_then(|entry| match scope {
                SelectIrRankingScope::Global => Some(entry.global.clone()),
                SelectIrRankingScope::SelfAndRivals => entry.self_and_rivals.clone(),
            })
            .map(|mut snapshot| {
                let entry = &self.cache[&sha256];
                match snapshot.state {
                    SkinIrState::Loaded => {
                        snapshot.connect_success_ms = Some(elapsed_since_ms(entry.completed_at));
                    }
                    SkinIrState::Failed => {
                        snapshot.connect_fail_ms = Some(elapsed_since_ms(entry.completed_at));
                    }
                    _ => {}
                }
                snapshot
            })
            .unwrap_or_else(|| {
                let begin_ms = self
                    .in_flight
                    .as_ref()
                    .filter(|(_, in_flight_sha, _)| *in_flight_sha == sha256)
                    .map(|(_, _, started_at)| elapsed_since_ms(*started_at));
                ResultIrSnapshot {
                    state: if begin_ms.is_some() {
                        SkinIrState::Loading
                    } else {
                        SkinIrState::Waiting
                    },
                    connect_begin_ms: begin_ms,
                    ..Default::default()
                }
            });
        snapshot.scope = match scope {
            SelectIrRankingScope::Global => SkinIrScope::Global,
            SelectIrRankingScope::SelfAndRivals => SkinIrScope::Rival,
        };
        snapshot.global_scope_supported = true;
        snapshot.rival_scope_supported =
            self.supports_scope(ir_config, Some(sha256), SelectIrRankingScope::SelfAndRivals);
        snapshot_with_provider(snapshot, ir_config)
    }

    pub fn active_scope(&self) -> SelectIrRankingScope {
        self.active_scope
    }

    pub fn battle_entries_for(&self, selected: Option<[u8; 32]>) -> &[SelectIrBattleEntry] {
        let Some(entry) = selected.and_then(|sha256| self.cache.get(&sha256)) else {
            return &[];
        };
        match self.active_scope {
            SelectIrRankingScope::Global => &entry.global_battle_entries,
            SelectIrRankingScope::SelfAndRivals => &entry.self_and_rivals_battle_entries,
        }
    }

    /// scope を選ぶ。Self-and-Rivals は取得済みの場合だけ選択できる。
    pub fn select_scope(
        &mut self,
        ir_config: &IrConfig,
        selected: Option<[u8; 32]>,
        scope: SelectIrRankingScope,
    ) -> bool {
        if self.active_scope == scope || !self.supports_scope(ir_config, selected, scope) {
            return false;
        }
        self.active_scope = scope;
        true
    }

    pub fn toggle_scope(&mut self, ir_config: &IrConfig, selected: Option<[u8; 32]>) -> bool {
        let next = match self.active_scope {
            SelectIrRankingScope::Global => SelectIrRankingScope::SelfAndRivals,
            SelectIrRankingScope::SelfAndRivals => SelectIrRankingScope::Global,
        };
        self.select_scope(ir_config, selected, next)
    }

    /// コース行は global のみ。単曲の Self-and-Rivals は取得済み時だけ有効にする。
    pub fn supports_scope(
        &self,
        ir_config: &IrConfig,
        selected: Option<[u8; 32]>,
        scope: SelectIrRankingScope,
    ) -> bool {
        enabled_provider(ir_config).is_some()
            && match scope {
                SelectIrRankingScope::Global => selected.is_some(),
                SelectIrRankingScope::SelfAndRivals => selected
                    .and_then(|sha256| self.cache.get(&sha256))
                    .and_then(|entry| entry.self_and_rivals.as_ref())
                    .is_some(),
            }
    }

    pub fn course_snapshot_for(
        &self,
        ir_config: &IrConfig,
        selected: Option<&SelectCourseIrTarget>,
    ) -> ResultIrSnapshot {
        if enabled_provider(ir_config).is_none() {
            return ResultIrSnapshot::default();
        }
        let Some(target) = selected else {
            return snapshot_with_provider(ResultIrSnapshot::default(), ir_config);
        };
        let mut snapshot = self
            .course_cache
            .get(target)
            .map(|entry| {
                let mut snapshot = entry.ir.clone();
                match snapshot.state {
                    SkinIrState::Loaded => {
                        snapshot.connect_success_ms = Some(elapsed_since_ms(entry.completed_at));
                    }
                    SkinIrState::Failed => {
                        snapshot.connect_fail_ms = Some(elapsed_since_ms(entry.completed_at));
                    }
                    _ => {}
                }
                snapshot
            })
            .unwrap_or_else(|| {
                let begin_ms = self
                    .course_in_flight
                    .as_ref()
                    .filter(|(_, current, _)| current == target)
                    .map(|(_, _, started_at)| elapsed_since_ms(*started_at));
                ResultIrSnapshot {
                    state: if begin_ms.is_some() {
                        SkinIrState::Loading
                    } else {
                        SkinIrState::Waiting
                    },
                    connect_begin_ms: begin_ms,
                    ..Default::default()
                }
            });
        snapshot.scope = SkinIrScope::Global;
        snapshot.global_scope_supported = true;
        snapshot.rival_scope_supported = false;
        snapshot_with_provider(snapshot, ir_config)
    }

    /// 選択中譜面のライバルベスト (最上位 1 名)。未取得 / IR 未設定なら None。
    pub fn rival_for(
        &self,
        ir_config: &IrConfig,
        selected: Option<[u8; 32]>,
    ) -> Option<SelectRivalSnapshot> {
        enabled_provider(ir_config)?;
        if self.active_rival.is_some() {
            let sha256 = selected?;
            let score = self
                .active_rival_scores
                .iter()
                .find_map(|((hash, _), score)| (*hash == sha256).then_some(score))?;
            return Some(SelectRivalSnapshot {
                display_name: self.active_rival.as_ref()?.display_name.clone(),
                ex_score: score.ex_score,
                clear_index: rival_clear_index(score.clear_type),
                max_combo: score.max_combo,
                bp: score.min_bp.max(0) as u32,
                judge_counts: None,
            });
        }
        self.cache.get(&selected?).and_then(|entry| entry.rival.clone())
    }

    pub fn active_rival_name(&self) -> Option<&str> {
        self.active_rival.as_ref().map(|rival| rival.display_name.as_str())
    }

    pub fn active_rival_score(
        &self,
        chart_sha256: [u8; 32],
        ln_mode: u8,
    ) -> Option<&IrRivalScoreRecord> {
        self.active_rival_scores.get(&(chart_sha256, ln_mode))
    }

    pub fn active_rival_snapshot(
        &self,
        chart_sha256: [u8; 32],
        ln_mode: u8,
    ) -> Option<SelectRivalSnapshot> {
        let score = self.active_rival_score(chart_sha256, ln_mode)?;
        Some(SelectRivalSnapshot {
            display_name: self.active_rival.as_ref()?.display_name.clone(),
            ex_score: score.ex_score,
            clear_index: rival_clear_index(score.clear_type),
            max_combo: score.max_combo,
            bp: score.min_bp.max(0) as u32,
            judge_counts: None,
        })
    }

    pub fn active_rival_display_name(&self) -> Option<&str> {
        self.active_rival_name()
    }

    pub fn resolved_target_for(
        &self,
        ir_config: &IrConfig,
        selected: Option<[u8; 32]>,
        target: TargetOption,
        local_best_ex_score: Option<u32>,
    ) -> Option<ResolvedTarget> {
        enabled_provider(ir_config)?;
        let entry = self.cache.get(&selected?)?;
        match target {
            TargetOption::IrTop => entry.global_targets.first().cloned(),
            TargetOption::IrNext => {
                next_target_above(&entry.global_targets, local_best_ex_score.unwrap_or(0))
            }
            TargetOption::RivalTop => entry.rival_targets.first().cloned(),
            TargetOption::RivalNext => {
                next_target_above(&entry.rival_targets, local_best_ex_score.unwrap_or(0))
            }
            TargetOption::RivalIndex(index) => {
                entry.rival_targets.get(index.saturating_sub(1) as usize).cloned()
            }
            _ => None,
        }
    }

    #[cfg(test)]
    pub(super) fn target_ex_score_for(
        &self,
        ir_config: &IrConfig,
        selected: Option<[u8; 32]>,
        target: TargetOption,
        local_best_ex_score: Option<u32>,
    ) -> Option<u32> {
        self.resolved_target_for(ir_config, selected, target, local_best_ex_score)
            .map(|target| target.ex_score)
    }

    /// ログイン状態が変わったとき等にキャッシュを破棄する。
    pub fn clear(&mut self) {
        self.cache.clear();
        self.in_flight = None;
        self.pending = None;
        self.course_cache.clear();
        self.course_in_flight = None;
        self.course_pending = None;
        self.active_scope = SelectIrRankingScope::Global;
    }

    pub(super) fn insert_entry(&mut self, sha256: [u8; 32], entry: CachedChartIr) {
        if self.cache.len() >= CACHE_CAPACITY && !self.cache.contains_key(&sha256) {
            self.cache.clear();
        }
        self.cache.insert(sha256, entry);
        if self.in_flight.as_ref().is_some_and(|(_, in_flight_sha, _)| *in_flight_sha == sha256) {
            self.in_flight = None;
        }
        if self.pending.as_ref().is_some_and(|(pending_sha, _)| *pending_sha == sha256) {
            self.pending = None;
        }
    }
}

pub(super) fn rival_clear_index(clear_type: i32) -> i64 {
    i64::from(clear_type).clamp(0, bmz_core::clear::ClearType::Max as i64)
}

fn snapshot_with_provider(
    mut snapshot: ResultIrSnapshot,
    ir_config: &IrConfig,
) -> ResultIrSnapshot {
    let Some(provider) = crate::ir::provider_key::primary_provider_config(ir_config) else {
        return snapshot;
    };
    snapshot.online = true;
    if let Some(name) = crate::ir::provider_key::configured_provider_display_name(provider) {
        snapshot.provider_name = bmz_render::scene::ResultIrRankingName::from_display_name(name);
    }
    snapshot.user_name =
        bmz_render::scene::ResultIrRankingName::from_display_name(&provider.account_display_name);
    snapshot
}
