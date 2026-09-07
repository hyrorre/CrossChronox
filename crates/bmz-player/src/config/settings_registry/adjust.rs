use super::*;

/// 設定値を 1 ステップ変更する。変更があった場合 `true`。
pub fn adjust_settings_value(profile: &mut ProfileConfig, id: SettingsEntryId, delta: i32) -> bool {
    if delta == 0 {
        return false;
    }
    match id {
        SettingsEntryId::NormalizeChartVolume => {
            profile.audio_mix.normalize_chart_volume = !profile.audio_mix.normalize_chart_volume;
            true
        }
        SettingsEntryId::NormalizeSystemBgmVolume => {
            profile.audio_mix.normalize_system_bgm_volume =
                !profile.audio_mix.normalize_system_bgm_volume;
            true
        }
        SettingsEntryId::MasterVolume => {
            adjust_u32(&mut profile.audio_mix.master_volume, delta, 0, 100)
        }
        SettingsEntryId::KeyVolume => adjust_u32(&mut profile.audio_mix.key_volume, delta, 0, 100),
        SettingsEntryId::BgmVolume => adjust_u32(&mut profile.audio_mix.bgm_volume, delta, 0, 100),
        SettingsEntryId::PreviewVolume => {
            adjust_u32(&mut profile.audio_mix.preview_volume, delta, 0, 100)
        }
        SettingsEntryId::SystemBgmVolume => {
            adjust_u32(&mut profile.audio_mix.system_bgm_volume, delta, 0, 100)
        }
        SettingsEntryId::SystemSeVolume => {
            adjust_u32(&mut profile.audio_mix.system_se_volume, delta, 0, 100)
        }
        SettingsEntryId::InputOffsetMs => {
            adjust_offset_ms(&mut profile.judge.input_offset_us, delta)
        }
        SettingsEntryId::VisualOffsetMs => {
            adjust_offset_ms(&mut profile.judge.visual_offset_us, delta)
        }
        SettingsEntryId::VisualOffsetAutoAdjust => {
            profile.judge.visual_offset_auto_adjust = !profile.judge.visual_offset_auto_adjust;
            true
        }
        SettingsEntryId::JudgeAlgorithm => {
            cycle_enum(delta, profile.judge.judge_algorithm, cycle_judge_algorithm)
                .map(|next| profile.judge.judge_algorithm = next)
                .is_some()
        }
        SettingsEntryId::FastSlowDisplayScope => {
            cycle_enum(delta, profile.judge.fast_slow_display_scope, cycle_fast_slow_display_scope)
                .map(|next| profile.judge.fast_slow_display_scope = next)
                .is_some()
        }
        SettingsEntryId::FastSlowDisplayThresholdMs => {
            adjust_u32(&mut profile.judge.fast_slow_display_threshold_ms, delta, 0, 50)
        }
        SettingsEntryId::RuleMode => cycle_enum(delta, profile.play.rule_mode, cycle_rule_mode)
            .map(|next| profile.play.rule_mode = next)
            .is_some(),
        SettingsEntryId::LnModePolicy => {
            cycle_enum(delta, profile.play.ln_mode_policy, cycle_ln_mode_policy)
                .map(|next| profile.play.ln_mode_policy = next)
                .is_some()
        }
        SettingsEntryId::Gauge => cycle_enum(delta, profile.play.gauge, cycle_gauge)
            .map(|next| profile.play.gauge = next)
            .is_some(),
        SettingsEntryId::GaugeAutoShift => {
            cycle_enum(delta, profile.play.gauge_auto_shift, cycle_gauge_auto_shift)
                .map(|next| profile.play.gauge_auto_shift = next)
                .is_some()
        }
        SettingsEntryId::BottomShiftableGauge => {
            cycle_enum(delta, profile.play.bottom_shiftable_gauge, cycle_bottom_shiftable_gauge)
                .map(|next| profile.play.bottom_shiftable_gauge = next)
                .is_some()
        }
        SettingsEntryId::Random => cycle_enum(delta, profile.play.random, cycle_random)
            .map(|next| profile.play.random = next)
            .is_some(),
        SettingsEntryId::Random2 => cycle_enum(delta, profile.play.random2, cycle_random)
            .map(|next| profile.play.random2 = next)
            .is_some(),
        SettingsEntryId::DoubleOption => {
            cycle_enum(delta, profile.play.double_option, cycle_double_option)
                .map(|next| profile.play.double_option = next)
                .is_some()
        }
        SettingsEntryId::HsFix => cycle_enum(delta, profile.play.hs_fix, cycle_hs_fix)
            .map(|next| profile.play.hs_fix = next)
            .is_some(),
        SettingsEntryId::Target => cycle_enum(delta, profile.play.target, cycle_target)
            .map(|next| profile.play.target = next)
            .is_some(),
        SettingsEntryId::Assist => cycle_enum(delta, profile.play.assist, cycle_assist)
            .map(|next| profile.play.assist = next)
            .is_some(),
        SettingsEntryId::BgaMode => cycle_enum(delta, profile.play.bga, cycle_bga_mode)
            .map(|next| profile.play.bga = next)
            .is_some(),
        SettingsEntryId::BgaExpand => cycle_enum(delta, profile.play.bga_expand, cycle_bga_expand)
            .map(|next| profile.play.bga_expand = next)
            .is_some(),
        SettingsEntryId::SessionMode => {
            let current = profile.play.session_mode.unwrap_or(if profile.play.auto_play {
                SessionMode::Autoplay
            } else {
                SessionMode::Normal
            });
            cycle_enum(delta, current, cycle_session_mode)
                .map(|next| {
                    profile.play.session_mode = Some(next);
                    profile.play.auto_play = next.primary_autoplay();
                })
                .is_some()
        }
        SettingsEntryId::KeyModeConversion => {
            cycle_enum(delta, profile.play.key_mode_conversion, cycle_key_mode_conversion)
                .map(|next| {
                    profile.play.key_mode_conversion = next;
                    if next != KeyModeConversionConfig::Off {
                        profile.play.double_option = DoubleOptionConfig::Off;
                    }
                })
                .is_some()
        }
        SettingsEntryId::SevenToNinePattern => {
            cycle_enum(delta, profile.play.seven_to_nine_pattern, cycle_seven_to_nine_pattern)
                .map(|next| profile.play.seven_to_nine_pattern = next)
                .is_some()
        }
        SettingsEntryId::SevenToNineType => {
            cycle_enum(delta, profile.play.seven_to_nine_type, cycle_seven_to_nine_type)
                .map(|next| profile.play.seven_to_nine_type = next)
                .is_some()
        }
        SettingsEntryId::SevenToNineRuleMode => {
            cycle_enum(delta, profile.play.seven_to_nine_rule_mode, cycle_seven_to_nine_rule_mode)
                .map(|next| profile.play.seven_to_nine_rule_mode = next)
                .is_some()
        }
        SettingsEntryId::PlayExitHoldMs => {
            adjust_u32(&mut profile.play.play_exit_hold_ms, delta, 100, 5000)
        }
        SettingsEntryId::AssistExpandJudge => {
            profile.play.assist.expand_judge = !profile.play.assist.expand_judge;
            true
        }
        SettingsEntryId::AssistJudgeArea => {
            profile.play.assist.judge_area = !profile.play.assist.judge_area;
            true
        }
        SettingsEntryId::AssistMarkNote => {
            profile.play.assist.mark_note = !profile.play.assist.mark_note;
            true
        }
        SettingsEntryId::AssistBpmGuide => {
            profile.play.assist.bpm_guide = !profile.play.assist.bpm_guide;
            true
        }
        SettingsEntryId::AssistScrollMode => {
            cycle_enum(delta, profile.play.assist.scroll_mode, cycle_assist_scroll_mode)
                .map(|next| profile.play.assist.scroll_mode = next)
                .is_some()
        }
        SettingsEntryId::AssistLongNoteMode => {
            cycle_enum(delta, profile.play.assist.long_note_mode, cycle_assist_long_note_mode)
                .map(|next| profile.play.assist.long_note_mode = next)
                .is_some()
        }
        SettingsEntryId::AssistMineMode => {
            cycle_enum(delta, profile.play.assist.mine_mode, cycle_assist_mine_mode)
                .map(|next| profile.play.assist.mine_mode = next)
                .is_some()
        }
        SettingsEntryId::AssistScrollSection => {
            adjust_u16(&mut profile.play.assist.scroll_section, delta, 1, 64)
        }
        SettingsEntryId::AssistScrollRate => {
            adjust_f64_percent(&mut profile.play.assist.scroll_rate, delta, 0.0, 1.0)
        }
        SettingsEntryId::AssistLongNoteRate => {
            adjust_f64_percent(&mut profile.play.assist.long_note_rate, delta, 0.0, 1.0)
        }
        SettingsEntryId::AssistExtraNoteDepth => {
            adjust_u8(&mut profile.play.assist.extra_note_depth, delta, 0, 16)
        }
        SettingsEntryId::AssistExtraNoteScratch => {
            profile.play.assist.extra_note_scratch = !profile.play.assist.extra_note_scratch;
            true
        }
        SettingsEntryId::AssistExtraNoteType => {
            adjust_u8(&mut profile.play.assist.extra_note_type, delta, 0, 2)
        }
        SettingsEntryId::AssistKeyPgreatRate => {
            adjust_u16(&mut profile.play.assist.key_pgreat_rate, delta, 0, 400)
        }
        SettingsEntryId::AssistKeyGreatRate => {
            adjust_u16(&mut profile.play.assist.key_great_rate, delta, 0, 400)
        }
        SettingsEntryId::AssistKeyGoodRate => {
            adjust_u16(&mut profile.play.assist.key_good_rate, delta, 0, 400)
        }
        SettingsEntryId::AssistScratchPgreatRate => {
            adjust_u16(&mut profile.play.assist.scratch_pgreat_rate, delta, 0, 400)
        }
        SettingsEntryId::AssistScratchGreatRate => {
            adjust_u16(&mut profile.play.assist.scratch_great_rate, delta, 0, 400)
        }
        SettingsEntryId::AssistScratchGoodRate => {
            adjust_u16(&mut profile.play.assist.scratch_good_rate, delta, 0, 400)
        }
        SettingsEntryId::AssistLongNoteMarginRate => {
            adjust_u16(&mut profile.play.assist.long_note_margin_rate, delta, 0, 400)
        }
        SettingsEntryId::MisslayerDurationMs => {
            adjust_u32(&mut profile.play.misslayer_duration_ms, delta, 0, 5000)
        }
        SettingsEntryId::NoteRetention => {
            if delta == 0 {
                false
            } else {
                profile.play.note_retention = !profile.play.note_retention;
                true
            }
        }
        SettingsEntryId::ShowLnTailCap => {
            if delta == 0 {
                false
            } else {
                profile.play.show_ln_tail_cap = !profile.play.show_ln_tail_cap;
                true
            }
        }
        SettingsEntryId::GuideSe => {
            profile.play.guide_se = !profile.play.guide_se;
            true
        }
        SettingsEntryId::Hispeed => adjust_hispeed(
            &mut profile.lane.hispeed,
            delta,
            profile.lane.classic_hispeed_step,
            default_classic_hispeed_step(),
        ),
        SettingsEntryId::HispeedMode => {
            cycle_enum(delta, profile.lane.hispeed_config(), cycle_hispeed_mode)
                .map(|next| profile.lane.set_hispeed_config(next))
                .is_some()
        }
        SettingsEntryId::NormalHispeedLevel => {
            let before = profile.lane.normal_hispeed_level;
            profile.lane.normal_hispeed_level = (i32::from(before) + delta).clamp(
                i32::from(crate::config::play::NORMAL_HISPEED_LEVEL_MIN),
                i32::from(crate::config::play::NORMAL_HISPEED_LEVEL_MAX),
            ) as u8;
            profile.lane.normal_hispeed_level != before
        }
        SettingsEntryId::ClassicHispeedStep => {
            adjust_hispeed_step(&mut profile.lane.classic_hispeed_step, delta)
        }
        SettingsEntryId::FloatingHispeedStep => {
            adjust_hispeed_step(&mut profile.lane.floating_hispeed_step, delta)
        }
        SettingsEntryId::SuddenEnabled => {
            profile.play.lane_effect = profile
                .play
                .lane_effect
                .with_sudden_enabled(!profile.play.lane_effect.sudden_enabled());
            true
        }
        SettingsEntryId::Sudden => adjust_u32(
            &mut profile.lane.sudden,
            delta,
            0,
            crate::config::play::lane_unit_max_for_other(profile.lane.lift),
        ),
        SettingsEntryId::LiftEnabled => {
            profile.lane.lift_enabled = !profile.lane.lift_enabled;
            true
        }
        SettingsEntryId::Lift => adjust_u32(
            &mut profile.lane.lift,
            delta,
            0,
            crate::config::play::lane_unit_max_for_other(profile.lane.sudden),
        ),
        SettingsEntryId::HispeedAutoAdjust => {
            profile.lane.hispeed_auto_adjust = !profile.lane.hispeed_auto_adjust;
            true
        }
        SettingsEntryId::HiddenEnabled => {
            profile.play.lane_effect = profile
                .play
                .lane_effect
                .with_hidden_enabled(!profile.play.lane_effect.hidden_enabled());
            true
        }
        SettingsEntryId::Hidden => adjust_u32(&mut profile.lane.hidden, delta, 0, 1000),
        SettingsEntryId::TargetGreenNumber => adjust_u32(
            &mut profile.lane.target_green_number,
            delta,
            TARGET_GREEN_NUMBER_MIN,
            TARGET_GREEN_NUMBER_MAX,
        ),
        SettingsEntryId::NoteDisplayDurationMs => {
            let current = profile.lane.target_green_number;
            let next = crate::config::play::adjust_green_number_by_duration_ms(current, delta);
            profile.lane.target_green_number = next;
            next != current
        }
        SettingsEntryId::Constant => {
            profile.lane.constant_enabled = !profile.lane.constant_enabled;
            true
        }
        SettingsEntryId::ConstantFadeMs => {
            let before = profile.lane.constant_fade_ms;
            profile.lane.constant_fade_ms = profile
                .lane
                .constant_fade_ms
                .saturating_add(delta)
                .clamp(CONSTANT_FADE_MIN_MS, CONSTANT_FADE_MAX_MS);
            profile.lane.constant_fade_ms != before
        }
        SettingsEntryId::SelectInputMode => {
            cycle_enum(delta, profile.input.select_input_mode, cycle_select_input_mode)
                .map(|next| profile.input.select_input_mode = next)
                .is_some()
        }
        SettingsEntryId::AnalogScratch1P => {
            profile.input.gamepad1.analog_scratch = !profile.input.gamepad1.analog_scratch;
            true
        }
        SettingsEntryId::AnalogScratchSensitivity1P => adjust_f32_tenths(
            &mut profile.input.gamepad1.analog_scratch_sensitivity,
            delta,
            0.1,
            5.0,
        ),
        SettingsEntryId::AnalogScratchThreshold1P => {
            adjust_u32(&mut profile.input.gamepad1.analog_scratch_threshold, delta, 1, 1000)
        }
        SettingsEntryId::AnalogScratch2P => {
            profile.input.gamepad2.analog_scratch = !profile.input.gamepad2.analog_scratch;
            true
        }
        SettingsEntryId::AnalogScratchSensitivity2P => adjust_f32_tenths(
            &mut profile.input.gamepad2.analog_scratch_sensitivity,
            delta,
            0.1,
            5.0,
        ),
        SettingsEntryId::AnalogScratchThreshold2P => {
            adjust_u32(&mut profile.input.gamepad2.analog_scratch_threshold, delta, 1, 1000)
        }
        SettingsEntryId::AnalogTicksPerScroll => {
            adjust_u32(&mut profile.input.analog_ticks_per_scroll, delta, 1, 100)
        }
        SettingsEntryId::KeyboardReleaseBounceMs => adjust_u32(
            &mut profile.input.keyboard_release_bounce_ms,
            delta,
            0,
            RELEASE_BOUNCE_MS_MAX,
        ),
        SettingsEntryId::ControllerReleaseBounceMs => adjust_u32(
            &mut profile.input.controller_release_bounce_ms,
            delta,
            0,
            RELEASE_BOUNCE_MS_MAX,
        ),
        id @ (SettingsEntryId::Hispeed8Key1
        | SettingsEntryId::Hispeed8Key2
        | SettingsEntryId::Hispeed8Key3
        | SettingsEntryId::Hispeed8Key4
        | SettingsEntryId::Hispeed8Key5
        | SettingsEntryId::Hispeed8Key6
        | SettingsEntryId::Hispeed8Key7
        | SettingsEntryId::Hispeed8Key8) => {
            let lane = eight_key_hispeed_lane(id).expect("guarded 8K hispeed setting");
            let current = crate::config::play_input::hispeed_direction_for_lane(
                &profile.input,
                KeyMode::K8,
                crate::config::play::lane_from_config(lane),
            )
            .expect("8K key lane has a hispeed direction");
            let next = match current {
                HispeedDirectionConfig::Down => HispeedDirectionConfig::Up,
                HispeedDirectionConfig::Up => HispeedDirectionConfig::Down,
            };
            crate::config::play_input::set_eight_key_hispeed_direction(
                &mut profile.input,
                lane,
                next,
            )
        }
        SettingsEntryId::DifficultyTableLevelDisplay => {
            profile.select.difficulty_table_level_display =
                match profile.select.difficulty_table_level_display {
                    DifficultyTableLevelDisplay::Table => DifficultyTableLevelDisplay::Chart,
                    DifficultyTableLevelDisplay::Chart => DifficultyTableLevelDisplay::Table,
                };
            true
        }
        SettingsEntryId::SelectRandomSelect => {
            if delta == 0 {
                false
            } else {
                profile.select.random_select = !profile.select.random_select;
                true
            }
        }
        SettingsEntryId::SelectRandomNoPlay => {
            if delta == 0 {
                false
            } else {
                profile.select.random_select_no_play = !profile.select.random_select_no_play;
                true
            }
        }
        SettingsEntryId::SelectRandomFailed => {
            if delta == 0 {
                false
            } else {
                profile.select.random_select_failed = !profile.select.random_select_failed;
                true
            }
        }
        SettingsEntryId::SelectRandomNotEasy => {
            if delta == 0 {
                false
            } else {
                profile.select.random_select_not_easy = !profile.select.random_select_not_easy;
                true
            }
        }
        SettingsEntryId::SelectRandomNotClear => {
            if delta == 0 {
                false
            } else {
                profile.select.random_select_not_clear = !profile.select.random_select_not_clear;
                true
            }
        }
        SettingsEntryId::SelectRandomNotHard => {
            if delta == 0 {
                false
            } else {
                profile.select.random_select_not_hard = !profile.select.random_select_not_hard;
                true
            }
        }
        SettingsEntryId::SelectRandomNotExHard => {
            if delta == 0 {
                false
            } else {
                profile.select.random_select_not_ex_hard =
                    !profile.select.random_select_not_ex_hard;
                true
            }
        }
        SettingsEntryId::SelectRandomNotFullCombo => {
            if delta == 0 {
                false
            } else {
                profile.select.random_select_not_full_combo =
                    !profile.select.random_select_not_full_combo;
                true
            }
        }
        SettingsEntryId::RandomMixTargetLevel => {
            adjust_u32(&mut profile.select.random_mix.target_level, delta, 0, 99)
        }
        SettingsEntryId::RandomMixMaxLevel => {
            adjust_u32(&mut profile.select.random_mix.max_level, delta, 0, 99)
        }
        SettingsEntryId::RandomMixMinLevel => {
            adjust_u32(&mut profile.select.random_mix.min_level, delta, 0, 99)
        }
        SettingsEntryId::RandomMixBpmRange => {
            adjust_u32(&mut profile.select.random_mix.bpm_range, delta, 0, 99)
        }
        SettingsEntryId::RandomMixMaxBpm => {
            adjust_u32(&mut profile.select.random_mix.max_bpm, delta, 0, 990)
        }
        SettingsEntryId::RandomMixMinBpm => {
            adjust_u32(&mut profile.select.random_mix.min_bpm, delta, 0, 990)
        }
        SettingsEntryId::RandomMixStages => {
            adjust_u32(&mut profile.select.random_mix.stages, delta, 0, 5)
        }
        SettingsEntryId::ReplayAutoSave => {
            if delta == 0 {
                false
            } else {
                profile.replay.auto_save = !profile.replay.auto_save;
                true
            }
        }
        SettingsEntryId::ReplayCompress => {
            profile.replay.compress = !profile.replay.compress;
            true
        }
        SettingsEntryId::ReplaySlot1Rule => {
            adjust_replay_slot_rule(&mut profile.replay.slot_rules[0], delta)
        }
        SettingsEntryId::ReplaySlot2Rule => {
            adjust_replay_slot_rule(&mut profile.replay.slot_rules[1], delta)
        }
        SettingsEntryId::ReplaySlot3Rule => {
            adjust_replay_slot_rule(&mut profile.replay.slot_rules[2], delta)
        }
        SettingsEntryId::ReplaySlot4Rule => {
            adjust_replay_slot_rule(&mut profile.replay.slot_rules[3], delta)
        }
        SettingsEntryId::Language => {
            let current = profile.ui.locale();
            cycle_enum(delta, current, cycle_language)
                .map(|next| profile.ui.set_locale(next))
                .is_some()
        }
        SettingsEntryId::ShowFps => {
            profile.ui.show_fps = !profile.ui.show_fps;
            true
        }
    }
}

pub const fn eight_key_hispeed_lane(id: SettingsEntryId) -> Option<LaneConfig> {
    match id {
        SettingsEntryId::Hispeed8Key1 => Some(LaneConfig::Key1),
        SettingsEntryId::Hispeed8Key2 => Some(LaneConfig::Key2),
        SettingsEntryId::Hispeed8Key3 => Some(LaneConfig::Key3),
        SettingsEntryId::Hispeed8Key4 => Some(LaneConfig::Key4),
        SettingsEntryId::Hispeed8Key5 => Some(LaneConfig::Key5),
        SettingsEntryId::Hispeed8Key6 => Some(LaneConfig::Key6),
        SettingsEntryId::Hispeed8Key7 => Some(LaneConfig::Key7),
        SettingsEntryId::Hispeed8Key8 => Some(LaneConfig::Key8),
        _ => None,
    }
}
