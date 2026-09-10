use super::*;

pub(super) fn write_action_keyboard_bindings(
    input: &mut ProfileInputConfig,
    action: InputActionConfig,
    primary: Option<&str>,
    secondary: Option<&str>,
) {
    remove_action_device_bindings(input, action, "keyboard");
    if let Some(control) = primary.filter(|value| !value.is_empty()) {
        input.ui.bindings.push(action_binding_for_device(
            "keyboard",
            control,
            action,
            Some(KeyboardBindingSlotConfig::Primary),
        ));
    }
    if let Some(control) = secondary.filter(|value| !value.is_empty()) {
        input.ui.bindings.push(action_binding_for_device(
            "keyboard",
            control,
            action,
            Some(KeyboardBindingSlotConfig::Secondary),
        ));
    }
}

pub(super) fn write_action_gamepad_bindings(
    input: &mut ProfileInputConfig,
    action: InputActionConfig,
    controls: &[String],
) {
    remove_action_device_bindings(input, action, "gamepad");
    for control in controls {
        if !control.is_empty() {
            input.ui.bindings.push(action_binding_for_device("gamepad", control, action, None));
        }
    }
}

pub(super) fn action_binding_for_device(
    device: &str,
    control: &str,
    action: InputActionConfig,
    keyboard_slot: Option<KeyboardBindingSlotConfig>,
) -> BindingConfigEntry {
    BindingConfigEntry {
        device: device.to_string(),
        control: control.to_string(),
        keyboard_slot,
        lane: None,
        action: Some(action),
        scratch: None,
    }
}

pub(super) fn apply_action_binding(
    input: &mut ProfileInputConfig,
    action: InputActionConfig,
    slot: KeyBindingSlot,
    control: &str,
) {
    let keyboard = read_action_keyboard_slots(input, action);
    let primary = keyboard.primary;
    let secondary = keyboard.secondary;
    let gamepad = action_controls_for_slot(input, action, KeyBindingSlot::Controller);

    remove_action_device_bindings(input, action, "keyboard");
    remove_action_device_bindings(input, action, "gamepad");

    match slot {
        KeyBindingSlot::KeyboardPrimary => {
            write_action_keyboard_bindings(input, action, Some(control), secondary.as_deref());
            write_action_gamepad_bindings(input, action, &gamepad);
        }
        KeyBindingSlot::KeyboardSecondary => {
            write_action_keyboard_bindings(input, action, primary.as_deref(), Some(control));
            write_action_gamepad_bindings(input, action, &gamepad);
        }
        KeyBindingSlot::Controller
        | KeyBindingSlot::Controller1P
        | KeyBindingSlot::Controller2P => {
            write_action_keyboard_bindings(input, action, primary.as_deref(), secondary.as_deref());
            write_action_gamepad_bindings(input, action, &[control.to_string()]);
        }
    }
}

pub(super) fn clear_action_binding(
    input: &mut ProfileInputConfig,
    action: InputActionConfig,
    slot: KeyBindingSlot,
) {
    let keyboard = read_action_keyboard_slots(input, action);
    let primary = keyboard.primary;
    let secondary = keyboard.secondary;
    let gamepad = action_controls_for_slot(input, action, KeyBindingSlot::Controller);

    remove_action_device_bindings(input, action, "keyboard");
    remove_action_device_bindings(input, action, "gamepad");

    match slot {
        KeyBindingSlot::KeyboardPrimary => {
            write_action_keyboard_bindings(input, action, None, secondary.as_deref());
            write_action_gamepad_bindings(input, action, &gamepad);
        }
        KeyBindingSlot::KeyboardSecondary => {
            write_action_keyboard_bindings(input, action, primary.as_deref(), None);
            write_action_gamepad_bindings(input, action, &gamepad);
        }
        KeyBindingSlot::Controller
        | KeyBindingSlot::Controller1P
        | KeyBindingSlot::Controller2P => {
            write_action_keyboard_bindings(input, action, primary.as_deref(), secondary.as_deref());
            write_action_gamepad_bindings(input, action, &[]);
        }
    }
}

pub(super) fn write_lane_keyboard_bindings(
    bindings: &mut Vec<BindingConfigEntry>,
    lane: LaneConfig,
    primary: Option<&str>,
    secondary: Option<&str>,
) {
    remove_lane_device_bindings(bindings, lane, "keyboard");
    if let Some(control) = primary.filter(|value| !value.is_empty()) {
        let mut entry = play_binding(control, lane);
        entry.keyboard_slot = Some(KeyboardBindingSlotConfig::Primary);
        bindings.push(entry);
    }
    if let Some(control) = secondary.filter(|value| !value.is_empty()) {
        let mut entry = play_binding(control, lane);
        entry.keyboard_slot = Some(KeyboardBindingSlotConfig::Secondary);
        bindings.push(entry);
    }
}

pub(super) fn write_scratch_keyboard_bindings(
    bindings: &mut Vec<BindingConfigEntry>,
    lane: LaneConfig,
    slots: &ScratchKeyboardSlots,
) {
    remove_lane_device_bindings(bindings, lane, "keyboard");
    for (control, direction, keyboard_slot) in [
        (
            slots.up_primary.as_deref(),
            ScratchDirectionConfig::Up,
            KeyboardBindingSlotConfig::Primary,
        ),
        (
            slots.down_primary.as_deref(),
            ScratchDirectionConfig::Down,
            KeyboardBindingSlotConfig::Primary,
        ),
        (
            slots.up_secondary.as_deref(),
            ScratchDirectionConfig::Up,
            KeyboardBindingSlotConfig::Secondary,
        ),
        (
            slots.down_secondary.as_deref(),
            ScratchDirectionConfig::Down,
            KeyboardBindingSlotConfig::Secondary,
        ),
    ] {
        if let Some(control) = control.filter(|value| !value.is_empty()) {
            let mut entry = scratch_play_binding(control, lane, direction);
            entry.keyboard_slot = Some(keyboard_slot);
            bindings.push(entry);
        }
    }
}

pub(super) fn write_scratch_gamepad_bindings_for_device(
    bindings: &mut Vec<BindingConfigEntry>,
    lane: LaneConfig,
    device: &str,
    slots: &ScratchGamepadSlots,
) {
    remove_lane_device_bindings(bindings, lane, device);
    if device != "gamepad" {
        // 番号付きへ書き込むときは wildcard を消して二重マッチを防ぐ。
        bindings.retain(|entry| {
            !(entry.device.eq_ignore_ascii_case("gamepad") && entry.lane == Some(lane))
        });
    }
    for (control, direction) in [
        (slots.up.as_deref(), ScratchDirectionConfig::Up),
        (slots.down.as_deref(), ScratchDirectionConfig::Down),
    ] {
        if let Some(control) = control.filter(|value| !value.is_empty()) {
            let mut entry = gamepad_play_binding_for_device(device, control, lane);
            entry.scratch = Some(direction);
            bindings.push(entry);
        }
    }
}

pub(super) fn write_lane_gamepad_bindings_for_device(
    bindings: &mut Vec<BindingConfigEntry>,
    lane: LaneConfig,
    device: &str,
    controls: &[String],
) {
    remove_lane_device_bindings(bindings, lane, device);
    if device != "gamepad" {
        bindings.retain(|entry| {
            !(entry.device.eq_ignore_ascii_case("gamepad") && entry.lane == Some(lane))
        });
    }
    for control in controls {
        if !control.is_empty() {
            bindings.push(gamepad_play_binding_for_device(device, control, lane));
        }
    }
}

pub(super) fn persist_bindings(
    input: &mut ProfileInputConfig,
    key_mode: KeyMode,
    bindings: Vec<BindingConfigEntry>,
) -> Result<(), crate::config::play_input::InheritError> {
    let config = ensure_play_mode_config(input, key_mode);
    config.inherit = None;
    config.bindings = bindings;
    Ok(())
}

pub(super) fn lane_for_target(target: KeyBindingTarget) -> LaneConfig {
    match target {
        KeyBindingTarget::Key { lane, .. } | KeyBindingTarget::Scratch { lane, .. } => lane,
        KeyBindingTarget::Action { .. } => LaneConfig::Key1,
    }
}
