/// スキン設定パネルからのアクション要求。
pub(in crate::ui) struct SkinPanelActions {
    /// 「保存」ボタンが押された (profile.toml へ書き出し)。
    pub(in crate::ui) save: bool,
    /// 「リセット」ボタンが押された (profile.toml の値へ戻す)。
    pub(in crate::ui) reset: bool,
    /// パネル内のスキン設定変更に対して必要な反映対象。
    pub(in crate::ui) reload: SkinReloadRequest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::ui) enum SkinSlot {
    Select,
    Decide,
    Play4,
    Play5,
    Play6,
    Play7,
    Play8,
    Play9,
    Play10,
    Play14,
    Battle5,
    Battle7,
    Result,
    CourseResult,
}

impl SkinSlot {
    /// locale を切り替えても egui の永続 widget ID が変わらないよう、
    /// i18n 前に ID へ使われていた日本語ラベルを固定 salt として維持する。
    pub(in crate::ui) const fn path_combo_id(self) -> &'static str {
        match self {
            Self::Select => "選曲",
            Self::Decide => "決定",
            Self::Play4 => "プレイ (4K)",
            Self::Play5 => "プレイ (5K)",
            Self::Play6 => "プレイ (6K)",
            Self::Play7 => "プレイ (7K)",
            Self::Play8 => "プレイ (8K)",
            Self::Play9 => "プレイ (9K)",
            Self::Play10 => "プレイ (10K)",
            Self::Play14 => "プレイ (14K)",
            Self::Battle5 => "プレイ (5K BATTLE)",
            Self::Battle7 => "プレイ (7K BATTLE)",
            Self::Result => "リザルト",
            Self::CourseResult => "コースリザルト",
        }
    }

    pub(in crate::ui) const fn defs_header_id(self) -> &'static str {
        match self {
            Self::Select => "選曲スキン",
            Self::Decide => "決定スキン",
            Self::Play4 => "プレイスキン (4K)",
            Self::Play5 => "プレイスキン (5K)",
            Self::Play6 => "プレイスキン (6K)",
            Self::Play7 => "プレイスキン (7K)",
            Self::Play8 => "プレイスキン (8K)",
            Self::Play9 => "プレイスキン (9K)",
            Self::Play10 => "プレイスキン (10K)",
            Self::Play14 => "プレイスキン (14K)",
            Self::Battle5 => "プレイスキン (5K BATTLE)",
            Self::Battle7 => "プレイスキン (7K BATTLE)",
            Self::Result => "リザルトスキン",
            Self::CourseResult => "コースリザルトスキン",
        }
    }
}

pub(in crate::ui) fn skin_scene_label(slot: SkinSlot, text: Localizer) -> String {
    match slot {
        SkinSlot::Select => tr!(text, "skin-scene-select"),
        SkinSlot::Decide => tr!(text, "skin-scene-decide"),
        SkinSlot::Play4 => tr!(text, "skin-scene-play", "keys" => "4K"),
        SkinSlot::Play5 => tr!(text, "skin-scene-play", "keys" => "5K"),
        SkinSlot::Play6 => tr!(text, "skin-scene-play", "keys" => "6K"),
        SkinSlot::Play7 => tr!(text, "skin-scene-play", "keys" => "7K"),
        SkinSlot::Play8 => tr!(text, "skin-scene-play", "keys" => "8K"),
        SkinSlot::Play9 => tr!(text, "skin-scene-play", "keys" => "9K"),
        SkinSlot::Play10 => tr!(text, "skin-scene-play", "keys" => "10K"),
        SkinSlot::Play14 => tr!(text, "skin-scene-play", "keys" => "14K"),
        SkinSlot::Battle5 => tr!(text, "skin-scene-play", "keys" => "5K BATTLE"),
        SkinSlot::Battle7 => tr!(text, "skin-scene-play", "keys" => "7K BATTLE"),
        SkinSlot::Result => tr!(text, "skin-scene-result"),
        SkinSlot::CourseResult => tr!(text, "skin-scene-course-result"),
    }
}

pub(in crate::ui) fn skin_scene_defs_label(slot: SkinSlot, text: Localizer) -> String {
    tr!(text, "skin-scene-options", "scene" => skin_scene_label(slot, text))
}

pub(in crate::ui) fn request_skin_reload(
    request: &mut SkinReloadRequest,
    slot: SkinSlot,
    offsets_changed: bool,
) {
    match slot {
        SkinSlot::Select => request.select = true,
        SkinSlot::Decide => request.decide = true,
        SkinSlot::Play4 => request.play4 = true,
        SkinSlot::Play5 => request.play5 = true,
        SkinSlot::Play6 => request.play6 = true,
        SkinSlot::Play7 => request.play7 = true,
        SkinSlot::Play8 => request.play8 = true,
        SkinSlot::Play9 => request.play9 = true,
        SkinSlot::Play10 => request.play10 = true,
        SkinSlot::Play14 => request.play14 = true,
        SkinSlot::Battle5 => request.play5 = true,
        SkinSlot::Battle7 => request.play7 = true,
        SkinSlot::Result => request.result = true,
        SkinSlot::CourseResult => request.course_result = true,
    }
    request.offsets |= offsets_changed;
}

#[cfg(test)]
pub(in crate::ui) fn skin_reload_request_from_diff(
    before: &SkinConfig,
    after: &SkinConfig,
) -> SkinReloadRequest {
    let mut request = SkinReloadRequest::default();
    let select_offsets_changed = before.select_offsets != after.select_offsets;
    let decide_offsets_changed = before.decide_offsets != after.decide_offsets;
    let play4_offsets_changed = before.play4_offsets != after.play4_offsets;
    let play5_offsets_changed = before.play5_offsets != after.play5_offsets;
    let play6_offsets_changed = before.play6_offsets != after.play6_offsets;
    let play7_offsets_changed = before.play7_offsets != after.play7_offsets;
    let play8_offsets_changed = before.play8_offsets != after.play8_offsets;
    let play9_offsets_changed = before.play9_offsets != after.play9_offsets;
    let play10_offsets_changed = before.play10_offsets != after.play10_offsets;
    let play14_offsets_changed = before.play14_offsets != after.play14_offsets;
    let battle5_offsets_changed = before.battle5_offsets != after.battle5_offsets;
    let battle7_offsets_changed = before.battle7_offsets != after.battle7_offsets;
    let result_offsets_changed = before.result_offsets != after.result_offsets;
    let course_result_offsets_changed = before.course_result_offsets != after.course_result_offsets;
    if before.select != after.select
        || before.select_options != after.select_options
        || before.select_files != after.select_files
        || select_offsets_changed
    {
        request.select = true;
    }
    if before.decide != after.decide
        || before.decide_options != after.decide_options
        || before.decide_files != after.decide_files
        || decide_offsets_changed
    {
        request.decide = true;
    }
    if before.play4 != after.play4
        || before.play4_options != after.play4_options
        || before.play4_files != after.play4_files
        || play4_offsets_changed
    {
        request.play4 = true;
    }
    if before.play5 != after.play5
        || before.play5_options != after.play5_options
        || before.play5_files != after.play5_files
        || play5_offsets_changed
    {
        request.play5 = true;
    }
    if before.play6 != after.play6
        || before.play6_options != after.play6_options
        || before.play6_files != after.play6_files
        || play6_offsets_changed
    {
        request.play6 = true;
    }
    if before.play7 != after.play7
        || before.play7_options != after.play7_options
        || before.play7_files != after.play7_files
        || play7_offsets_changed
    {
        request.play7 = true;
    }
    if before.play8 != after.play8
        || before.play8_options != after.play8_options
        || before.play8_files != after.play8_files
        || play8_offsets_changed
    {
        request.play8 = true;
    }
    if before.play9 != after.play9
        || before.play9_options != after.play9_options
        || before.play9_files != after.play9_files
        || play9_offsets_changed
    {
        request.play9 = true;
    }
    if before.play10 != after.play10
        || before.play10_options != after.play10_options
        || before.play10_files != after.play10_files
        || play10_offsets_changed
    {
        request.play10 = true;
    }
    if before.play14 != after.play14
        || before.play14_options != after.play14_options
        || before.play14_files != after.play14_files
        || play14_offsets_changed
    {
        request.play14 = true;
    }
    if before.battle5 != after.battle5
        || before.battle5_options != after.battle5_options
        || before.battle5_files != after.battle5_files
        || battle5_offsets_changed
    {
        request.play5 = true;
    }
    if before.battle7 != after.battle7
        || before.battle7_options != after.battle7_options
        || before.battle7_files != after.battle7_files
        || battle7_offsets_changed
    {
        request.play7 = true;
    }
    if before.result != after.result
        || before.result_options != after.result_options
        || before.result_files != after.result_files
        || result_offsets_changed
    {
        request.result = true;
    }
    if before.course_result != after.course_result
        || before.course_result_options != after.course_result_options
        || before.course_result_files != after.course_result_files
        || course_result_offsets_changed
    {
        request.course_result = true;
    }
    request.offsets = select_offsets_changed
        || decide_offsets_changed
        || play4_offsets_changed
        || play5_offsets_changed
        || play6_offsets_changed
        || play7_offsets_changed
        || play8_offsets_changed
        || play9_offsets_changed
        || play10_offsets_changed
        || play14_offsets_changed
        || battle5_offsets_changed
        || battle7_offsets_changed
        || result_offsets_changed
        || course_result_offsets_changed;
    request
}
use super::*;
