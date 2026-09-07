use super::*;

pub fn settings_adjust_step(id: SettingsEntryId) -> i32 {
    match id {
        SettingsEntryId::InputOffsetMs | SettingsEntryId::VisualOffsetMs => 1,
        SettingsEntryId::PlayExitHoldMs => 100,
        SettingsEntryId::Sudden | SettingsEntryId::Lift | SettingsEntryId::Hidden => 25,
        SettingsEntryId::TargetGreenNumber => 10,
        SettingsEntryId::NoteDisplayDurationMs => 10,
        SettingsEntryId::ConstantFadeMs => 10,
        SettingsEntryId::MisslayerDurationMs => 50,
        SettingsEntryId::AnalogScratchThreshold1P | SettingsEntryId::AnalogScratchThreshold2P => 10,
        SettingsEntryId::RandomMixMaxBpm | SettingsEntryId::RandomMixMinBpm => 10,
        SettingsEntryId::AssistScrollRate
        | SettingsEntryId::AssistLongNoteRate
        | SettingsEntryId::AssistKeyPgreatRate
        | SettingsEntryId::AssistKeyGreatRate
        | SettingsEntryId::AssistKeyGoodRate
        | SettingsEntryId::AssistScratchPgreatRate
        | SettingsEntryId::AssistScratchGreatRate
        | SettingsEntryId::AssistScratchGoodRate
        | SettingsEntryId::AssistLongNoteMarginRate => 5,
        _ => 1,
    }
}

pub fn format_settings_value(profile: &ProfileConfig, id: SettingsEntryId) -> String {
    match id {
        SettingsEntryId::NormalizeChartVolume => {
            format_bool_on_off(profile.audio_mix.normalize_chart_volume)
        }
        SettingsEntryId::NormalizeSystemBgmVolume => {
            format_bool_on_off(profile.audio_mix.normalize_system_bgm_volume)
        }
        SettingsEntryId::MasterVolume => format!("{}", profile.audio_mix.master_volume),
        SettingsEntryId::KeyVolume => format!("{}", profile.audio_mix.key_volume),
        SettingsEntryId::BgmVolume => format!("{}", profile.audio_mix.bgm_volume),
        SettingsEntryId::PreviewVolume => format!("{}", profile.audio_mix.preview_volume),
        SettingsEntryId::SystemBgmVolume => format!("{}", profile.audio_mix.system_bgm_volume),
        SettingsEntryId::SystemSeVolume => format!("{}", profile.audio_mix.system_se_volume),
        SettingsEntryId::InputOffsetMs => {
            format!("{} ms", profile.judge.input_offset_us / 1_000)
        }
        SettingsEntryId::VisualOffsetMs => {
            format!("{} ms", profile.judge.visual_offset_us / 1_000)
        }
        SettingsEntryId::VisualOffsetAutoAdjust => {
            format_bool_on_off(profile.judge.visual_offset_auto_adjust)
        }
        SettingsEntryId::JudgeAlgorithm => format_judge_algorithm(profile.judge.judge_algorithm),
        SettingsEntryId::FastSlowDisplayScope => match profile.judge.fast_slow_display_scope {
            FastSlowDisplayScope::Auto => "AUTO".to_string(),
            FastSlowDisplayScope::ThresholdMs => "THRESHOLD".to_string(),
        },
        SettingsEntryId::FastSlowDisplayThresholdMs => {
            format!("{} ms", profile.judge.fast_slow_display_threshold_ms)
        }
        SettingsEntryId::RuleMode => format_rule_mode(profile.play.rule_mode),
        SettingsEntryId::LnModePolicy => profile.play.ln_mode_policy.display_label().to_string(),
        SettingsEntryId::Gauge => format_gauge(profile.play.gauge),
        SettingsEntryId::GaugeAutoShift => format_gauge_auto_shift(profile.play.gauge_auto_shift),
        SettingsEntryId::BottomShiftableGauge => {
            format_bottom_shiftable_gauge(profile.play.bottom_shiftable_gauge)
        }
        SettingsEntryId::Random => format_random(profile.play.random),
        SettingsEntryId::Random2 => format_random(profile.play.random2),
        SettingsEntryId::DoubleOption => format_double_option(profile.play.double_option),
        SettingsEntryId::HsFix => format_hs_fix(profile.play.hs_fix),
        SettingsEntryId::Target => format_target(profile.play.target),
        SettingsEntryId::Assist => format_assist(profile.play.assist),
        SettingsEntryId::BgaMode => format_bga_mode(profile.play.bga),
        SettingsEntryId::BgaExpand => format_bga_expand(profile.play.bga_expand),
        SettingsEntryId::SessionMode => profile
            .play
            .session_mode
            .unwrap_or(if profile.play.auto_play {
                SessionMode::Autoplay
            } else {
                SessionMode::Normal
            })
            .as_str()
            .to_string(),
        SettingsEntryId::KeyModeConversion => profile.play.key_mode_conversion.as_str().to_string(),
        SettingsEntryId::SevenToNinePattern => {
            profile.play.seven_to_nine_pattern.label().to_string()
        }
        SettingsEntryId::SevenToNineType => profile.play.seven_to_nine_type.label().to_string(),
        SettingsEntryId::SevenToNineRuleMode => {
            profile.play.seven_to_nine_rule_mode.as_str().to_string()
        }
        SettingsEntryId::PlayExitHoldMs => format!("{} ms", profile.play.play_exit_hold_ms),
        SettingsEntryId::AssistExpandJudge => format_bool_on_off(profile.play.assist.expand_judge),
        SettingsEntryId::AssistJudgeArea => format_bool_on_off(profile.play.assist.judge_area),
        SettingsEntryId::AssistMarkNote => format_bool_on_off(profile.play.assist.mark_note),
        SettingsEntryId::AssistBpmGuide => format_bool_on_off(profile.play.assist.bpm_guide),
        SettingsEntryId::AssistScrollMode => match profile.play.assist.scroll_mode {
            AssistScrollMode::Off => "OFF",
            AssistScrollMode::Remove => "REMOVE (CONSTANT)",
            AssistScrollMode::Add => "ADD",
        }
        .to_string(),
        SettingsEntryId::AssistLongNoteMode => match profile.play.assist.long_note_mode {
            AssistLongNoteMode::Off => "OFF",
            AssistLongNoteMode::Remove => "REMOVE (LEGACY NOTE)",
            AssistLongNoteMode::AddLn => "ADD LN",
            AssistLongNoteMode::AddCn => "ADD CN",
            AssistLongNoteMode::AddHcn => "ADD HCN",
            AssistLongNoteMode::AddAll => "ADD ALL",
        }
        .to_string(),
        SettingsEntryId::AssistMineMode => match profile.play.assist.mine_mode {
            AssistMineMode::Off => "OFF",
            AssistMineMode::Remove => "REMOVE (NO MINE)",
            AssistMineMode::AddRandom => "ADD RANDOM",
            AssistMineMode::AddNear => "ADD NEAR",
            AssistMineMode::AddBlank => "ADD BLANK",
        }
        .to_string(),
        SettingsEntryId::AssistScrollSection => profile.play.assist.scroll_section.to_string(),
        SettingsEntryId::AssistScrollRate => {
            format!("{}%", (profile.play.assist.scroll_rate * 100.0).round() as i32)
        }
        SettingsEntryId::AssistLongNoteRate => {
            format!("{}%", (profile.play.assist.long_note_rate * 100.0).round() as i32)
        }
        SettingsEntryId::AssistExtraNoteDepth => profile.play.assist.extra_note_depth.to_string(),
        SettingsEntryId::AssistExtraNoteScratch => {
            format_bool_on_off(profile.play.assist.extra_note_scratch)
        }
        SettingsEntryId::AssistExtraNoteType => profile.play.assist.extra_note_type.to_string(),
        SettingsEntryId::AssistKeyPgreatRate => {
            format!("{}%", profile.play.assist.key_pgreat_rate)
        }
        SettingsEntryId::AssistKeyGreatRate => {
            format!("{}%", profile.play.assist.key_great_rate)
        }
        SettingsEntryId::AssistKeyGoodRate => {
            format!("{}%", profile.play.assist.key_good_rate)
        }
        SettingsEntryId::AssistScratchPgreatRate => {
            format!("{}%", profile.play.assist.scratch_pgreat_rate)
        }
        SettingsEntryId::AssistScratchGreatRate => {
            format!("{}%", profile.play.assist.scratch_great_rate)
        }
        SettingsEntryId::AssistScratchGoodRate => {
            format!("{}%", profile.play.assist.scratch_good_rate)
        }
        SettingsEntryId::AssistLongNoteMarginRate => {
            format!("{}%", profile.play.assist.long_note_margin_rate)
        }
        SettingsEntryId::NoteRetention => format_bool_on_off(profile.play.note_retention),
        SettingsEntryId::ShowLnTailCap => format_bool_on_off(profile.play.show_ln_tail_cap),
        SettingsEntryId::GuideSe => format_bool_on_off(profile.play.guide_se),
        SettingsEntryId::MisslayerDurationMs => {
            format!("{} ms", profile.play.misslayer_duration_ms)
        }
        SettingsEntryId::Hispeed => format!("{:.2}", profile.lane.hispeed),
        SettingsEntryId::HispeedMode => format_hispeed_mode(profile.lane.hispeed_config()),
        SettingsEntryId::NormalHispeedLevel => {
            format!("{}", profile.lane.normal_hispeed_level)
        }
        SettingsEntryId::ClassicHispeedStep => format!("{:.2}", profile.lane.classic_hispeed_step),
        SettingsEntryId::FloatingHispeedStep => {
            format!("{:.2}", profile.lane.floating_hispeed_step)
        }
        SettingsEntryId::SuddenEnabled => {
            format_bool_on_off(profile.play.lane_effect.sudden_enabled())
        }
        SettingsEntryId::Sudden => format_lane_unit(profile.lane.sudden),
        SettingsEntryId::LiftEnabled => format_bool_on_off(profile.lane.lift_enabled),
        SettingsEntryId::Lift => format_lane_unit(profile.lane.lift),
        SettingsEntryId::HispeedAutoAdjust => format_bool_on_off(profile.lane.hispeed_auto_adjust),
        SettingsEntryId::HiddenEnabled => {
            format_bool_on_off(profile.play.lane_effect.hidden_enabled())
        }
        SettingsEntryId::Hidden => format_lane_unit(profile.lane.hidden),
        SettingsEntryId::TargetGreenNumber => format!("{}", profile.lane.target_green_number),
        SettingsEntryId::NoteDisplayDurationMs => {
            format!(
                "{} ms",
                crate::config::play::duration_ms_from_green_number(
                    profile.lane.target_green_number.max(1),
                )
            )
        }
        SettingsEntryId::Constant => format_bool_on_off(profile.lane.constant_enabled),
        SettingsEntryId::ConstantFadeMs => format!("{} ms", profile.lane.constant_fade_ms),
        SettingsEntryId::SelectInputMode => {
            profile.input.select_input_mode.display_label().to_string()
        }
        SettingsEntryId::AnalogScratch1P => {
            format_bool_on_off(profile.input.gamepad1.analog_scratch)
        }
        SettingsEntryId::AnalogScratchSensitivity1P => {
            format!("{:.1}", profile.input.gamepad1.analog_scratch_sensitivity)
        }
        SettingsEntryId::AnalogScratchThreshold1P => {
            format!("{} ticks", profile.input.gamepad1.analog_scratch_threshold)
        }
        SettingsEntryId::AnalogScratch2P => {
            format_bool_on_off(profile.input.gamepad2.analog_scratch)
        }
        SettingsEntryId::AnalogScratchSensitivity2P => {
            format!("{:.1}", profile.input.gamepad2.analog_scratch_sensitivity)
        }
        SettingsEntryId::AnalogScratchThreshold2P => {
            format!("{} ticks", profile.input.gamepad2.analog_scratch_threshold)
        }
        SettingsEntryId::AnalogTicksPerScroll => {
            format!("{} ticks", profile.input.analog_ticks_per_scroll)
        }
        SettingsEntryId::KeyboardReleaseBounceMs => {
            format!("{} ms", profile.input.keyboard_release_bounce_ms)
        }
        SettingsEntryId::ControllerReleaseBounceMs => {
            format!("{} ms", profile.input.controller_release_bounce_ms)
        }
        id @ (SettingsEntryId::Hispeed8Key1
        | SettingsEntryId::Hispeed8Key2
        | SettingsEntryId::Hispeed8Key3
        | SettingsEntryId::Hispeed8Key4
        | SettingsEntryId::Hispeed8Key5
        | SettingsEntryId::Hispeed8Key6
        | SettingsEntryId::Hispeed8Key7
        | SettingsEntryId::Hispeed8Key8) => {
            let lane = eight_key_hispeed_lane(id).expect("guarded 8K hispeed setting");
            format_hispeed_direction(
                crate::config::play_input::hispeed_direction_for_lane(
                    &profile.input,
                    KeyMode::K8,
                    crate::config::play::lane_from_config(lane),
                )
                .expect("8K key lane has a hispeed direction"),
            )
        }
        SettingsEntryId::DifficultyTableLevelDisplay => {
            match profile.select.difficulty_table_level_display {
                DifficultyTableLevelDisplay::Table => "TABLE LEVEL",
                DifficultyTableLevelDisplay::Chart => "CHART LEVEL",
            }
            .to_string()
        }
        SettingsEntryId::SelectRandomSelect => format_bool_on_off(profile.select.random_select),
        SettingsEntryId::SelectRandomNoPlay => {
            format_bool_on_off(profile.select.random_select_no_play)
        }
        SettingsEntryId::SelectRandomFailed => {
            format_bool_on_off(profile.select.random_select_failed)
        }
        SettingsEntryId::SelectRandomNotEasy => {
            format_bool_on_off(profile.select.random_select_not_easy)
        }
        SettingsEntryId::SelectRandomNotClear => {
            format_bool_on_off(profile.select.random_select_not_clear)
        }
        SettingsEntryId::SelectRandomNotHard => {
            format_bool_on_off(profile.select.random_select_not_hard)
        }
        SettingsEntryId::SelectRandomNotExHard => {
            format_bool_on_off(profile.select.random_select_not_ex_hard)
        }
        SettingsEntryId::SelectRandomNotFullCombo => {
            format_bool_on_off(profile.select.random_select_not_full_combo)
        }
        SettingsEntryId::RandomMixTargetLevel => {
            format_random_mix_level(profile.select.random_mix.target_level, "OFF")
        }
        SettingsEntryId::RandomMixMaxLevel => {
            format_random_mix_level(profile.select.random_mix.max_level, "NO LIMIT")
        }
        SettingsEntryId::RandomMixMinLevel => {
            format_random_mix_level(profile.select.random_mix.min_level, "NO LIMIT")
        }
        SettingsEntryId::RandomMixBpmRange => {
            let value = profile.select.random_mix.bpm_range;
            if value == 0 { "NO LIMIT".to_string() } else { format!("+- {value} BPM") }
        }
        SettingsEntryId::RandomMixMaxBpm => {
            format_random_mix_bpm(profile.select.random_mix.max_bpm)
        }
        SettingsEntryId::RandomMixMinBpm => {
            format_random_mix_bpm(profile.select.random_mix.min_bpm)
        }
        SettingsEntryId::RandomMixStages => {
            let value = profile.select.random_mix.stages;
            if value == 0 { "RANDOM".to_string() } else { format!("{value} STAGE") }
        }
        SettingsEntryId::ReplayAutoSave => format_bool_on_off(profile.replay.auto_save),
        SettingsEntryId::ReplayCompress => format_bool_on_off(profile.replay.compress),
        SettingsEntryId::ReplaySlot1Rule => format_replay_slot_rule(profile.replay.slot_rules[0]),
        SettingsEntryId::ReplaySlot2Rule => format_replay_slot_rule(profile.replay.slot_rules[1]),
        SettingsEntryId::ReplaySlot3Rule => format_replay_slot_rule(profile.replay.slot_rules[2]),
        SettingsEntryId::ReplaySlot4Rule => format_replay_slot_rule(profile.replay.slot_rules[3]),
        SettingsEntryId::Language => profile.ui.locale().native_name().to_string(),
        SettingsEntryId::ShowFps => format_bool_on_off(profile.ui.show_fps),
    }
}

fn format_random_mix_level(value: u32, zero: &str) -> String {
    if value == 0 { zero.to_string() } else { format!("LEVEL {value}") }
}

fn format_random_mix_bpm(value: u32) -> String {
    if value == 0 { "NO LIMIT".to_string() } else { format!("{value} BPM") }
}
