#[cfg(windows)]
mod windows {
    use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
    use std::ffi::c_void;
    use std::mem::{size_of, size_of_val};
    use std::slice;
    use std::sync::{Arc, Mutex};
    use std::time::Instant;

    use anyhow::{Context, Result, bail};
    use bmz_gameplay::input::backend::{DeviceId, DeviceTimestamp};
    use windows_sys::Win32::Devices::DeviceAndDriverInstallation::{
        CM_Get_Device_Interface_PropertyW, CR_SUCCESS,
    };
    use windows_sys::Win32::Devices::HumanInterfaceDevice::{
        HIDP_BUTTON_CAPS, HIDP_CAPS, HIDP_DATA, HIDP_STATUS_SUCCESS, HIDP_VALUE_CAPS,
        HidP_GetButtonCaps, HidP_GetCaps, HidP_GetData, HidP_GetValueCaps, HidP_Input,
        HidP_MaxDataListLength, PHIDP_PREPARSED_DATA,
    };
    use windows_sys::Win32::Devices::Properties::{DEVPKEY_Device_ContainerId, DEVPROP_TYPE_GUID};
    use windows_sys::Win32::Foundation::{GetLastError, HANDLE, HWND};
    use windows_sys::Win32::UI::Input::{
        GetRawInputData, GetRawInputDeviceInfoW, GetRawInputDeviceList, RAWHID, RAWINPUT,
        RAWINPUTDEVICE, RAWINPUTDEVICELIST, RAWINPUTHEADER, RID_DEVICE_INFO, RID_DEVICE_INFO_HID,
        RID_INPUT, RIDEV_DEVNOTIFY, RIDEV_REMOVE, RIDI_DEVICEINFO, RIDI_DEVICENAME,
        RIDI_PREPARSEDDATA, RIM_TYPEHID, RegisterRawInputDevices,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GIDC_ARRIVAL, GIDC_REMOVAL, MSG, WM_INPUT, WM_INPUT_DEVICE_CHANGE,
    };
    use windows_sys::core::GUID;
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use winit::window::Window;

    use super::super::gamepad::{
        AnalogGamepadProcessor, ConnectedGamepad, GamepadButtonEvent, GamepadPollOutput,
        GamepadPressedButton, GamepadScratchConfig, GamepadSlotMap, RawControlCode, RawInputEvent,
        RawInputEventKind, current_device_timestamp, gamepad_device_id_from_stable_id,
    };

    const GENERIC_DESKTOP_USAGE_PAGE: u16 = 0x01;
    const JOYSTICK_USAGE: u16 = 0x04;
    const GAMEPAD_USAGE: u16 = 0x05;
    const MULTI_AXIS_USAGE: u16 = 0x08;
    const MAX_QUEUED_EVENTS: usize = 16_384;
    const RAW_INPUT_ERROR: u32 = u32::MAX;

    const REGISTERED_USAGES: [u16; 3] = [JOYSTICK_USAGE, GAMEPAD_USAGE, MULTI_AXIS_USAGE];

    #[derive(Clone)]
    pub struct RawInputBridge {
        shared: Arc<Mutex<RawInputState>>,
    }

    impl Default for RawInputBridge {
        fn default() -> Self {
            Self::new()
        }
    }

    impl RawInputBridge {
        pub(crate) fn register_capture_window(&self, hwnd: HWND) -> Result<()> {
            self.register_window(hwnd)?;
            register_usages(hwnd, RIDEV_DEVNOTIFY | windows_sys::Win32::UI::Input::RIDEV_INPUTSINK)
        }
        pub fn new() -> Self {
            Self { shared: Arc::new(Mutex::new(RawInputState::default())) }
        }

        /// winitのmessage hookから呼ぶ。hook側ではfalseを返し、winitの通常処理と
        /// DefWindowProcWによるWM_INPUTの後処理を継続させる。
        pub fn handle_message(&self, message: *const c_void) {
            if message.is_null() {
                return;
            }
            // SAFETY: winitのmessage hookはコールバック中だけ有効なMSGへのポインタを渡す。
            let message = unsafe { &*message.cast::<MSG>() };
            match message.message {
                WM_INPUT => self.handle_raw_input(message.lParam as _),
                WM_INPUT_DEVICE_CHANGE => {
                    self.handle_device_change(message.wParam as u32, message.lParam as HANDLE);
                }
                _ => {}
            }
        }

        fn handle_raw_input(&self, raw_input: *mut c_void) {
            if !self.shared.lock().is_ok_and(|shared| shared.registered) {
                return;
            }
            let Some((handle, reports)) = read_raw_input_reports(raw_input) else { return };
            let timestamp = current_device_timestamp();
            let Ok(mut shared) = self.shared.lock() else {
                tracing::error!("Raw Input state lock is poisoned");
                return;
            };
            shared.process_reports(handle, &reports, timestamp);
        }

        fn handle_device_change(&self, change: u32, handle: HANDLE) {
            let Ok(mut shared) = self.shared.lock() else {
                tracing::error!("Raw Input state lock is poisoned");
                return;
            };
            if !shared.registered {
                return;
            }
            match change {
                GIDC_ARRIVAL => shared.ensure_device(handle),
                GIDC_REMOVAL => shared.remove_device(handle, current_device_timestamp()),
                _ => {}
            }
        }

        fn register_window(&self, hwnd: HWND) -> Result<()> {
            register_usages(hwnd, RIDEV_DEVNOTIFY)?;
            {
                let mut shared = match self.shared.lock() {
                    Ok(shared) => shared,
                    Err(_) => {
                        let _ = register_usages(std::ptr::null_mut(), RIDEV_REMOVE);
                        bail!("Raw Input state lock is poisoned");
                    }
                };
                shared.registered = true;
            }
            let handles = enumerate_supported_devices()?;
            let mut shared = match self.shared.lock() {
                Ok(shared) => shared,
                Err(_) => {
                    let _ = register_usages(std::ptr::null_mut(), RIDEV_REMOVE);
                    bail!("Raw Input state lock is poisoned");
                }
            };
            for handle in handles {
                shared.ensure_device(handle);
            }
            Ok(())
        }

        fn unregister(&self) {
            let registered = self.shared.lock().map(|shared| shared.registered).unwrap_or(true);
            if !registered {
                return;
            }
            if let Err(error) = register_usages(std::ptr::null_mut(), RIDEV_REMOVE) {
                tracing::warn!(%error, "failed to unregister Raw Input gamepad usages");
            }
            if let Ok(mut shared) = self.shared.lock() {
                shared.deactivate();
            }
        }

        fn drain_poll_state(&self) -> (Vec<QueuedHidEvent>, Vec<GamepadPressedButton>) {
            let Ok(mut shared) = self.shared.lock() else {
                tracing::error!("Raw Input state lock is poisoned");
                return (Vec::new(), Vec::new());
            };
            let events = shared.events.drain(..).collect();
            let mut pressed_buttons = shared
                .devices
                .values()
                .flat_map(HidDeviceState::pressed_button_snapshot)
                .collect::<Vec<_>>();
            pressed_buttons.sort_by(|left, right| {
                left.device_id.0.cmp(&right.device_id.0).then_with(|| left.name.cmp(&right.name))
            });
            (events, pressed_buttons)
        }

        fn connected_gamepads(&self) -> Vec<ConnectedGamepad> {
            let Ok(shared) = self.shared.lock() else {
                tracing::error!("Raw Input state lock is poisoned");
                return Vec::new();
            };
            let mut gamepads = shared
                .known_devices
                .values()
                .map(KnownDevice::connected_gamepad)
                .collect::<Vec<_>>();
            gamepads.sort_by_key(|gamepad| gamepad.backend_id);
            gamepads
        }
    }

    pub struct RawInputBackend {
        bridge: RawInputBridge,
        analog: AnalogGamepadProcessor,
    }

    impl RawInputBackend {
        pub fn new(bridge: RawInputBridge, configs: [GamepadScratchConfig; 2]) -> Self {
            Self { bridge, analog: AnalogGamepadProcessor::new(configs, GamepadSlotMap::default()) }
        }

        pub fn attach_window(&mut self, window: &Window) -> Result<()> {
            let handle = window.window_handle().context("failed to get the Win32 window handle")?;
            let RawWindowHandle::Win32(handle) = handle.as_raw() else {
                bail!("Raw Input requires a Win32 window handle");
            };
            self.bridge.register_window(handle.hwnd.get() as HWND)
        }

        pub fn set_analog_config(
            &mut self,
            configs: [GamepadScratchConfig; 2],
            slots: GamepadSlotMap,
        ) {
            self.analog.set_config(configs, slots);
        }

        pub fn poll(&mut self) -> GamepadPollOutput {
            let mut output = GamepadPollOutput::default();
            self.analog.check_timeouts(Instant::now(), &mut output.buttons);
            let (events, mut pressed_buttons) = self.bridge.drain_poll_state();
            for event in events {
                match event {
                    QueuedHidEvent::Button {
                        device_id,
                        name,
                        logical,
                        raw_code,
                        pressed,
                        timestamp,
                    } => {
                        output.raw_events.push(RawInputEvent {
                            device_id,
                            kind: RawInputEventKind::Button,
                            logical,
                            raw_code,
                            timestamp,
                            mapped_control: Some(name.clone()),
                            pressed: Some(pressed),
                            value: None,
                            ticks: None,
                        });
                        output.buttons.push(GamepadButtonEvent {
                            name,
                            device_id,
                            pressed,
                            timestamp,
                            synthesized_analog_axis: false,
                        });
                    }
                    QueuedHidEvent::Axis {
                        device_id,
                        name,
                        logical,
                        raw_code,
                        value,
                        timestamp,
                    } => self.analog.process_axis(
                        device_id,
                        raw_code.value,
                        &name,
                        logical,
                        raw_code,
                        value,
                        timestamp,
                        &mut output,
                    ),
                    QueuedHidEvent::Disconnected { device_id, timestamp } => {
                        self.analog.release_device(device_id, timestamp, &mut output.buttons);
                    }
                }
            }
            self.analog.check_timeouts(Instant::now(), &mut output.buttons);
            pressed_buttons.extend(self.analog.pressed_buttons());
            pressed_buttons.sort_by(|left, right| {
                left.device_id.0.cmp(&right.device_id.0).then_with(|| left.name.cmp(&right.name))
            });
            pressed_buttons.dedup();
            output.pressed_buttons = Some(pressed_buttons);
            output
        }

        pub fn connected_gamepads(&self) -> Vec<ConnectedGamepad> {
            self.bridge.connected_gamepads()
        }
    }

    impl Drop for RawInputBackend {
        fn drop(&mut self) {
            self.bridge.unregister();
        }
    }

    #[derive(Default)]
    struct RawInputState {
        devices: HashMap<usize, HidDeviceState>,
        rejected_handles: HashSet<usize>,
        known_devices: BTreeMap<String, KnownDevice>,
        events: VecDeque<QueuedHidEvent>,
        next_backend_id: u32,
        dropped_event_count: u64,
        registered: bool,
    }

    impl RawInputState {
        fn deactivate(&mut self) {
            self.registered = false;
            self.devices.clear();
            self.rejected_handles.clear();
            self.events.clear();
            for device in self.known_devices.values_mut() {
                device.is_connected = false;
            }
        }

        fn ensure_device(&mut self, handle: HANDLE) {
            let key = handle as usize;
            if self.devices.contains_key(&key) || self.rejected_handles.contains(&key) {
                return;
            }
            let mut device = match HidDeviceState::inspect(handle) {
                Ok(device) => device,
                Err(error) => {
                    self.rejected_handles.insert(key);
                    tracing::debug!(handle = key, %error, "ignoring unsupported Raw Input HID");
                    return;
                }
            };

            if self.devices.values().any(|known| known.stable_id == device.stable_id) {
                device.stable_id = format!(
                    "{}:collection-{:016x}",
                    device.stable_id,
                    stable_hash64(device.device_path.as_bytes())
                );
            }
            device.device_id = gamepad_device_id_from_stable_id(&device.stable_id);
            let known = self.known_devices.entry(device.stable_id.clone()).or_insert_with(|| {
                let backend_id = self.next_backend_id;
                self.next_backend_id = self.next_backend_id.saturating_add(1);
                KnownDevice {
                    stable_id: device.stable_id.clone(),
                    backend_id,
                    device_id: device.device_id,
                    name: device.name.clone(),
                    is_connected: true,
                }
            });
            known.name.clone_from(&device.name);
            known.device_id = device.device_id;
            known.is_connected = true;
            tracing::info!(
                device = %device.stable_id,
                name = %device.name,
                buttons = device.button_count,
                axes = device.axis_count,
                "Raw Input controller connected"
            );
            self.devices.insert(key, device);
        }

        fn process_reports(
            &mut self,
            handle: HANDLE,
            reports: &[Vec<u8>],
            timestamp: DeviceTimestamp,
        ) {
            self.ensure_device(handle);
            let key = handle as usize;
            let Some(device) = self.devices.get_mut(&key) else { return };
            let mut decoded = Vec::new();
            for report in reports {
                match device.decode_report(report, timestamp) {
                    Ok(events) => {
                        if !events.is_empty() {
                            tracing::trace!(
                                device = %device.stable_id,
                                decoded_events = events.len(),
                                "Raw Input HID report decoded"
                            );
                        }
                        decoded.extend(events);
                    }
                    Err(error) => tracing::warn!(
                        device = %device.stable_id,
                        %error,
                        "failed to decode Raw Input HID report"
                    ),
                }
            }
            for event in decoded {
                self.push_event(event);
            }
        }

        fn remove_device(&mut self, handle: HANDLE, timestamp: DeviceTimestamp) {
            let key = handle as usize;
            self.rejected_handles.remove(&key);
            let Some(mut device) = self.devices.remove(&key) else { return };
            for event in device.release_events(timestamp) {
                self.push_event(event);
            }
            self.push_event(QueuedHidEvent::Disconnected {
                device_id: device.device_id,
                timestamp,
            });
            if let Some(known) = self.known_devices.get_mut(&device.stable_id) {
                known.is_connected = false;
            }
            tracing::info!(device = %device.stable_id, "Raw Input controller disconnected");
        }

        fn push_event(&mut self, event: QueuedHidEvent) {
            if self.events.len() >= MAX_QUEUED_EVENTS {
                self.events.pop_front();
                self.dropped_event_count = self.dropped_event_count.saturating_add(1);
                if self.dropped_event_count == 1 || self.dropped_event_count.is_multiple_of(1_000) {
                    tracing::warn!(
                        dropped_event_count = self.dropped_event_count,
                        "Raw Input event queue overflow"
                    );
                }
            }
            self.events.push_back(event);
        }
    }

    struct KnownDevice {
        stable_id: String,
        backend_id: u32,
        device_id: DeviceId,
        name: String,
        is_connected: bool,
    }

    impl KnownDevice {
        fn connected_gamepad(&self) -> ConnectedGamepad {
            ConnectedGamepad {
                stable_id: self.stable_id.clone(),
                backend_id: self.backend_id,
                device_id: self.device_id,
                name: self.name.clone(),
                is_connected: self.is_connected,
            }
        }
    }

    struct HidDeviceState {
        stable_id: String,
        device_path: String,
        device_id: DeviceId,
        name: String,
        preparsed_data: AlignedBuffer,
        controls: BTreeMap<u16, ControlDescriptor>,
        pressed_buttons: HashSet<u16>,
        axis_values: HashMap<u16, u32>,
        input_data_count: u32,
        uses_report_ids: bool,
        button_count: usize,
        axis_count: usize,
    }

    impl HidDeviceState {
        fn inspect(handle: HANDLE) -> Result<Self> {
            let info = raw_input_device_info(handle)?;
            if !is_supported_usage(info.usUsagePage, info.usUsage) {
                bail!("unsupported HID usage {:04x}:{:04x}", info.usUsagePage, info.usUsage);
            }
            let device_path = raw_input_device_name(handle)?;
            let preparsed_data = raw_input_preparsed_data(handle)?;
            let preparsed = preparsed_data.as_preparsed();
            let mut caps = HIDP_CAPS::default();
            // SAFETY: preparsed points to the complete RIDI_PREPARSEDDATA buffer and caps is writable.
            let status = unsafe { HidP_GetCaps(preparsed, &mut caps) };
            if status != HIDP_STATUS_SUCCESS {
                bail!("HidP_GetCaps failed with status 0x{status:08x}");
            }
            let controls = control_descriptors(preparsed, &caps)?;
            if controls.is_empty() {
                bail!("HID has no input buttons or values");
            }
            // SAFETY: preparsed belongs to this device and remains valid for the lifetime of self.
            let input_data_count = unsafe { HidP_MaxDataListLength(HidP_Input, preparsed) };
            if input_data_count == 0 {
                bail!("HID has no decodable input data");
            }
            let button_count = controls.values().filter(|control| control.is_button()).count();
            let axis_count = controls.len().saturating_sub(button_count);
            let uses_report_ids = controls.values().any(|control| control.report_id != 0);
            let stable_id = stable_id_for_device(
                &device_path,
                info.usUsagePage,
                info.usUsage,
                container_id_for_interface(&device_path),
            );
            let name = format!(
                "Raw HID {:04X}:{:04X}",
                info.dwVendorId & 0xffff,
                info.dwProductId & 0xffff
            );
            Ok(Self {
                stable_id,
                device_path,
                device_id: DeviceId(0),
                name,
                preparsed_data,
                controls,
                pressed_buttons: HashSet::new(),
                axis_values: HashMap::new(),
                input_data_count,
                uses_report_ids,
                button_count,
                axis_count,
            })
        }

        fn decode_report(
            &mut self,
            report: &[u8],
            timestamp: DeviceTimestamp,
        ) -> Result<Vec<QueuedHidEvent>> {
            if report.is_empty() {
                return Ok(Vec::new());
            }
            let mut data = vec![HIDP_DATA::default(); self.input_data_count as usize];
            let mut data_count = self.input_data_count;
            // SAFETY: buffers cover data_count entries and the complete report. The preparsed
            // pointer stays valid because its backing allocation is owned by self.
            let status = unsafe {
                HidP_GetData(
                    HidP_Input,
                    data.as_mut_ptr(),
                    &mut data_count,
                    self.preparsed_data.as_preparsed(),
                    report.as_ptr().cast_mut(),
                    report.len() as u32,
                )
            };
            if status != HIDP_STATUS_SUCCESS {
                bail!("HidP_GetData failed with status 0x{status:08x}");
            }
            data.truncate(data_count as usize);
            let report_id = if self.uses_report_ids { report[0] } else { 0 };
            let mut values = HashMap::with_capacity(data.len());
            for item in data {
                values.insert(item.DataIndex, item);
            }

            let mut events = Vec::new();
            for (&data_index, control) in &self.controls {
                if control.report_id != report_id {
                    continue;
                }
                match control.kind {
                    ControlKind::Button => {
                        // SAFETY: button descriptors make the HIDP_DATA union's On member active.
                        let pressed = values
                            .get(&data_index)
                            .is_some_and(|data| unsafe { data.Anonymous.On });
                        let was_pressed = self.pressed_buttons.contains(&data_index);
                        if pressed == was_pressed {
                            continue;
                        }
                        if pressed {
                            self.pressed_buttons.insert(data_index);
                        } else {
                            self.pressed_buttons.remove(&data_index);
                        }
                        events.push(control.button_event(self.device_id, pressed, timestamp));
                    }
                    ControlKind::Axis { logical_min, logical_max, bit_size } => {
                        let Some(data) = values.get(&data_index) else { continue };
                        // SAFETY: value descriptors make the HIDP_DATA union's RawValue member active.
                        let raw_value = unsafe { data.Anonymous.RawValue };
                        if self.axis_values.insert(data_index, raw_value) == Some(raw_value) {
                            continue;
                        }
                        let Some(value) =
                            normalize_axis_value(raw_value, logical_min, logical_max, bit_size)
                        else {
                            continue;
                        };
                        events.push(control.axis_event(self.device_id, value, timestamp));
                    }
                }
            }
            Ok(events)
        }

        fn release_events(&mut self, timestamp: DeviceTimestamp) -> Vec<QueuedHidEvent> {
            let pressed = std::mem::take(&mut self.pressed_buttons);
            pressed
                .into_iter()
                .filter_map(|data_index| self.controls.get(&data_index))
                .map(|control| control.button_event(self.device_id, false, timestamp))
                .collect()
        }

        fn pressed_button_snapshot(&self) -> Vec<GamepadPressedButton> {
            self.pressed_buttons
                .iter()
                .filter_map(|data_index| self.controls.get(data_index))
                .map(|control| GamepadPressedButton {
                    name: control.name.clone(),
                    device_id: self.device_id,
                })
                .collect()
        }
    }

    #[derive(Clone)]
    struct ControlDescriptor {
        kind: ControlKind,
        report_id: u8,
        name: String,
        logical: String,
        raw_code: RawControlCode,
    }

    #[derive(Clone, Copy)]
    enum ControlKind {
        Button,
        Axis { logical_min: i32, logical_max: i32, bit_size: u16 },
    }

    impl ControlDescriptor {
        fn is_button(&self) -> bool {
            matches!(self.kind, ControlKind::Button)
        }

        fn button_event(
            &self,
            device_id: DeviceId,
            pressed: bool,
            timestamp: DeviceTimestamp,
        ) -> QueuedHidEvent {
            QueuedHidEvent::Button {
                device_id,
                name: self.name.clone(),
                logical: self.logical.clone(),
                raw_code: self.raw_code.clone(),
                pressed,
                timestamp,
            }
        }

        fn axis_event(
            &self,
            device_id: DeviceId,
            value: f32,
            timestamp: DeviceTimestamp,
        ) -> QueuedHidEvent {
            QueuedHidEvent::Axis {
                device_id,
                name: self.name.clone(),
                logical: self.logical.clone(),
                raw_code: self.raw_code.clone(),
                value,
                timestamp,
            }
        }
    }

    enum QueuedHidEvent {
        Button {
            device_id: DeviceId,
            name: String,
            logical: String,
            raw_code: RawControlCode,
            pressed: bool,
            timestamp: DeviceTimestamp,
        },
        Axis {
            device_id: DeviceId,
            name: String,
            logical: String,
            raw_code: RawControlCode,
            value: f32,
            timestamp: DeviceTimestamp,
        },
        Disconnected {
            device_id: DeviceId,
            timestamp: DeviceTimestamp,
        },
    }

    struct AlignedBuffer {
        words: Vec<usize>,
        byte_len: usize,
    }

    impl AlignedBuffer {
        fn new(byte_len: usize) -> Self {
            let word_count = byte_len.div_ceil(size_of::<usize>());
            Self { words: vec![0; word_count], byte_len }
        }

        fn as_mut_bytes(&mut self) -> *mut c_void {
            self.words.as_mut_ptr().cast()
        }

        fn as_preparsed(&self) -> PHIDP_PREPARSED_DATA {
            self.words.as_ptr() as PHIDP_PREPARSED_DATA
        }
    }

    fn register_usages(hwnd: HWND, flags: u32) -> Result<()> {
        let devices = REGISTERED_USAGES.map(|usage| RAWINPUTDEVICE {
            usUsagePage: GENERIC_DESKTOP_USAGE_PAGE,
            usUsage: usage,
            dwFlags: flags,
            hwndTarget: hwnd,
        });
        // SAFETY: devices is a complete array of RAWINPUTDEVICE values for this call.
        let registered = unsafe {
            RegisterRawInputDevices(
                devices.as_ptr(),
                devices.len() as u32,
                size_of::<RAWINPUTDEVICE>() as u32,
            )
        };
        if registered == 0 {
            // SAFETY: GetLastError has no preconditions and is read immediately after failure.
            let error = unsafe { GetLastError() };
            bail!("RegisterRawInputDevices failed with Win32 error {error}");
        }
        Ok(())
    }

    fn enumerate_supported_devices() -> Result<Vec<HANDLE>> {
        let mut count = 0;
        // SAFETY: null buffer requests the current device count.
        let first = unsafe {
            GetRawInputDeviceList(
                std::ptr::null_mut(),
                &mut count,
                size_of::<RAWINPUTDEVICELIST>() as u32,
            )
        };
        if first == RAW_INPUT_ERROR {
            bail!("GetRawInputDeviceList count query failed");
        }
        let mut devices = vec![RAWINPUTDEVICELIST::default(); count as usize];
        // SAFETY: devices has capacity for count entries and count is writable.
        let read = unsafe {
            GetRawInputDeviceList(
                devices.as_mut_ptr(),
                &mut count,
                size_of::<RAWINPUTDEVICELIST>() as u32,
            )
        };
        if read == RAW_INPUT_ERROR {
            bail!("GetRawInputDeviceList failed");
        }
        devices.truncate(read as usize);
        Ok(devices
            .into_iter()
            .filter(|device| device.dwType == RIM_TYPEHID)
            .filter_map(|device| {
                raw_input_device_info(device.hDevice)
                    .ok()
                    .filter(|info| is_supported_usage(info.usUsagePage, info.usUsage))
                    .map(|_| device.hDevice)
            })
            .collect())
    }

    fn is_supported_usage(usage_page: u16, usage: u16) -> bool {
        usage_page == GENERIC_DESKTOP_USAGE_PAGE && REGISTERED_USAGES.contains(&usage)
    }

    fn read_raw_input_reports(raw_input: *mut c_void) -> Option<(HANDLE, Vec<Vec<u8>>)> {
        let header_size = size_of::<RAWINPUTHEADER>() as u32;
        let mut header = RAWINPUTHEADER::default();
        let mut header_bytes = header_size;
        // SAFETY: header is writable for header_bytes and raw_input comes from WM_INPUT lParam.
        let read = unsafe {
            GetRawInputData(
                raw_input,
                windows_sys::Win32::UI::Input::RID_HEADER,
                (&mut header as *mut RAWINPUTHEADER).cast(),
                &mut header_bytes,
                header_size,
            )
        };
        if read != header_size || header.dwType != RIM_TYPEHID {
            return None;
        }

        let mut byte_len = 0;
        // SAFETY: null buffer requests the packet size for this HRAWINPUT.
        let size_result = unsafe {
            GetRawInputData(raw_input, RID_INPUT, std::ptr::null_mut(), &mut byte_len, header_size)
        };
        if size_result == RAW_INPUT_ERROR || byte_len < raw_hid_data_offset() as u32 {
            return None;
        }
        let mut buffer = AlignedBuffer::new(byte_len as usize);
        let mut actual_len = byte_len;
        // SAFETY: the aligned buffer contains at least actual_len writable bytes.
        let read = unsafe {
            GetRawInputData(
                raw_input,
                RID_INPUT,
                buffer.as_mut_bytes(),
                &mut actual_len,
                header_size,
            )
        };
        if read == RAW_INPUT_ERROR
            || read < raw_hid_data_offset() as u32
            || read > buffer.byte_len as u32
        {
            return None;
        }
        // SAFETY: GetRawInputData wrote `read` bytes into the aligned allocation.
        let packet =
            unsafe { slice::from_raw_parts(buffer.words.as_ptr().cast::<u8>(), read as usize) };
        Some((header.hDevice, split_raw_hid_reports(packet)?))
    }

    fn split_raw_hid_reports(packet: &[u8]) -> Option<Vec<Vec<u8>>> {
        let hid_offset = std::mem::offset_of!(RAWINPUT, data);
        let report_size =
            read_packet_u32(packet, hid_offset + std::mem::offset_of!(RAWHID, dwSizeHid))? as usize;
        let report_count =
            read_packet_u32(packet, hid_offset + std::mem::offset_of!(RAWHID, dwCount))? as usize;
        let data_offset = raw_hid_data_offset();
        let data_len = report_size.checked_mul(report_count)?;
        let data = packet.get(data_offset..data_offset.checked_add(data_len)?)?;
        (report_size != 0).then(|| data.chunks_exact(report_size).map(<[u8]>::to_vec).collect())
    }

    fn raw_hid_data_offset() -> usize {
        std::mem::offset_of!(RAWINPUT, data) + std::mem::offset_of!(RAWHID, bRawData)
    }

    fn read_packet_u32(packet: &[u8], offset: usize) -> Option<u32> {
        Some(u32::from_ne_bytes(packet.get(offset..offset.checked_add(4)?)?.try_into().ok()?))
    }

    fn raw_input_device_info(handle: HANDLE) -> Result<RID_DEVICE_INFO_HID> {
        let mut info = RID_DEVICE_INFO::default();
        info.cbSize = size_of_val(&info) as u32;
        let mut size = size_of_val(&info) as u32;
        // SAFETY: info is initialized with cbSize and writable for size bytes.
        let result = unsafe {
            GetRawInputDeviceInfoW(
                handle,
                RIDI_DEVICEINFO,
                (&mut info as *mut RID_DEVICE_INFO).cast(),
                &mut size,
            )
        };
        if result == RAW_INPUT_ERROR || info.dwType != RIM_TYPEHID {
            bail!("GetRawInputDeviceInfoW(RIDI_DEVICEINFO) failed");
        }
        // SAFETY: dwType was checked as RIM_TYPEHID, making the hid union member active.
        Ok(unsafe { info.Anonymous.hid })
    }

    fn raw_input_device_name(handle: HANDLE) -> Result<String> {
        let mut chars = 0;
        // SAFETY: null buffer requests the required UTF-16 character count.
        let first = unsafe {
            GetRawInputDeviceInfoW(handle, RIDI_DEVICENAME, std::ptr::null_mut(), &mut chars)
        };
        if first == RAW_INPUT_ERROR || chars == 0 {
            bail!("GetRawInputDeviceInfoW(RIDI_DEVICENAME) size query failed");
        }
        let mut name = vec![0u16; chars as usize + 1];
        let mut available = name.len() as u32;
        // SAFETY: name is writable for available UTF-16 code units.
        let read = unsafe {
            GetRawInputDeviceInfoW(
                handle,
                RIDI_DEVICENAME,
                name.as_mut_ptr().cast(),
                &mut available,
            )
        };
        if read == RAW_INPUT_ERROR {
            bail!("GetRawInputDeviceInfoW(RIDI_DEVICENAME) failed");
        }
        name.truncate(read as usize);
        while name.last() == Some(&0) {
            name.pop();
        }
        String::from_utf16(&name).context("Raw Input device name is not valid UTF-16")
    }

    fn raw_input_preparsed_data(handle: HANDLE) -> Result<AlignedBuffer> {
        let mut byte_len = 0;
        // SAFETY: null buffer requests the preparsed-data size.
        let first = unsafe {
            GetRawInputDeviceInfoW(handle, RIDI_PREPARSEDDATA, std::ptr::null_mut(), &mut byte_len)
        };
        if first == RAW_INPUT_ERROR || byte_len == 0 {
            bail!("GetRawInputDeviceInfoW(RIDI_PREPARSEDDATA) size query failed");
        }
        let mut data = AlignedBuffer::new(byte_len as usize);
        let mut available = byte_len;
        // SAFETY: the aligned allocation is writable for available bytes.
        let read = unsafe {
            GetRawInputDeviceInfoW(handle, RIDI_PREPARSEDDATA, data.as_mut_bytes(), &mut available)
        };
        if read == RAW_INPUT_ERROR {
            bail!("GetRawInputDeviceInfoW(RIDI_PREPARSEDDATA) failed");
        }
        Ok(data)
    }

    fn control_descriptors(
        preparsed: PHIDP_PREPARSED_DATA,
        caps: &HIDP_CAPS,
    ) -> Result<BTreeMap<u16, ControlDescriptor>> {
        let mut controls = BTreeMap::new();
        let mut button_caps =
            vec![HIDP_BUTTON_CAPS::default(); caps.NumberInputButtonCaps as usize];
        let mut button_count = caps.NumberInputButtonCaps;
        if button_count > 0 {
            // SAFETY: button_caps has button_count writable entries and preparsed is valid.
            let status = unsafe {
                HidP_GetButtonCaps(
                    HidP_Input,
                    button_caps.as_mut_ptr(),
                    &mut button_count,
                    preparsed,
                )
            };
            if status != HIDP_STATUS_SUCCESS {
                bail!("HidP_GetButtonCaps failed with status 0x{status:08x}");
            }
        }
        button_caps.truncate(button_count as usize);
        for cap in button_caps.into_iter().filter(|cap| !cap.IsAlias) {
            if cap.IsRange {
                // SAFETY: IsRange selects the Range union member.
                let range = unsafe { cap.Anonymous.Range };
                for data_index in range.DataIndexMin..=range.DataIndexMax {
                    let usage = range
                        .UsageMin
                        .saturating_add(data_index.saturating_sub(range.DataIndexMin));
                    controls.entry(data_index).or_insert_with(|| {
                        button_descriptor(data_index, cap.ReportID, cap.UsagePage, usage)
                    });
                }
            } else {
                // SAFETY: !IsRange selects the NotRange union member.
                let value = unsafe { cap.Anonymous.NotRange };
                controls.entry(value.DataIndex).or_insert_with(|| {
                    button_descriptor(value.DataIndex, cap.ReportID, cap.UsagePage, value.Usage)
                });
            }
        }

        let mut value_caps = vec![HIDP_VALUE_CAPS::default(); caps.NumberInputValueCaps as usize];
        let mut value_count = caps.NumberInputValueCaps;
        if value_count > 0 {
            // SAFETY: value_caps has value_count writable entries and preparsed is valid.
            let status = unsafe {
                HidP_GetValueCaps(HidP_Input, value_caps.as_mut_ptr(), &mut value_count, preparsed)
            };
            if status != HIDP_STATUS_SUCCESS {
                bail!("HidP_GetValueCaps failed with status 0x{status:08x}");
            }
        }
        value_caps.truncate(value_count as usize);
        for cap in value_caps.into_iter().filter(|cap| !cap.IsAlias) {
            if cap.IsRange {
                // SAFETY: IsRange selects the Range union member.
                let range = unsafe { cap.Anonymous.Range };
                for data_index in range.DataIndexMin..=range.DataIndexMax {
                    let usage = range
                        .UsageMin
                        .saturating_add(data_index.saturating_sub(range.DataIndexMin));
                    controls.entry(data_index).or_insert_with(|| {
                        axis_descriptor(data_index, cap.ReportID, cap.UsagePage, usage, &cap)
                    });
                }
            } else {
                // SAFETY: !IsRange selects the NotRange union member.
                let value = unsafe { cap.Anonymous.NotRange };
                controls.entry(value.DataIndex).or_insert_with(|| {
                    axis_descriptor(value.DataIndex, cap.ReportID, cap.UsagePage, value.Usage, &cap)
                });
            }
        }

        let mut button_number = 0u32;
        let mut axis_number = 0u32;
        for control in controls.values_mut() {
            if control.is_button() {
                button_number = button_number.saturating_add(1);
                control.name = format!("Button{button_number}");
            } else {
                axis_number = axis_number.saturating_add(1);
                control.name = format!("Axis{axis_number}");
            }
        }
        Ok(controls)
    }

    fn button_descriptor(
        data_index: u16,
        report_id: u8,
        usage_page: u16,
        usage: u16,
    ) -> ControlDescriptor {
        descriptor(ControlKind::Button, data_index, report_id, usage_page, usage)
    }

    fn axis_descriptor(
        data_index: u16,
        report_id: u8,
        usage_page: u16,
        usage: u16,
        caps: &HIDP_VALUE_CAPS,
    ) -> ControlDescriptor {
        descriptor(
            ControlKind::Axis {
                logical_min: caps.LogicalMin,
                logical_max: caps.LogicalMax,
                bit_size: caps.BitSize,
            },
            data_index,
            report_id,
            usage_page,
            usage,
        )
    }

    fn descriptor(
        kind: ControlKind,
        data_index: u16,
        report_id: u8,
        usage_page: u16,
        usage: u16,
    ) -> ControlDescriptor {
        let logical = format!("Usage({usage_page:04X}:{usage:04X})");
        ControlDescriptor {
            kind,
            report_id,
            name: String::new(),
            logical: logical.clone(),
            raw_code: RawControlCode { value: u32::from(data_index), label: logical },
        }
    }

    fn normalize_axis_value(
        raw: u32,
        logical_min: i32,
        logical_max: i32,
        bit_size: u16,
    ) -> Option<f32> {
        let raw =
            if logical_min < 0 { i64::from(sign_extend(raw, bit_size)) } else { i64::from(raw) };
        let min = i64::from(logical_min);
        let max = if logical_min >= 0 && logical_max < 0 && bit_size < 32 {
            (1i64 << bit_size) - 1
        } else {
            i64::from(logical_max)
        };
        if max <= min {
            return None;
        }
        let unit = (raw.clamp(min, max) - min) as f64 / (max - min) as f64;
        Some((unit.mul_add(2.0, -1.0) as f32).clamp(-1.0, 1.0))
    }

    fn sign_extend(value: u32, bit_size: u16) -> i32 {
        if bit_size == 0 || bit_size >= 32 {
            return value as i32;
        }
        let shift = 32 - u32::from(bit_size);
        ((value << shift) as i32) >> shift
    }

    fn stable_id_for_device(
        device_path: &str,
        usage_page: u16,
        usage: u16,
        container_id: Option<GUID>,
    ) -> String {
        if let Some(container_id) = container_id.filter(|guid| {
            guid.data1 != 0
                || guid.data2 != 0
                || guid.data3 != 0
                || guid.data4.iter().any(|&v| v != 0)
        }) {
            return format!("rawinput:{}:{usage_page:04x}:{usage:04x}", format_guid(container_id));
        }
        format!(
            "rawinput:path-{:016x}:{usage_page:04x}:{usage:04x}",
            stable_hash64(device_path.to_ascii_lowercase().as_bytes())
        )
    }

    fn container_id_for_interface(device_path: &str) -> Option<GUID> {
        let path = device_path.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
        let mut property_type = 0;
        let mut container_id = GUID::from_u128(0);
        let mut byte_len = size_of::<GUID>() as u32;
        // SAFETY: path is a terminated device-interface path and all output buffers are writable.
        let result = unsafe {
            CM_Get_Device_Interface_PropertyW(
                path.as_ptr(),
                &DEVPKEY_Device_ContainerId,
                &mut property_type,
                (&mut container_id as *mut GUID).cast(),
                &mut byte_len,
                0,
            )
        };
        (result == CR_SUCCESS
            && property_type == DEVPROP_TYPE_GUID
            && byte_len as usize == size_of::<GUID>())
        .then_some(container_id)
    }

    fn format_guid(guid: GUID) -> String {
        format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            guid.data1,
            guid.data2,
            guid.data3,
            guid.data4[0],
            guid.data4[1],
            guid.data4[2],
            guid.data4[3],
            guid.data4[4],
            guid.data4[5],
            guid.data4[6],
            guid.data4[7]
        )
    }

    fn stable_hash64(bytes: &[u8]) -> u64 {
        bytes.iter().fold(14_695_981_039_346_656_037u64, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(1_099_511_628_211)
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn normalizes_unsigned_axis_to_bmz_range() {
            assert_eq!(normalize_axis_value(0, 0, 65_535, 16), Some(-1.0));
            assert!((normalize_axis_value(32_768, 0, 65_535, 16).unwrap()).abs() < 0.0001);
            assert_eq!(normalize_axis_value(65_535, 0, 65_535, 16), Some(1.0));
        }

        #[test]
        fn sign_extends_signed_hid_values() {
            assert_eq!(sign_extend(0x81, 8), -127);
            assert_eq!(sign_extend(0x7f, 8), 127);
            assert_eq!(normalize_axis_value(0x81, -127, 127, 8), Some(-1.0));
            assert_eq!(normalize_axis_value(0x7f, -127, 127, 8), Some(1.0));
        }

        #[test]
        fn stable_id_prefers_physical_container() {
            let guid = GUID::from_u128(0x12345678_1234_5678_90ab_cdef01234567);
            assert_eq!(
                stable_id_for_device("ignored", 1, 5, Some(guid)),
                "rawinput:12345678-1234-5678-90ab-cdef01234567:0001:0005"
            );
        }

        #[test]
        fn stable_id_path_fallback_is_case_insensitive() {
            assert_eq!(
                stable_id_for_device(r"\\?\HID#VID_1234", 1, 4, None),
                stable_id_for_device(r"\\?\hid#vid_1234", 1, 4, None)
            );
        }

        #[test]
        fn poll_exposes_current_pressed_button_snapshot() {
            let bridge = RawInputBridge::new();
            let mut button = button_descriptor(7, 0, 0x09, 0x09);
            button.name = "Button9".to_string();
            let mut controls = BTreeMap::new();
            controls.insert(7, button);
            let device_id = DeviceId(42);
            bridge.shared.lock().unwrap().devices.insert(
                1,
                HidDeviceState {
                    stable_id: "rawinput:test".to_string(),
                    device_path: "test".to_string(),
                    device_id,
                    name: "test controller".to_string(),
                    preparsed_data: AlignedBuffer::new(0),
                    controls,
                    pressed_buttons: HashSet::from([7]),
                    axis_values: HashMap::new(),
                    input_data_count: 1,
                    uses_report_ids: false,
                    button_count: 1,
                    axis_count: 0,
                },
            );
            let mut backend = RawInputBackend::new(bridge, [GamepadScratchConfig::default(); 2]);

            assert_eq!(
                backend.poll().pressed_buttons,
                Some(vec![GamepadPressedButton { name: "Button9".to_string(), device_id }])
            );
        }

        #[test]
        fn splits_variable_length_raw_hid_packet() {
            let hid_offset = std::mem::offset_of!(RAWINPUT, data);
            let size_offset = hid_offset + std::mem::offset_of!(RAWHID, dwSizeHid);
            let count_offset = hid_offset + std::mem::offset_of!(RAWHID, dwCount);
            let data_offset = raw_hid_data_offset();
            let mut packet = vec![0; data_offset + 6];
            packet[size_offset..size_offset + 4].copy_from_slice(&3u32.to_ne_bytes());
            packet[count_offset..count_offset + 4].copy_from_slice(&2u32.to_ne_bytes());
            packet[data_offset..].copy_from_slice(&[1, 2, 3, 4, 5, 6]);

            assert_eq!(split_raw_hid_reports(&packet), Some(vec![vec![1, 2, 3], vec![4, 5, 6]]));
        }

        #[test]
        fn rejects_truncated_raw_hid_packet() {
            let hid_offset = std::mem::offset_of!(RAWINPUT, data);
            let size_offset = hid_offset + std::mem::offset_of!(RAWHID, dwSizeHid);
            let count_offset = hid_offset + std::mem::offset_of!(RAWHID, dwCount);
            let data_offset = raw_hid_data_offset();
            let mut packet = vec![0; data_offset + 5];
            packet[size_offset..size_offset + 4].copy_from_slice(&3u32.to_ne_bytes());
            packet[count_offset..count_offset + 4].copy_from_slice(&2u32.to_ne_bytes());

            assert_eq!(split_raw_hid_reports(&packet), None);
        }
    }
}

#[cfg(windows)]
pub use windows::{RawInputBackend, RawInputBridge};

#[cfg(not(windows))]
#[derive(Clone, Default)]
pub struct RawInputBridge;

#[cfg(not(windows))]
impl RawInputBridge {
    pub fn new() -> Self {
        Self
    }
}
