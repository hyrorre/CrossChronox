use super::*;

#[derive(Debug, Clone, Default)]
pub(super) struct KeyboardSlots {
    pub(super) primary: Option<String>,
    pub(super) secondary: Option<String>,
}

impl KeyboardSlots {
    pub(super) fn get(self, slot: KeyBindingSlot) -> Option<String> {
        match slot {
            KeyBindingSlot::KeyboardPrimary => self.primary,
            KeyBindingSlot::KeyboardSecondary => self.secondary,
            KeyBindingSlot::Controller
            | KeyBindingSlot::Controller1P
            | KeyBindingSlot::Controller2P => None,
        }
    }
}

pub(super) fn read_keyboard_slots<'a>(
    entries: impl Iterator<Item = &'a BindingConfigEntry>,
) -> KeyboardSlots {
    let mut slots = KeyboardSlots::default();
    let mut legacy = Vec::new();
    for entry in entries {
        match entry.keyboard_slot {
            Some(KeyboardBindingSlotConfig::Primary) if slots.primary.is_none() => {
                slots.primary = Some(entry.control.clone());
            }
            Some(KeyboardBindingSlotConfig::Secondary) if slots.secondary.is_none() => {
                slots.secondary = Some(entry.control.clone());
            }
            Some(_) => {}
            None => legacy.push(entry.control.clone()),
        }
    }
    for control in legacy {
        if slots.primary.is_none() {
            slots.primary = Some(control);
        } else if slots.secondary.is_none() {
            slots.secondary = Some(control);
        }
    }
    slots
}

pub(super) fn read_lane_keyboard_slots(
    bindings: &[BindingConfigEntry],
    lane: LaneConfig,
) -> KeyboardSlots {
    read_keyboard_slots(
        bindings.iter().filter(|entry| entry.device == "keyboard" && entry.lane == Some(lane)),
    )
}

pub(super) fn read_action_keyboard_slots(
    input: &ProfileInputConfig,
    action: InputActionConfig,
) -> KeyboardSlots {
    read_keyboard_slots(
        input
            .ui
            .bindings
            .iter()
            .filter(|entry| entry.device == "keyboard" && entry.action == Some(action)),
    )
}

#[derive(Debug, Clone, Default)]
pub(super) struct ScratchKeyboardSlots {
    pub(super) up_primary: Option<String>,
    pub(super) down_primary: Option<String>,
    pub(super) up_secondary: Option<String>,
    pub(super) down_secondary: Option<String>,
}

impl ScratchKeyboardSlots {
    pub(super) fn get(self, direction: ScratchDirection, slot: KeyBindingSlot) -> Option<String> {
        match (direction, slot) {
            (ScratchDirection::Up, KeyBindingSlot::KeyboardPrimary) => self.up_primary,
            (ScratchDirection::Down, KeyBindingSlot::KeyboardPrimary) => self.down_primary,
            (ScratchDirection::Up, KeyBindingSlot::KeyboardSecondary) => self.up_secondary,
            (ScratchDirection::Down, KeyBindingSlot::KeyboardSecondary) => self.down_secondary,
            (
                _,
                KeyBindingSlot::Controller
                | KeyBindingSlot::Controller1P
                | KeyBindingSlot::Controller2P,
            ) => None,
        }
    }

    pub(super) fn set(
        &mut self,
        direction: ScratchDirection,
        slot: KeyBindingSlot,
        control: Option<String>,
    ) {
        match (direction, slot) {
            (ScratchDirection::Up, KeyBindingSlot::KeyboardPrimary) => self.up_primary = control,
            (ScratchDirection::Down, KeyBindingSlot::KeyboardPrimary) => {
                self.down_primary = control
            }
            (ScratchDirection::Up, KeyBindingSlot::KeyboardSecondary) => {
                self.up_secondary = control
            }
            (ScratchDirection::Down, KeyBindingSlot::KeyboardSecondary) => {
                self.down_secondary = control
            }
            (
                _,
                KeyBindingSlot::Controller
                | KeyBindingSlot::Controller1P
                | KeyBindingSlot::Controller2P,
            ) => {}
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct ScratchGamepadSlots {
    pub(super) up: Option<String>,
    pub(super) down: Option<String>,
}

impl ScratchGamepadSlots {
    pub(super) fn get(self, direction: ScratchDirection) -> Option<String> {
        match direction {
            ScratchDirection::Up => self.up,
            ScratchDirection::Down => self.down,
        }
    }

    pub(super) fn set(&mut self, direction: ScratchDirection, control: Option<String>) {
        match direction {
            ScratchDirection::Up => self.up = control,
            ScratchDirection::Down => self.down = control,
        }
    }
}

pub(super) fn read_scratch_keyboard_slots(
    bindings: &[BindingConfigEntry],
    lane: LaneConfig,
) -> ScratchKeyboardSlots {
    let keyboard_entries =
        || bindings.iter().filter(|entry| entry.device == "keyboard" && entry.lane == Some(lane));
    let up = read_keyboard_slots(
        keyboard_entries().filter(|entry| entry.scratch == Some(ScratchDirectionConfig::Up)),
    );
    let down = read_keyboard_slots(
        keyboard_entries().filter(|entry| entry.scratch == Some(ScratchDirectionConfig::Down)),
    );
    let mut slots = ScratchKeyboardSlots {
        up_primary: up.primary,
        down_primary: down.primary,
        up_secondary: up.secondary,
        down_secondary: down.secondary,
    };

    // scratch direction を持たない旧 profile は従来の表示順
    // (UP, DOWN, UP, DOWN) をフォールバックとして使う。
    let undirected: Vec<_> = keyboard_entries()
        .filter(|entry| entry.scratch.is_none())
        .map(|entry| entry.control.clone())
        .collect();
    for (index, control) in undirected.iter().cloned().enumerate() {
        match index {
            0 if slots.up_primary.is_none() => slots.up_primary = Some(control),
            1 if slots.down_primary.is_none() => slots.down_primary = Some(control),
            2 if slots.up_secondary.is_none() => slots.up_secondary = Some(control),
            3 if slots.down_secondary.is_none() => slots.down_secondary = Some(control),
            _ => {}
        }
    }
    if !undirected.is_empty() && slots.down_primary.is_none() {
        slots.down_primary = slots.up_primary.clone();
    }
    if !undirected.is_empty() && slots.down_secondary.is_none() {
        slots.down_secondary = slots.up_secondary.clone();
    }
    slots
}

pub(super) fn read_scratch_gamepad_slots_for_device(
    bindings: &[BindingConfigEntry],
    lane: LaneConfig,
    device: &str,
) -> ScratchGamepadSlots {
    let mut slots = ScratchGamepadSlots::default();
    let mut undirected = Vec::new();

    // 明示の direction タグを最優先する。コントロール名 (+/-) からの推測は
    // 旧 entry 向けのフォールバックで、軸極性が逆のデバイスでは当てにならない。
    for entry in bindings
        .iter()
        .filter(|e| device_matches_for_read(&e.device, device) && e.lane == Some(lane))
    {
        let control = entry.control.clone();
        match entry.scratch {
            Some(ScratchDirectionConfig::Up) => slots.up = Some(control),
            Some(ScratchDirectionConfig::Down) => slots.down = Some(control),
            None => {
                if is_scratch_up_control(&control) {
                    slots.up = Some(control);
                } else if is_scratch_down_control(&control) {
                    slots.down = Some(control);
                } else {
                    undirected.push(control);
                }
            }
        }
    }

    if let Some(control) = undirected.into_iter().next() {
        if slots.up.is_none() {
            slots.up = Some(control.clone());
        }
        if slots.down.is_none() {
            slots.down = Some(control);
        }
    }

    slots
}

pub(super) fn gamepad_controls_for_lane_device(
    bindings: &[BindingConfigEntry],
    lane: LaneConfig,
    device: &str,
) -> Vec<String> {
    bindings
        .iter()
        .filter(|entry| device_matches_for_read(&entry.device, device) && entry.lane == Some(lane))
        .map(|entry| entry.control.clone())
        .collect()
}

pub(super) fn remove_lane_device_bindings(
    bindings: &mut Vec<BindingConfigEntry>,
    lane: LaneConfig,
    device: &str,
) {
    bindings.retain(|entry| !(device_matches(&entry.device, device) && entry.lane == Some(lane)));
}

pub(super) fn action_controls_for_slot(
    input: &ProfileInputConfig,
    action: InputActionConfig,
    slot: KeyBindingSlot,
) -> Vec<String> {
    input
        .ui
        .bindings
        .iter()
        .filter(|entry| {
            device_matches(&entry.device, slot.device()) && entry.action == Some(action)
        })
        .map(|entry| entry.control.clone())
        .collect()
}

pub(super) fn remove_action_device_bindings(
    input: &mut ProfileInputConfig,
    action: InputActionConfig,
    device: &str,
) {
    input
        .ui
        .bindings
        .retain(|entry| !(device_matches(&entry.device, device) && entry.action == Some(action)));
}

pub(super) fn device_matches(entry_device: &str, requested_device: &str) -> bool {
    if requested_device == "gamepad" {
        is_gamepad_device(entry_device)
    } else {
        entry_device == requested_device
    }
}

/// 表示・読取用。番号付きスロットは exact match を優先し、無ければ wildcard をフォールバック。
pub(super) fn device_matches_for_read(entry_device: &str, requested_device: &str) -> bool {
    if requested_device == "gamepad" {
        is_gamepad_device(entry_device)
    } else if is_gamepad_device(requested_device) {
        entry_device == requested_device || entry_device.eq_ignore_ascii_case("gamepad")
    } else {
        entry_device == requested_device
    }
}
