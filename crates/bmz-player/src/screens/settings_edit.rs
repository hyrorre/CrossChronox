use std::collections::HashSet;

use bmz_core::lane::KeyMode;
use bmz_gameplay::rule::RuleMode;

use crate::config::app_config::{AppConfig, AudioConfig, VideoConfig};
use crate::config::app_settings_registry::{
    AppSettingsChoices, AppSettingsEntryId, adjust_app_settings_value,
};
use crate::config::play_input::resolve_play_bindings;
use crate::config::profile_config::{
    AssistOptionConfig, BgaExpandConfig, BgaModeConfig, BottomShiftableGaugeConfig,
    DifficultyTableLevelDisplay, DoubleOptionConfig, FastSlowDisplayScope, GaugeAutoShiftConfig,
    GaugeTypeConfig, HispeedConfigPreset, HispeedDirectionConfig, HsFixConfig, InputActionConfig,
    JudgeAlgorithmConfig, KeyModeConversionConfig, LaneConfig, LaneEffectConfig, ProfileConfig,
    ProfileInputConfig, RandomOptionConfig, ReplaySlotRule, ScratchDirectionConfig,
    SelectInputModeConfig, SevenToNinePattern, SevenToNineRuleMode, SevenToNineType,
    TargetOptionConfig,
};
use crate::config::settings_registry::{
    SettingsEntryId, adjust_settings_value, eight_key_hispeed_lane, format_settings_value,
};

use crate::ln_policy::LnPolicySetting;
use crate::select_options::SessionMode;

/// 7KEY / 14KEY + スクラッチ向けの設定画面入力マッピング。
#[derive(Debug, Clone)]
pub struct SettingsBindings {
    confirm: HashSet<String>,
    back: HashSet<String>,
    increase: HashSet<String>,
    decrease: HashSet<String>,
}

impl SettingsBindings {
    pub fn from_profile(input: &ProfileInputConfig) -> Self {
        let mut confirm = HashSet::new();
        let mut back = HashSet::new();
        let mut increase = HashSet::new();
        let mut decrease = HashSet::new();

        let mut play_controls = HashSet::new();
        match input.select_input_mode {
            SelectInputModeConfig::Key7Key14 => {
                collect_play_settings_bindings(
                    input,
                    KeyMode::K7,
                    &mut confirm,
                    &mut back,
                    &mut increase,
                    &mut decrease,
                );
                collect_play_settings_bindings(
                    input,
                    KeyMode::K14,
                    &mut confirm,
                    &mut back,
                    &mut increase,
                    &mut decrease,
                );
            }
            SelectInputModeConfig::Key9 => {
                collect_play_9k_settings_bindings(
                    input,
                    &mut confirm,
                    &mut back,
                    &mut increase,
                    &mut decrease,
                    &mut play_controls,
                );
            }
        }

        for entry in &input.ui.bindings {
            if input.select_input_mode == SelectInputModeConfig::Key9
                && play_controls.contains(&entry.control)
            {
                continue;
            }
            if entry.action == Some(InputActionConfig::E2) {
                back.insert(entry.control.clone());
            }
        }

        for key in ["Enter", "Space", "ArrowRight"] {
            confirm.insert(key.to_string());
        }
        for key in ["ArrowLeft", "Escape"] {
            back.insert(key.to_string());
        }
        for key in ["ArrowUp", "DPadDown", "ScratchDown"] {
            increase.insert(key.to_string());
        }
        for key in ["ArrowDown", "DPadUp", "ScratchUp"] {
            decrease.insert(key.to_string());
        }
        confirm.insert("Button1".to_string());
        back.insert("Select".to_string());

        Self { confirm, back, increase, decrease }
    }

    pub fn is_confirm(&self, control: &str) -> bool {
        self.confirm.contains(control)
    }

    pub fn is_back(&self, control: &str) -> bool {
        self.back.contains(control)
    }

    pub fn is_increase(&self, control: &str) -> bool {
        self.increase.contains(control)
    }

    pub fn is_decrease(&self, control: &str) -> bool {
        self.decrease.contains(control)
    }
}

fn collect_play_9k_settings_bindings(
    input: &ProfileInputConfig,
    confirm: &mut HashSet<String>,
    back: &mut HashSet<String>,
    increase: &mut HashSet<String>,
    decrease: &mut HashSet<String>,
    play_controls: &mut HashSet<String>,
) {
    let Ok(play) = resolve_play_bindings(input, KeyMode::K9) else {
        return;
    };
    for entry in play {
        play_controls.insert(entry.control.clone());
        let Some(lane) = entry.lane else { continue };
        match lane {
            LaneConfig::Key3 => {
                back.insert(entry.control.clone());
            }
            LaneConfig::Key4 => {
                decrease.insert(entry.control.clone());
            }
            LaneConfig::Key5 | LaneConfig::Key7 => {
                confirm.insert(entry.control.clone());
            }
            LaneConfig::Key6 => {
                increase.insert(entry.control.clone());
            }
            _ => {}
        }
    }
}

fn collect_play_settings_bindings(
    input: &ProfileInputConfig,
    key_mode: KeyMode,
    confirm: &mut HashSet<String>,
    back: &mut HashSet<String>,
    increase: &mut HashSet<String>,
    decrease: &mut HashSet<String>,
) {
    let Ok(play) = resolve_play_bindings(input, key_mode) else {
        return;
    };
    for entry in play {
        let Some(lane) = entry.lane else { continue };
        match lane {
            LaneConfig::Key1
            | LaneConfig::Key3
            | LaneConfig::Key5
            | LaneConfig::Key7
            | LaneConfig::Key8
            | LaneConfig::Key10
            | LaneConfig::Key12
            | LaneConfig::Key14 => {
                confirm.insert(entry.control.clone());
            }
            LaneConfig::Key2
            | LaneConfig::Key4
            | LaneConfig::Key6
            | LaneConfig::Key9
            | LaneConfig::Key11
            | LaneConfig::Key13 => {
                back.insert(entry.control.clone());
            }
            LaneConfig::Scratch | LaneConfig::Scratch2 => match entry.scratch {
                Some(ScratchDirectionConfig::Down) => {
                    increase.insert(entry.control.clone());
                }
                Some(ScratchDirectionConfig::Up) => {
                    decrease.insert(entry.control.clone());
                }
                None => {
                    classify_scratch_control(&entry.control, increase, decrease);
                }
            },
        }
    }
}

fn classify_scratch_control(
    control: &str,
    increase: &mut HashSet<String>,
    decrease: &mut HashSet<String>,
) {
    if control.contains("ScratchDown")
        || control.ends_with('+')
        || control == "Axis1+"
        || control == "Button8"
    {
        increase.insert(control.to_string());
        return;
    }
    if control.contains("ScratchUp")
        || control.ends_with('-')
        || control == "Axis1-"
        || control == "Button9"
    {
        decrease.insert(control.to_string());
        return;
    }
    increase.insert(control.to_string());
    decrease.insert(control.to_string());
}

#[derive(Debug, Clone)]
enum SettingsBaseline {
    Volume(u32),
    OffsetUs(i64),
    F32(f32),
    U32(u32),
    I32(i32),
    Bool(bool),
    JudgeAlgorithm(JudgeAlgorithmConfig),
    FastSlowDisplayScope(FastSlowDisplayScope),
    RuleMode(RuleMode),
    LnModePolicy(LnPolicySetting),
    Gauge(GaugeTypeConfig),
    GaugeAutoShift(GaugeAutoShiftConfig),
    BottomShiftableGauge(BottomShiftableGaugeConfig),
    Random(RandomOptionConfig),
    DoubleOption(DoubleOptionConfig),
    HsFix(HsFixConfig),
    Target(TargetOptionConfig),
    LaneEffect(LaneEffectConfig),
    Assist(AssistOptionConfig),
    BgaMode(BgaModeConfig),
    BgaExpand(BgaExpandConfig),
    SessionMode { value: Option<SessionMode>, auto_play: bool },
    KeyModeConversion { value: KeyModeConversionConfig, double_option: DoubleOptionConfig },
    SevenToNinePattern(SevenToNinePattern),
    SevenToNineType(SevenToNineType),
    SevenToNineRuleMode(SevenToNineRuleMode),
    HispeedMode(HispeedConfigPreset),
    HispeedDirection(HispeedDirectionConfig),
    SelectInputMode(SelectInputModeConfig),
    DifficultyTableLevelDisplay(DifficultyTableLevelDisplay),
    ReplaySlotRule(ReplaySlotRule),
    Language(String),
}

/// 編集開始時点の値。キャンセル時に profile へ戻す。
#[derive(Debug, Clone)]
pub struct SettingsEditSession {
    pub entry_id: SettingsEntryId,
    baseline: SettingsBaseline,
}

#[derive(Debug, Clone)]
enum AppSettingsBaseline {
    Audio(AudioConfig),
    Video(VideoConfig),
}

/// `data/config.toml` の音声・映像設定を一項目ずつ編集するセッション。
#[derive(Debug, Clone)]
pub struct AppSettingsEditSession {
    pub entry_id: AppSettingsEntryId,
    baseline: AppSettingsBaseline,
    choices: AppSettingsChoices,
}

impl AppSettingsEditSession {
    pub fn capture(
        config: &AppConfig,
        entry_id: AppSettingsEntryId,
        choices: AppSettingsChoices,
    ) -> Self {
        let baseline = if entry_id.is_audio() {
            AppSettingsBaseline::Audio(config.audio.clone())
        } else {
            AppSettingsBaseline::Video(config.video.clone())
        };
        Self { entry_id, baseline, choices }
    }

    pub fn restore(&self, config: &mut AppConfig) {
        match &self.baseline {
            AppSettingsBaseline::Audio(value) => config.audio = value.clone(),
            AppSettingsBaseline::Video(value) => config.video = value.clone(),
        }
    }

    pub fn adjust(&self, config: &mut AppConfig, direction: i32) -> bool {
        adjust_app_settings_value(config, self.entry_id, &self.choices, direction)
    }
}

/// 選曲設定で現在編集中の保存先を表す。
#[derive(Debug, Clone)]
pub enum SelectSettingsEditSession {
    Profile(SettingsEditSession),
    App(AppSettingsEditSession),
}

impl SettingsEditSession {
    pub fn capture(profile: &ProfileConfig, entry_id: SettingsEntryId) -> Self {
        let baseline = match entry_id {
            SettingsEntryId::NormalizeChartVolume => {
                SettingsBaseline::Bool(profile.audio_mix.normalize_chart_volume)
            }
            SettingsEntryId::NormalizeSystemBgmVolume => {
                SettingsBaseline::Bool(profile.audio_mix.normalize_system_bgm_volume)
            }
            SettingsEntryId::MasterVolume => {
                SettingsBaseline::Volume(profile.audio_mix.master_volume)
            }
            SettingsEntryId::KeyVolume => SettingsBaseline::Volume(profile.audio_mix.key_volume),
            SettingsEntryId::BgmVolume => SettingsBaseline::Volume(profile.audio_mix.bgm_volume),
            SettingsEntryId::PreviewVolume => {
                SettingsBaseline::Volume(profile.audio_mix.preview_volume)
            }
            SettingsEntryId::SystemBgmVolume => {
                SettingsBaseline::Volume(profile.audio_mix.system_bgm_volume)
            }
            SettingsEntryId::SystemSeVolume => {
                SettingsBaseline::Volume(profile.audio_mix.system_se_volume)
            }
            SettingsEntryId::InputOffsetMs => {
                SettingsBaseline::OffsetUs(profile.judge.input_offset_us)
            }
            SettingsEntryId::VisualOffsetMs => {
                SettingsBaseline::OffsetUs(profile.judge.visual_offset_us)
            }
            SettingsEntryId::VisualOffsetAutoAdjust => {
                SettingsBaseline::Bool(profile.judge.visual_offset_auto_adjust)
            }
            SettingsEntryId::JudgeAlgorithm => {
                SettingsBaseline::JudgeAlgorithm(profile.judge.judge_algorithm)
            }
            SettingsEntryId::FastSlowDisplayScope => {
                SettingsBaseline::FastSlowDisplayScope(profile.judge.fast_slow_display_scope)
            }
            SettingsEntryId::FastSlowDisplayThresholdMs => {
                SettingsBaseline::U32(profile.judge.fast_slow_display_threshold_ms)
            }
            SettingsEntryId::RuleMode => SettingsBaseline::RuleMode(profile.play.rule_mode),
            SettingsEntryId::LnModePolicy => {
                SettingsBaseline::LnModePolicy(profile.play.ln_mode_policy)
            }
            SettingsEntryId::Gauge => SettingsBaseline::Gauge(profile.play.gauge),
            SettingsEntryId::GaugeAutoShift => {
                SettingsBaseline::GaugeAutoShift(profile.play.gauge_auto_shift)
            }
            SettingsEntryId::BottomShiftableGauge => {
                SettingsBaseline::BottomShiftableGauge(profile.play.bottom_shiftable_gauge)
            }
            SettingsEntryId::Random => SettingsBaseline::Random(profile.play.random),
            SettingsEntryId::Random2 => SettingsBaseline::Random(profile.play.random2),
            SettingsEntryId::DoubleOption => {
                SettingsBaseline::DoubleOption(profile.play.double_option)
            }
            SettingsEntryId::HsFix => SettingsBaseline::HsFix(profile.play.hs_fix),
            SettingsEntryId::Target => SettingsBaseline::Target(profile.play.target),
            SettingsEntryId::Assist => SettingsBaseline::Assist(profile.play.assist),
            SettingsEntryId::BgaMode => SettingsBaseline::BgaMode(profile.play.bga),
            SettingsEntryId::BgaExpand => SettingsBaseline::BgaExpand(profile.play.bga_expand),
            SettingsEntryId::SessionMode => SettingsBaseline::SessionMode {
                value: profile.play.session_mode,
                auto_play: profile.play.auto_play,
            },
            SettingsEntryId::KeyModeConversion => SettingsBaseline::KeyModeConversion {
                value: profile.play.key_mode_conversion,
                double_option: profile.play.double_option,
            },
            SettingsEntryId::SevenToNinePattern => {
                SettingsBaseline::SevenToNinePattern(profile.play.seven_to_nine_pattern)
            }
            SettingsEntryId::SevenToNineType => {
                SettingsBaseline::SevenToNineType(profile.play.seven_to_nine_type)
            }
            SettingsEntryId::SevenToNineRuleMode => {
                SettingsBaseline::SevenToNineRuleMode(profile.play.seven_to_nine_rule_mode)
            }
            SettingsEntryId::PlayExitHoldMs => {
                SettingsBaseline::U32(profile.play.play_exit_hold_ms)
            }
            SettingsEntryId::AssistExpandJudge
            | SettingsEntryId::AssistJudgeArea
            | SettingsEntryId::AssistMarkNote
            | SettingsEntryId::AssistBpmGuide
            | SettingsEntryId::AssistScrollMode
            | SettingsEntryId::AssistLongNoteMode
            | SettingsEntryId::AssistMineMode
            | SettingsEntryId::AssistScrollSection
            | SettingsEntryId::AssistScrollRate
            | SettingsEntryId::AssistLongNoteRate
            | SettingsEntryId::AssistExtraNoteDepth
            | SettingsEntryId::AssistExtraNoteScratch
            | SettingsEntryId::AssistExtraNoteType
            | SettingsEntryId::AssistKeyPgreatRate
            | SettingsEntryId::AssistKeyGreatRate
            | SettingsEntryId::AssistKeyGoodRate
            | SettingsEntryId::AssistScratchPgreatRate
            | SettingsEntryId::AssistScratchGreatRate
            | SettingsEntryId::AssistScratchGoodRate
            | SettingsEntryId::AssistLongNoteMarginRate => {
                SettingsBaseline::Assist(profile.play.assist)
            }
            SettingsEntryId::MisslayerDurationMs => {
                SettingsBaseline::U32(profile.play.misslayer_duration_ms)
            }
            SettingsEntryId::NoteRetention => SettingsBaseline::Bool(profile.play.note_retention),
            SettingsEntryId::ShowLnTailCap => SettingsBaseline::Bool(profile.play.show_ln_tail_cap),
            SettingsEntryId::GuideSe => SettingsBaseline::Bool(profile.play.guide_se),
            SettingsEntryId::Hispeed => SettingsBaseline::F32(profile.lane.hispeed),
            SettingsEntryId::HispeedMode => {
                SettingsBaseline::HispeedMode(profile.lane.hispeed_config())
            }
            SettingsEntryId::NormalHispeedLevel => {
                SettingsBaseline::U32(u32::from(profile.lane.normal_hispeed_level))
            }
            SettingsEntryId::ClassicHispeedStep => {
                SettingsBaseline::F32(profile.lane.classic_hispeed_step)
            }
            SettingsEntryId::FloatingHispeedStep => {
                SettingsBaseline::F32(profile.lane.floating_hispeed_step)
            }
            SettingsEntryId::SuddenEnabled => {
                SettingsBaseline::LaneEffect(profile.play.lane_effect)
            }
            SettingsEntryId::Sudden => SettingsBaseline::U32(profile.lane.sudden),
            SettingsEntryId::LiftEnabled => SettingsBaseline::Bool(profile.lane.lift_enabled),
            SettingsEntryId::Lift => SettingsBaseline::U32(profile.lane.lift),
            SettingsEntryId::HispeedAutoAdjust => {
                SettingsBaseline::Bool(profile.lane.hispeed_auto_adjust)
            }
            SettingsEntryId::HiddenEnabled => {
                SettingsBaseline::LaneEffect(profile.play.lane_effect)
            }
            SettingsEntryId::Hidden => SettingsBaseline::U32(profile.lane.hidden),
            SettingsEntryId::TargetGreenNumber => {
                SettingsBaseline::U32(profile.lane.target_green_number)
            }
            SettingsEntryId::NoteDisplayDurationMs => {
                SettingsBaseline::U32(profile.lane.target_green_number)
            }
            SettingsEntryId::Constant => SettingsBaseline::Bool(profile.lane.constant_enabled),
            SettingsEntryId::ConstantFadeMs => SettingsBaseline::I32(profile.lane.constant_fade_ms),
            SettingsEntryId::SelectInputMode => {
                SettingsBaseline::SelectInputMode(profile.input.select_input_mode)
            }
            SettingsEntryId::DifficultyTableLevelDisplay => {
                SettingsBaseline::DifficultyTableLevelDisplay(
                    profile.select.difficulty_table_level_display,
                )
            }
            SettingsEntryId::SelectRandomNoPlay => {
                SettingsBaseline::Bool(profile.select.random_select_no_play)
            }
            SettingsEntryId::SelectRandomFailed => {
                SettingsBaseline::Bool(profile.select.random_select_failed)
            }
            SettingsEntryId::SelectRandomNotEasy => {
                SettingsBaseline::Bool(profile.select.random_select_not_easy)
            }
            SettingsEntryId::SelectRandomNotClear => {
                SettingsBaseline::Bool(profile.select.random_select_not_clear)
            }
            SettingsEntryId::SelectRandomNotHard => {
                SettingsBaseline::Bool(profile.select.random_select_not_hard)
            }
            SettingsEntryId::SelectRandomNotExHard => {
                SettingsBaseline::Bool(profile.select.random_select_not_ex_hard)
            }
            SettingsEntryId::SelectRandomNotFullCombo => {
                SettingsBaseline::Bool(profile.select.random_select_not_full_combo)
            }
            SettingsEntryId::SelectRandomSelect => {
                SettingsBaseline::Bool(profile.select.random_select)
            }
            SettingsEntryId::RandomMixTargetLevel => {
                SettingsBaseline::U32(profile.select.random_mix.target_level)
            }
            SettingsEntryId::RandomMixMaxLevel => {
                SettingsBaseline::U32(profile.select.random_mix.max_level)
            }
            SettingsEntryId::RandomMixMinLevel => {
                SettingsBaseline::U32(profile.select.random_mix.min_level)
            }
            SettingsEntryId::RandomMixBpmRange => {
                SettingsBaseline::U32(profile.select.random_mix.bpm_range)
            }
            SettingsEntryId::RandomMixMaxBpm => {
                SettingsBaseline::U32(profile.select.random_mix.max_bpm)
            }
            SettingsEntryId::RandomMixMinBpm => {
                SettingsBaseline::U32(profile.select.random_mix.min_bpm)
            }
            SettingsEntryId::RandomMixStages => {
                SettingsBaseline::U32(profile.select.random_mix.stages)
            }
            SettingsEntryId::AnalogScratch1P => {
                SettingsBaseline::Bool(profile.input.gamepad1.analog_scratch)
            }
            SettingsEntryId::AnalogScratchSensitivity1P => {
                SettingsBaseline::F32(profile.input.gamepad1.analog_scratch_sensitivity)
            }
            SettingsEntryId::AnalogScratchThreshold1P => {
                SettingsBaseline::U32(profile.input.gamepad1.analog_scratch_threshold)
            }
            SettingsEntryId::AnalogScratch2P => {
                SettingsBaseline::Bool(profile.input.gamepad2.analog_scratch)
            }
            SettingsEntryId::AnalogScratchSensitivity2P => {
                SettingsBaseline::F32(profile.input.gamepad2.analog_scratch_sensitivity)
            }
            SettingsEntryId::AnalogScratchThreshold2P => {
                SettingsBaseline::U32(profile.input.gamepad2.analog_scratch_threshold)
            }
            SettingsEntryId::AnalogTicksPerScroll => {
                SettingsBaseline::U32(profile.input.analog_ticks_per_scroll)
            }
            SettingsEntryId::KeyboardReleaseBounceMs => {
                SettingsBaseline::U32(profile.input.keyboard_release_bounce_ms)
            }
            SettingsEntryId::ControllerReleaseBounceMs => {
                SettingsBaseline::U32(profile.input.controller_release_bounce_ms)
            }
            id @ (SettingsEntryId::Hispeed8Key1
            | SettingsEntryId::Hispeed8Key2
            | SettingsEntryId::Hispeed8Key3
            | SettingsEntryId::Hispeed8Key4
            | SettingsEntryId::Hispeed8Key5
            | SettingsEntryId::Hispeed8Key6
            | SettingsEntryId::Hispeed8Key7
            | SettingsEntryId::Hispeed8Key8) => {
                let lane = eight_key_hispeed_lane(id).expect("8K hispeed setting has a lane");
                SettingsBaseline::HispeedDirection(
                    crate::config::play_input::hispeed_direction_for_lane(
                        &profile.input,
                        KeyMode::K8,
                        crate::config::play::lane_from_config(lane),
                    )
                    .expect("8K key lane has a hispeed direction"),
                )
            }
            SettingsEntryId::ReplayAutoSave => SettingsBaseline::Bool(profile.replay.auto_save),
            SettingsEntryId::ReplayCompress => SettingsBaseline::Bool(profile.replay.compress),
            SettingsEntryId::ReplaySlot1Rule => {
                SettingsBaseline::ReplaySlotRule(profile.replay.slot_rules[0])
            }
            SettingsEntryId::ReplaySlot2Rule => {
                SettingsBaseline::ReplaySlotRule(profile.replay.slot_rules[1])
            }
            SettingsEntryId::ReplaySlot3Rule => {
                SettingsBaseline::ReplaySlotRule(profile.replay.slot_rules[2])
            }
            SettingsEntryId::ReplaySlot4Rule => {
                SettingsBaseline::ReplaySlotRule(profile.replay.slot_rules[3])
            }
            SettingsEntryId::Language => SettingsBaseline::Language(profile.ui.language.clone()),
            SettingsEntryId::ShowFps => SettingsBaseline::Bool(profile.ui.show_fps),
        };
        Self { entry_id, baseline }
    }

    pub fn restore(&self, profile: &mut ProfileConfig) {
        match (&self.entry_id, &self.baseline) {
            (SettingsEntryId::NormalizeChartVolume, SettingsBaseline::Bool(value)) => {
                profile.audio_mix.normalize_chart_volume = *value;
            }
            (SettingsEntryId::NormalizeSystemBgmVolume, SettingsBaseline::Bool(value)) => {
                profile.audio_mix.normalize_system_bgm_volume = *value;
            }
            (SettingsEntryId::MasterVolume, SettingsBaseline::Volume(value)) => {
                profile.audio_mix.master_volume = *value;
            }
            (SettingsEntryId::KeyVolume, SettingsBaseline::Volume(value)) => {
                profile.audio_mix.key_volume = *value;
            }
            (SettingsEntryId::BgmVolume, SettingsBaseline::Volume(value)) => {
                profile.audio_mix.bgm_volume = *value;
            }
            (SettingsEntryId::PreviewVolume, SettingsBaseline::Volume(value)) => {
                profile.audio_mix.preview_volume = *value;
            }
            (SettingsEntryId::SystemBgmVolume, SettingsBaseline::Volume(value)) => {
                profile.audio_mix.system_bgm_volume = *value;
            }
            (SettingsEntryId::SystemSeVolume, SettingsBaseline::Volume(value)) => {
                profile.audio_mix.system_se_volume = *value;
            }
            (SettingsEntryId::InputOffsetMs, SettingsBaseline::OffsetUs(value)) => {
                profile.judge.input_offset_us = *value;
            }
            (SettingsEntryId::VisualOffsetMs, SettingsBaseline::OffsetUs(value)) => {
                profile.judge.visual_offset_us = *value;
            }
            (SettingsEntryId::VisualOffsetAutoAdjust, SettingsBaseline::Bool(value)) => {
                profile.judge.visual_offset_auto_adjust = *value;
            }
            (SettingsEntryId::JudgeAlgorithm, SettingsBaseline::JudgeAlgorithm(value)) => {
                profile.judge.judge_algorithm = *value;
            }
            (
                SettingsEntryId::FastSlowDisplayScope,
                SettingsBaseline::FastSlowDisplayScope(value),
            ) => {
                profile.judge.fast_slow_display_scope = *value;
            }
            (SettingsEntryId::FastSlowDisplayThresholdMs, SettingsBaseline::U32(value)) => {
                profile.judge.fast_slow_display_threshold_ms = *value;
            }
            (SettingsEntryId::RuleMode, SettingsBaseline::RuleMode(value)) => {
                profile.play.rule_mode = *value;
            }
            (SettingsEntryId::LnModePolicy, SettingsBaseline::LnModePolicy(value)) => {
                profile.play.ln_mode_policy = *value;
            }
            (SettingsEntryId::Gauge, SettingsBaseline::Gauge(value)) => {
                profile.play.gauge = *value;
            }
            (SettingsEntryId::GaugeAutoShift, SettingsBaseline::GaugeAutoShift(value)) => {
                profile.play.gauge_auto_shift = *value;
            }
            (
                SettingsEntryId::BottomShiftableGauge,
                SettingsBaseline::BottomShiftableGauge(value),
            ) => {
                profile.play.bottom_shiftable_gauge = *value;
            }
            (SettingsEntryId::Random, SettingsBaseline::Random(value)) => {
                profile.play.random = *value;
            }
            (SettingsEntryId::Random2, SettingsBaseline::Random(value)) => {
                profile.play.random2 = *value;
            }
            (SettingsEntryId::DoubleOption, SettingsBaseline::DoubleOption(value)) => {
                profile.play.double_option = *value;
            }
            (SettingsEntryId::HsFix, SettingsBaseline::HsFix(value)) => {
                profile.play.hs_fix = *value;
            }
            (SettingsEntryId::Target, SettingsBaseline::Target(value)) => {
                profile.play.target = *value;
            }
            (
                SettingsEntryId::SuddenEnabled | SettingsEntryId::HiddenEnabled,
                SettingsBaseline::LaneEffect(value),
            ) => {
                profile.play.lane_effect = *value;
            }
            (SettingsEntryId::Assist, SettingsBaseline::Assist(value)) => {
                profile.play.assist = *value;
            }
            (SettingsEntryId::BgaMode, SettingsBaseline::BgaMode(value)) => {
                profile.play.bga = *value;
            }
            (SettingsEntryId::BgaExpand, SettingsBaseline::BgaExpand(value)) => {
                profile.play.bga_expand = *value;
            }
            (SettingsEntryId::SessionMode, SettingsBaseline::SessionMode { value, auto_play }) => {
                profile.play.session_mode = *value;
                profile.play.auto_play = *auto_play;
            }
            (
                SettingsEntryId::KeyModeConversion,
                SettingsBaseline::KeyModeConversion { value, double_option },
            ) => {
                profile.play.key_mode_conversion = *value;
                profile.play.double_option = *double_option;
            }
            (SettingsEntryId::SevenToNinePattern, SettingsBaseline::SevenToNinePattern(value)) => {
                profile.play.seven_to_nine_pattern = *value;
            }
            (SettingsEntryId::SevenToNineType, SettingsBaseline::SevenToNineType(value)) => {
                profile.play.seven_to_nine_type = *value;
            }
            (
                SettingsEntryId::SevenToNineRuleMode,
                SettingsBaseline::SevenToNineRuleMode(value),
            ) => {
                profile.play.seven_to_nine_rule_mode = *value;
            }
            (SettingsEntryId::PlayExitHoldMs, SettingsBaseline::U32(value)) => {
                profile.play.play_exit_hold_ms = *value;
            }
            (
                SettingsEntryId::AssistExpandJudge
                | SettingsEntryId::AssistJudgeArea
                | SettingsEntryId::AssistMarkNote
                | SettingsEntryId::AssistBpmGuide
                | SettingsEntryId::AssistScrollMode
                | SettingsEntryId::AssistLongNoteMode
                | SettingsEntryId::AssistMineMode
                | SettingsEntryId::AssistScrollSection
                | SettingsEntryId::AssistScrollRate
                | SettingsEntryId::AssistLongNoteRate
                | SettingsEntryId::AssistExtraNoteDepth
                | SettingsEntryId::AssistExtraNoteScratch
                | SettingsEntryId::AssistExtraNoteType
                | SettingsEntryId::AssistKeyPgreatRate
                | SettingsEntryId::AssistKeyGreatRate
                | SettingsEntryId::AssistKeyGoodRate
                | SettingsEntryId::AssistScratchPgreatRate
                | SettingsEntryId::AssistScratchGreatRate
                | SettingsEntryId::AssistScratchGoodRate
                | SettingsEntryId::AssistLongNoteMarginRate,
                SettingsBaseline::Assist(value),
            ) => {
                profile.play.assist = *value;
            }
            (SettingsEntryId::MisslayerDurationMs, SettingsBaseline::U32(value)) => {
                profile.play.misslayer_duration_ms = *value;
            }
            (SettingsEntryId::NoteRetention, SettingsBaseline::Bool(value)) => {
                profile.play.note_retention = *value;
            }
            (SettingsEntryId::ShowLnTailCap, SettingsBaseline::Bool(value)) => {
                profile.play.show_ln_tail_cap = *value;
            }
            (SettingsEntryId::GuideSe, SettingsBaseline::Bool(value)) => {
                profile.play.guide_se = *value;
            }
            (SettingsEntryId::Hispeed, SettingsBaseline::F32(value)) => {
                profile.lane.hispeed = *value;
            }
            (SettingsEntryId::HispeedMode, SettingsBaseline::HispeedMode(value)) => {
                profile.lane.set_hispeed_config(*value);
            }
            (SettingsEntryId::NormalHispeedLevel, SettingsBaseline::U32(value)) => {
                profile.lane.normal_hispeed_level = *value as u8;
            }
            (SettingsEntryId::ClassicHispeedStep, SettingsBaseline::F32(value)) => {
                profile.lane.classic_hispeed_step = *value;
            }
            (SettingsEntryId::FloatingHispeedStep, SettingsBaseline::F32(value)) => {
                profile.lane.floating_hispeed_step = *value;
            }
            (SettingsEntryId::Sudden, SettingsBaseline::U32(value)) => {
                profile.lane.sudden = *value;
            }
            (SettingsEntryId::LiftEnabled, SettingsBaseline::Bool(value)) => {
                profile.lane.lift_enabled = *value;
            }
            (SettingsEntryId::Lift, SettingsBaseline::U32(value)) => {
                profile.lane.lift = *value;
            }
            (SettingsEntryId::HispeedAutoAdjust, SettingsBaseline::Bool(value)) => {
                profile.lane.hispeed_auto_adjust = *value;
            }
            (SettingsEntryId::Hidden, SettingsBaseline::U32(value)) => {
                profile.lane.hidden = *value;
            }
            (SettingsEntryId::TargetGreenNumber, SettingsBaseline::U32(value)) => {
                profile.lane.target_green_number = *value;
            }
            (SettingsEntryId::NoteDisplayDurationMs, SettingsBaseline::U32(value)) => {
                profile.lane.target_green_number = *value;
            }
            (SettingsEntryId::Constant, SettingsBaseline::Bool(value)) => {
                profile.lane.constant_enabled = *value;
            }
            (SettingsEntryId::ConstantFadeMs, SettingsBaseline::I32(value)) => {
                profile.lane.constant_fade_ms = *value;
            }
            (SettingsEntryId::SelectInputMode, SettingsBaseline::SelectInputMode(value)) => {
                profile.input.select_input_mode = *value;
            }
            (
                SettingsEntryId::DifficultyTableLevelDisplay,
                SettingsBaseline::DifficultyTableLevelDisplay(value),
            ) => {
                profile.select.difficulty_table_level_display = *value;
            }
            (SettingsEntryId::SelectRandomNoPlay, SettingsBaseline::Bool(value)) => {
                profile.select.random_select_no_play = *value;
            }
            (SettingsEntryId::SelectRandomFailed, SettingsBaseline::Bool(value)) => {
                profile.select.random_select_failed = *value;
            }
            (SettingsEntryId::SelectRandomNotEasy, SettingsBaseline::Bool(value)) => {
                profile.select.random_select_not_easy = *value;
            }
            (SettingsEntryId::SelectRandomNotClear, SettingsBaseline::Bool(value)) => {
                profile.select.random_select_not_clear = *value;
            }
            (SettingsEntryId::SelectRandomNotHard, SettingsBaseline::Bool(value)) => {
                profile.select.random_select_not_hard = *value;
            }
            (SettingsEntryId::SelectRandomNotExHard, SettingsBaseline::Bool(value)) => {
                profile.select.random_select_not_ex_hard = *value;
            }
            (SettingsEntryId::SelectRandomNotFullCombo, SettingsBaseline::Bool(value)) => {
                profile.select.random_select_not_full_combo = *value;
            }
            (SettingsEntryId::SelectRandomSelect, SettingsBaseline::Bool(value)) => {
                profile.select.random_select = *value;
            }
            (SettingsEntryId::RandomMixTargetLevel, SettingsBaseline::U32(value)) => {
                profile.select.random_mix.target_level = *value;
            }
            (SettingsEntryId::RandomMixMaxLevel, SettingsBaseline::U32(value)) => {
                profile.select.random_mix.max_level = *value;
            }
            (SettingsEntryId::RandomMixMinLevel, SettingsBaseline::U32(value)) => {
                profile.select.random_mix.min_level = *value;
            }
            (SettingsEntryId::RandomMixBpmRange, SettingsBaseline::U32(value)) => {
                profile.select.random_mix.bpm_range = *value;
            }
            (SettingsEntryId::RandomMixMaxBpm, SettingsBaseline::U32(value)) => {
                profile.select.random_mix.max_bpm = *value;
            }
            (SettingsEntryId::RandomMixMinBpm, SettingsBaseline::U32(value)) => {
                profile.select.random_mix.min_bpm = *value;
            }
            (SettingsEntryId::RandomMixStages, SettingsBaseline::U32(value)) => {
                profile.select.random_mix.stages = *value;
            }
            (SettingsEntryId::AnalogScratch1P, SettingsBaseline::Bool(value)) => {
                profile.input.gamepad1.analog_scratch = *value;
            }
            (SettingsEntryId::AnalogScratchSensitivity1P, SettingsBaseline::F32(value)) => {
                profile.input.gamepad1.analog_scratch_sensitivity = *value;
            }
            (SettingsEntryId::AnalogScratchThreshold1P, SettingsBaseline::U32(value)) => {
                profile.input.gamepad1.analog_scratch_threshold = *value;
            }
            (SettingsEntryId::AnalogScratch2P, SettingsBaseline::Bool(value)) => {
                profile.input.gamepad2.analog_scratch = *value;
            }
            (SettingsEntryId::AnalogScratchSensitivity2P, SettingsBaseline::F32(value)) => {
                profile.input.gamepad2.analog_scratch_sensitivity = *value;
            }
            (SettingsEntryId::AnalogScratchThreshold2P, SettingsBaseline::U32(value)) => {
                profile.input.gamepad2.analog_scratch_threshold = *value;
            }
            (SettingsEntryId::AnalogTicksPerScroll, SettingsBaseline::U32(value)) => {
                profile.input.analog_ticks_per_scroll = *value;
            }
            (SettingsEntryId::KeyboardReleaseBounceMs, SettingsBaseline::U32(value)) => {
                profile.input.keyboard_release_bounce_ms = *value;
            }
            (SettingsEntryId::ControllerReleaseBounceMs, SettingsBaseline::U32(value)) => {
                profile.input.controller_release_bounce_ms = *value;
            }
            (
                id @ (SettingsEntryId::Hispeed8Key1
                | SettingsEntryId::Hispeed8Key2
                | SettingsEntryId::Hispeed8Key3
                | SettingsEntryId::Hispeed8Key4
                | SettingsEntryId::Hispeed8Key5
                | SettingsEntryId::Hispeed8Key6
                | SettingsEntryId::Hispeed8Key7
                | SettingsEntryId::Hispeed8Key8),
                SettingsBaseline::HispeedDirection(value),
            ) => {
                let lane = eight_key_hispeed_lane(*id).expect("8K hispeed setting has a lane");
                crate::config::play_input::set_eight_key_hispeed_direction(
                    &mut profile.input,
                    lane,
                    *value,
                );
            }
            (SettingsEntryId::ReplayAutoSave, SettingsBaseline::Bool(value)) => {
                profile.replay.auto_save = *value;
            }
            (SettingsEntryId::ReplayCompress, SettingsBaseline::Bool(value)) => {
                profile.replay.compress = *value;
            }
            (SettingsEntryId::ReplaySlot1Rule, SettingsBaseline::ReplaySlotRule(value)) => {
                profile.replay.slot_rules[0] = *value;
            }
            (SettingsEntryId::ReplaySlot2Rule, SettingsBaseline::ReplaySlotRule(value)) => {
                profile.replay.slot_rules[1] = *value;
            }
            (SettingsEntryId::ReplaySlot3Rule, SettingsBaseline::ReplaySlotRule(value)) => {
                profile.replay.slot_rules[2] = *value;
            }
            (SettingsEntryId::ReplaySlot4Rule, SettingsBaseline::ReplaySlotRule(value)) => {
                profile.replay.slot_rules[3] = *value;
            }
            (SettingsEntryId::Language, SettingsBaseline::Language(value)) => {
                profile.ui.language.clone_from(value);
            }
            (SettingsEntryId::ShowFps, SettingsBaseline::Bool(value)) => {
                profile.ui.show_fps = *value;
            }
            _ => {}
        }
    }

    pub fn preview_value(&self, profile: &ProfileConfig) -> String {
        format_settings_value(profile, self.entry_id)
    }
}

pub fn adjust_settings_draft(
    profile: &mut ProfileConfig,
    session: &SettingsEditSession,
    delta: i32,
) -> bool {
    adjust_settings_value(profile, session.entry_id, delta)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::app_config::AppConfig;
    use crate::config::profile_config::ProfileConfig;

    #[test]
    fn default_7k_bindings_map_scratch_and_keys() {
        let profile = ProfileConfig::new_default("default", "Default", 0);
        let bindings = SettingsBindings::from_profile(&profile.input);

        assert!(bindings.is_confirm("Z"));
        assert!(bindings.is_confirm("C"));
        assert!(bindings.is_back("S"));
        assert!(bindings.is_back("D"));
        assert!(bindings.is_increase("Axis1-") || bindings.is_increase("LControl"));
    }

    #[test]
    fn default_14k_2p_bindings_map_scratch_and_keys() {
        let profile = ProfileConfig::new_default("default", "Default", 0);
        let bindings = SettingsBindings::from_profile(&profile.input);

        assert!(bindings.is_confirm("M"));
        assert!(bindings.is_confirm("Period"));
        assert!(bindings.is_confirm("Slash"));
        assert!(bindings.is_back("K"));
        assert!(bindings.is_back("L"));
        assert!(bindings.is_back("Semicolon"));
        assert!(bindings.is_decrease("RShift"));
        assert!(bindings.is_increase("RControl"));
    }

    #[test]
    fn cursor_up_increases_and_down_decreases_settings_values() {
        let profile = ProfileConfig::new_default("default", "Default", 0);
        let bindings = SettingsBindings::from_profile(&profile.input);

        assert!(bindings.is_increase("ArrowUp"));
        assert!(!bindings.is_decrease("ArrowUp"));
        assert!(bindings.is_decrease("ArrowDown"));
        assert!(!bindings.is_increase("ArrowDown"));
    }

    #[test]
    fn key9_select_input_maps_settings_navigation_keys() {
        let mut profile = ProfileConfig::new_default("default", "Default", 0);
        profile.input.select_input_mode = SelectInputModeConfig::Key9;
        let bindings = SettingsBindings::from_profile(&profile.input);

        assert!(bindings.is_confirm("C"));
        assert!(bindings.is_confirm("V"));
        assert!(bindings.is_back("X"));
        assert!(bindings.is_decrease("D"));
        assert!(bindings.is_increase("F"));
        assert!(!bindings.is_confirm("Z"));
        assert!(!bindings.is_back("S"));
    }

    #[test]
    fn edit_session_restore_reverts_volume() {
        let mut profile = ProfileConfig::new_default("default", "Default", 0);
        let session = SettingsEditSession::capture(&profile, SettingsEntryId::MasterVolume);
        profile.audio_mix.master_volume = 20;
        session.restore(&mut profile);
        assert_eq!(profile.audio_mix.master_volume, 50);

        let normalize_session =
            SettingsEditSession::capture(&profile, SettingsEntryId::NormalizeChartVolume);
        profile.audio_mix.normalize_chart_volume = false;
        normalize_session.restore(&mut profile);
        assert!(profile.audio_mix.normalize_chart_volume);

        let system_bgm_session =
            SettingsEditSession::capture(&profile, SettingsEntryId::NormalizeSystemBgmVolume);
        profile.audio_mix.normalize_system_bgm_volume = false;
        system_bgm_session.restore(&mut profile);
        assert!(profile.audio_mix.normalize_system_bgm_volume);
    }

    #[test]
    fn app_edit_session_restores_the_edited_config_section() {
        let mut config = AppConfig::default();
        let audio = AppSettingsEditSession::capture(
            &config,
            AppSettingsEntryId::AudioBufferSize,
            AppSettingsChoices::None,
        );
        config.audio.buffer_size = 64;
        audio.restore(&mut config);
        assert_eq!(config.audio.buffer_size, 256);

        let video = AppSettingsEditSession::capture(
            &config,
            AppSettingsEntryId::VideoTargetFps,
            AppSettingsChoices::None,
        );
        config.video.target_fps = 0;
        video.restore(&mut config);
        assert_eq!(config.video.target_fps, 240);
    }

    #[test]
    fn edit_session_restore_reverts_gauge() {
        let mut profile = ProfileConfig::new_default("default", "Default", 0);
        let session = SettingsEditSession::capture(&profile, SettingsEntryId::Gauge);
        profile.play.gauge = GaugeTypeConfig::Hazard;
        session.restore(&mut profile);
        assert_eq!(profile.play.gauge, GaugeTypeConfig::Normal);
    }

    #[test]
    fn edit_session_restore_reverts_input_and_replay_settings() {
        let mut profile = ProfileConfig::new_default("default", "Default", 0);
        let table_level_session =
            SettingsEditSession::capture(&profile, SettingsEntryId::DifficultyTableLevelDisplay);
        profile.select.difficulty_table_level_display = DifficultyTableLevelDisplay::Chart;
        table_level_session.restore(&mut profile);
        assert_eq!(
            profile.select.difficulty_table_level_display,
            DifficultyTableLevelDisplay::Table
        );

        let analog_2p_session =
            SettingsEditSession::capture(&profile, SettingsEntryId::AnalogScratch2P);
        profile.input.gamepad2.analog_scratch = false;
        analog_2p_session.restore(&mut profile);
        assert!(profile.input.gamepad2.analog_scratch);

        let controller_bounce_session =
            SettingsEditSession::capture(&profile, SettingsEntryId::ControllerReleaseBounceMs);
        profile.input.controller_release_bounce_ms = 12;
        controller_bounce_session.restore(&mut profile);
        assert_eq!(profile.input.controller_release_bounce_ms, 0);

        let replay_session =
            SettingsEditSession::capture(&profile, SettingsEntryId::ReplaySlot2Rule);
        profile.replay.slot_rules[1] = ReplaySlotRule::ClearUpdate;
        replay_session.restore(&mut profile);
        assert_eq!(profile.replay.slot_rules[1], ReplaySlotRule::ScoreUpdate);
    }

    #[test]
    fn edit_session_restore_reverts_new_dependent_settings() {
        let mut profile = ProfileConfig::new_default("default", "Default", 0);

        let session_mode = SettingsEditSession::capture(&profile, SettingsEntryId::SessionMode);
        assert!(adjust_settings_draft(&mut profile, &session_mode, 1));
        assert!(adjust_settings_draft(&mut profile, &session_mode, 1));
        assert_eq!(profile.play.session_mode, Some(SessionMode::Autoplay));
        assert!(profile.play.auto_play);
        session_mode.restore(&mut profile);
        assert_eq!(profile.play.session_mode, Some(SessionMode::Normal));
        assert!(!profile.play.auto_play);

        profile.play.double_option = DoubleOptionConfig::Battle;
        let conversion = SettingsEditSession::capture(&profile, SettingsEntryId::KeyModeConversion);
        assert!(adjust_settings_draft(&mut profile, &conversion, 1));
        assert_eq!(profile.play.double_option, DoubleOptionConfig::Off);
        conversion.restore(&mut profile);
        assert_eq!(profile.play.key_mode_conversion, KeyModeConversionConfig::Off);
        assert_eq!(profile.play.double_option, DoubleOptionConfig::Battle);

        let assist = SettingsEditSession::capture(&profile, SettingsEntryId::AssistScrollRate);
        assert!(adjust_settings_draft(&mut profile, &assist, 5));
        assist.restore(&mut profile);
        assert!((profile.play.assist.scroll_rate - 0.5).abs() < f64::EPSILON);

        let note_retention = SettingsEditSession::capture(&profile, SettingsEntryId::NoteRetention);
        assert!(adjust_settings_draft(&mut profile, &note_retention, 1));
        assert!(profile.play.note_retention);
        note_retention.restore(&mut profile);
        assert!(!profile.play.note_retention);

        let duration =
            SettingsEditSession::capture(&profile, SettingsEntryId::NoteDisplayDurationMs);
        assert!(adjust_settings_draft(&mut profile, &duration, 10));
        assert_eq!(profile.lane.target_green_number, 306);
        duration.restore(&mut profile);
        assert_eq!(profile.lane.target_green_number, 300);

        let language = SettingsEditSession::capture(&profile, SettingsEntryId::Language);
        assert!(adjust_settings_draft(&mut profile, &language, 1));
        language.restore(&mut profile);
        assert_eq!(profile.ui.language, "ja");
    }

    #[test]
    fn random_select_settings_toggle_independently_and_cancel() {
        let mut profile = ProfileConfig::new_default("default", "Default", 0);
        let entries = [
            SettingsEntryId::SelectRandomSelect,
            SettingsEntryId::SelectRandomNoPlay,
            SettingsEntryId::SelectRandomFailed,
            SettingsEntryId::SelectRandomNotEasy,
            SettingsEntryId::SelectRandomNotClear,
            SettingsEntryId::SelectRandomNotHard,
            SettingsEntryId::SelectRandomNotExHard,
            SettingsEntryId::SelectRandomNotFullCombo,
        ];
        for (index, entry) in entries.into_iter().enumerate() {
            assert!(SettingsEntryId::SELECT_ENTRIES.contains(&entry));
            let session = SettingsEditSession::capture(&profile, entry);
            assert!(!adjust_settings_draft(&mut profile, &session, 0));
            assert!(adjust_settings_draft(&mut profile, &session, 1));
            let mut expected = [false; 8];
            expected[index] = true;
            assert_eq!(profile.select.random_select_flags(), expected);
            session.restore(&mut profile);
            assert_eq!(profile.select.random_select_flags(), [false; 8]);
        }
    }
}
