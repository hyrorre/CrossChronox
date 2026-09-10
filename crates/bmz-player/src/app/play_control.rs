use crate::config::profile_config::{InputActionConfig, ProfileInputConfig};
use bmz_gameplay::input::backend::PhysicalControl;

use super::input_runtime::ControlInputEvent;
use super::{LANE_COVER_REPEAT_STEP, LANE_COVER_STEP};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum HispeedChange {
    Down,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum PlayLaneAction {
    ToggleHispeedMode,
    Hispeed(HispeedChange),
    LaneCoverDelta(f32),
    AnalogLaneCoverDelta(f32),
    GreenNumberDelta(i32),
    ToggleLaneCoverVisibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PlayLaneTarget {
    Sudden,
    Lift,
    Hidden,
}

impl PlayLaneTarget {
    pub(super) const fn toggled_lift_hidden(self) -> Self {
        match self {
            Self::Hidden => Self::Lift,
            Self::Sudden | Self::Lift => Self::Hidden,
        }
    }
}

pub(super) fn resolved_play_lane_target(
    sudden_enabled: bool,
    lane_cover_visible: bool,
    lift_enabled: bool,
    hidden_enabled: bool,
    preferred: PlayLaneTarget,
) -> Option<PlayLaneTarget> {
    if sudden_enabled && lane_cover_visible {
        return Some(PlayLaneTarget::Sudden);
    }
    match (lift_enabled, hidden_enabled) {
        (true, true) => Some(match preferred {
            PlayLaneTarget::Hidden => PlayLaneTarget::Hidden,
            PlayLaneTarget::Sudden | PlayLaneTarget::Lift => PlayLaneTarget::Lift,
        }),
        (true, false) => Some(PlayLaneTarget::Lift),
        (false, true) => Some(PlayLaneTarget::Hidden),
        (false, false) => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PlayOptionControl {
    ToggleHispeedMode,
    Hispeed(HispeedChange),
    LaneCover(LaneCoverChange),
    GreenNumber(GreenNumberChange),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PlayAnalogOptionMode {
    LaneCover,
    GreenNumber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LaneCoverChange {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GreenNumberChange {
    Up,
    Down,
}

pub(super) fn keyboard_lane_action(
    event: &ControlInputEvent,
    input: &ProfileInputConfig,
) -> Option<PlayLaneAction> {
    if !event.pressed {
        return None;
    }
    let PhysicalControl::KeyboardKey(control) = event.physical.as_ref()? else {
        return None;
    };
    let lane_cover_step = if event.repeat { LANE_COVER_REPEAT_STEP } else { LANE_COVER_STEP };
    let action = input.ui.bindings.iter().find_map(|entry| {
        (entry.device == "keyboard" && entry.control == *control)
            .then_some(entry.action)
            .flatten()
            .filter(|action| {
                crate::config::profile_config::PLAY_KEYBOARD_SHORTCUT_ACTIONS.contains(action)
            })
    })?;
    match action {
        InputActionConfig::PlayHispeedDown => Some(PlayLaneAction::Hispeed(HispeedChange::Down)),
        InputActionConfig::PlayHispeedUp => Some(PlayLaneAction::Hispeed(HispeedChange::Up)),
        InputActionConfig::PlayLaneCoverUp => Some(PlayLaneAction::LaneCoverDelta(lane_cover_step)),
        InputActionConfig::PlayLaneCoverDown => {
            Some(PlayLaneAction::LaneCoverDelta(-lane_cover_step))
        }
        _ => None,
    }
}

pub(super) fn lane_action_from_option(
    action: PlayOptionControl,
    is_axis: bool,
) -> Option<PlayLaneAction> {
    match action {
        PlayOptionControl::ToggleHispeedMode => Some(PlayLaneAction::ToggleHispeedMode),
        PlayOptionControl::Hispeed(change) => Some(PlayLaneAction::Hispeed(change)),
        PlayOptionControl::LaneCover(_) if is_axis => None,
        PlayOptionControl::LaneCover(LaneCoverChange::Up) => {
            Some(PlayLaneAction::LaneCoverDelta(LANE_COVER_STEP))
        }
        PlayOptionControl::LaneCover(LaneCoverChange::Down) => {
            Some(PlayLaneAction::LaneCoverDelta(-LANE_COVER_STEP))
        }
        PlayOptionControl::GreenNumber(_) if is_axis => None,
        PlayOptionControl::GreenNumber(GreenNumberChange::Up) => {
            Some(PlayLaneAction::GreenNumberDelta(1))
        }
        PlayOptionControl::GreenNumber(GreenNumberChange::Down) => {
            Some(PlayLaneAction::GreenNumberDelta(-1))
        }
    }
}

#[cfg(test)]
mod tests {
    use winit::event::ElementState;
    use winit::keyboard::{KeyCode, PhysicalKey};

    use super::*;

    fn keyboard(code: KeyCode, repeat: bool) -> ControlInputEvent {
        ControlInputEvent::keyboard_parts(PhysicalKey::Code(code), ElementState::Pressed, repeat)
    }

    #[test]
    fn keyboard_arrows_map_to_shared_lane_actions() {
        let input = crate::config::play_input::default_profile_input();
        assert_eq!(
            keyboard_lane_action(&keyboard(KeyCode::ArrowLeft, false), &input),
            Some(PlayLaneAction::Hispeed(HispeedChange::Down))
        );
        assert_eq!(
            keyboard_lane_action(&keyboard(KeyCode::ArrowUp, false), &input),
            Some(PlayLaneAction::LaneCoverDelta(LANE_COVER_STEP))
        );
        assert_eq!(
            keyboard_lane_action(&keyboard(KeyCode::ArrowDown, true), &input),
            Some(PlayLaneAction::LaneCoverDelta(-LANE_COVER_REPEAT_STEP))
        );
    }

    #[test]
    fn remapped_and_cleared_shortcuts_survive_profile_reload() {
        use crate::config::key_config::{
            KeyBindingSlot, KeyBindingTarget, apply_play_binding, clear_play_binding,
        };
        use bmz_core::lane::KeyMode;
        let mut input = crate::config::play_input::default_profile_input();
        for (action, old_key, new_key, control, expected) in [
            (
                InputActionConfig::PlayHispeedDown,
                KeyCode::ArrowLeft,
                KeyCode::KeyH,
                "H",
                PlayLaneAction::Hispeed(HispeedChange::Down),
            ),
            (
                InputActionConfig::PlayHispeedUp,
                KeyCode::ArrowRight,
                KeyCode::KeyJ,
                "J",
                PlayLaneAction::Hispeed(HispeedChange::Up),
            ),
            (
                InputActionConfig::PlayLaneCoverUp,
                KeyCode::ArrowUp,
                KeyCode::KeyK,
                "K",
                PlayLaneAction::LaneCoverDelta(LANE_COVER_REPEAT_STEP),
            ),
            (
                InputActionConfig::PlayLaneCoverDown,
                KeyCode::ArrowDown,
                KeyCode::KeyL,
                "L",
                PlayLaneAction::LaneCoverDelta(-LANE_COVER_REPEAT_STEP),
            ),
        ] {
            let target = KeyBindingTarget::Action { action, slot: KeyBindingSlot::KeyboardPrimary };
            apply_play_binding(&mut input, KeyMode::K7, target, control).unwrap();
            input = toml::from_str(&toml::to_string(&input).unwrap()).unwrap();
            crate::config::play_input::normalize_profile_input(&mut input);
            assert_eq!(keyboard_lane_action(&keyboard(old_key, false), &input), None);
            assert_eq!(keyboard_lane_action(&keyboard(new_key, true), &input), Some(expected));
            let mut release = keyboard(new_key, false);
            release.pressed = false;
            assert_eq!(keyboard_lane_action(&release, &input), None);
            clear_play_binding(&mut input, KeyMode::K7, target).unwrap();
            input = toml::from_str(&toml::to_string(&input).unwrap()).unwrap();
            crate::config::play_input::normalize_profile_input(&mut input);
            assert_eq!(keyboard_lane_action(&keyboard(new_key, false), &input), None);
            assert_eq!(keyboard_lane_action(&keyboard(old_key, false), &input), None);
        }
    }

    #[test]
    fn axis_button_events_do_not_duplicate_analog_lane_changes() {
        assert_eq!(
            lane_action_from_option(PlayOptionControl::LaneCover(LaneCoverChange::Up), true,),
            None
        );
        assert_eq!(
            lane_action_from_option(PlayOptionControl::GreenNumber(GreenNumberChange::Down), true,),
            None
        );
        assert_eq!(
            lane_action_from_option(PlayOptionControl::Hispeed(HispeedChange::Up), true,),
            Some(PlayLaneAction::Hispeed(HispeedChange::Up))
        );
    }
}
