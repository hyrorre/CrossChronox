//! Windows Raw Input message pump. All HWND and message operations stay on the
//! creating thread. Keyboard timestamps are assigned here, before render work.
use super::{capture::InputRoute, rawinput::RawInputBridge};
use anyhow::{Result, bail};
use bmz_gameplay::input::backend::{DeviceTimestamp, monotonic_timestamp_ns};
use std::{collections::HashSet, mem::size_of, ptr};
use windows_sys::Win32::{
    Foundation::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::{KeyboardAndMouse::*, *},
        WindowsAndMessaging::*,
    },
};
use winit::{keyboard::PhysicalKey, platform::scancode::PhysicalKeyExtScancode};

pub struct NativeCapture {
    hwnd: HWND,
    owner: usize,
    bridge: Option<RawInputBridge>,
    hid: bool,
    pressed: HashSet<PhysicalKey>,
    input: Option<super::shared::SharedInputBackend>,
}

impl NativeCapture {
    pub fn new(bridge: Option<RawInputBridge>, hid: bool) -> Result<Self> {
        let class: Vec<u16> = "BMZInputCapture".encode_utf16().chain(Some(0)).collect();
        // SAFETY: buffers live for each Win32 call. The class procedure never
        // touches Rust state; the message pump handles input before dispatch.
        let hwnd = unsafe {
            let instance = GetModuleHandleW(ptr::null());
            let definition = WNDCLASSW {
                lpfnWndProc: Some(DefWindowProcW),
                hInstance: instance,
                lpszClassName: class.as_ptr(),
                ..std::mem::zeroed()
            };
            RegisterClassW(&definition);
            CreateWindowExW(
                0,
                class.as_ptr(),
                class.as_ptr(),
                0,
                0,
                0,
                0,
                0,
                HWND_MESSAGE,
                ptr::null_mut(),
                instance,
                ptr::null(),
            )
        };
        if hwnd.is_null() {
            bail!("failed to create input message window: {}", unsafe { GetLastError() });
        }
        Ok(Self { hwnd, owner: 0, bridge, hid, pressed: HashSet::new(), input: None })
    }

    pub fn attach(&mut self, owner: usize) -> Result<()> {
        if owner == 0 {
            bail!("input owner is not attached");
        }
        if owner == self.owner {
            return Ok(());
        }
        let keyboard = RAWINPUTDEVICE {
            usUsagePage: 1,
            usUsage: 6,
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: self.hwnd,
        };
        if unsafe { RegisterRawInputDevices(&keyboard, 1, size_of::<RAWINPUTDEVICE>() as u32) } == 0
        {
            bail!("failed to register keyboard Raw Input");
        }
        if self.hid
            && let Some(bridge) = &self.bridge
        {
            bridge.register_capture_window(self.hwnd)?;
        }
        self.owner = owner;
        Ok(())
    }

    pub fn poll(&mut self, route: Option<&InputRoute>) {
        let same = match (&self.input, route) {
            (Some(input), Some(route)) => input.same_source(&route.input),
            (None, None) => true,
            _ => false,
        };
        if !same {
            self.pressed.clear();
            self.input = route.map(|route| route.input.clone());
        }
        let active = route.filter(|route| {
            route.focused
                && route.keyboard_enabled
                && super::capture::foreground_matches(self.owner)
        });
        if active.is_none() && !self.pressed.is_empty() {
            if let Some(route) = route {
                for key in self.pressed.drain() {
                    if let Some(event) = super::winit::physical_key_to_device_input(
                        key,
                        winit::event::ElementState::Released,
                        false,
                    ) {
                        route.input.push_shared_event(event);
                    }
                }
            } else {
                self.pressed.clear();
            }
        }
        let mut message: MSG = unsafe { std::mem::zeroed() };
        while unsafe { PeekMessageW(&mut message, self.hwnd, 0, 0, PM_REMOVE) } != 0 {
            if let Some(bridge) = &self.bridge
                && self.hid
            {
                bridge.handle_message((&message as *const MSG).cast());
            }
            if message.message == WM_INPUT {
                let timestamp = DeviceTimestamp::MonotonicNs(monotonic_timestamp_ns());
                let mut raw: RAWINPUT = unsafe { std::mem::zeroed() };
                let mut size = size_of::<RAWINPUT>() as u32;
                let read = unsafe {
                    GetRawInputData(
                        message.lParam as _,
                        RID_INPUT,
                        (&mut raw as *mut RAWINPUT).cast(),
                        &mut size,
                        size_of::<RAWINPUTHEADER>() as u32,
                    )
                };
                if read != u32::MAX
                    && raw.header.dwType == RIM_TYPEKEYBOARD
                    && let Some(route) = active
                {
                    let key = unsafe { raw.data.keyboard };
                    let extension = if key.Flags & RI_KEY_E0 as u16 != 0 {
                        0xe000
                    } else if key.Flags & RI_KEY_E1 as u16 != 0 {
                        0xe100
                    } else {
                        0
                    };
                    let scancode = if key.MakeCode == 0 {
                        unsafe { MapVirtualKeyW(u32::from(key.VKey), MAPVK_VK_TO_VSC_EX) as u16 }
                    } else {
                        key.MakeCode | extension
                    };
                    if matches!(scancode, 0xe11d | 0xe02a) {
                        unsafe {
                            DispatchMessageW(&message);
                        }
                        continue;
                    }
                    let physical = if key.VKey == VK_NUMLOCK {
                        PhysicalKey::Code(winit::keyboard::KeyCode::NumLock)
                    } else {
                        PhysicalKey::from_scancode(u32::from(scancode))
                    };
                    if key.VKey == VK_SHIFT && matches!(scancode, 0x47..=0x53) {
                        unsafe {
                            DispatchMessageW(&message);
                        }
                        continue;
                    }
                    let pressed = key.Flags & RI_KEY_BREAK as u16 == 0;
                    let changed = if pressed {
                        self.pressed.insert(physical)
                    } else {
                        self.pressed.remove(&physical)
                    };
                    if changed
                        && let Some(mut event) = super::winit::physical_key_to_device_input(
                            physical,
                            if pressed {
                                winit::event::ElementState::Pressed
                            } else {
                                winit::event::ElementState::Released
                            },
                            false,
                        )
                    {
                        event.timestamp = timestamp;
                        route.input.push_shared_event(event);
                    }
                }
            }
            unsafe {
                DispatchMessageW(&message);
            }
        }
    }
}

impl Drop for NativeCapture {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.hwnd);
        }
    }
}

pub fn wait_for_input() {
    // Wake on WM_INPUT immediately, with a 1ms bound for gamepad polling and
    // analog stop detection. This wait consumes no CPU while idle.
    unsafe {
        MsgWaitForMultipleObjectsEx(0, ptr::null(), 1, QS_ALLINPUT, MWMO_INPUTAVAILABLE);
    }
}
