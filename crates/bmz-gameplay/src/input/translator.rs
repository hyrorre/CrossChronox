use bmz_audio::clock::AudioClock;
use bmz_core::input::{InputDeviceKind, InputEvent, InputSource};
use bmz_core::time::TimeUs;

use crate::session::PlayOffsets;

use super::backend::{DeviceInputEvent, DeviceTimestamp, PhysicalControl};
use super::binding::LaneBinding;

pub struct InputTimingContext<'a> {
    pub audio_clock: &'a AudioClock,
    pub offsets: PlayOffsets,
    pub timestamp_anchor: Option<InputTimestampAnchor>,
}

#[derive(Debug, Clone, Copy)]
pub struct InputTimestampAnchor {
    pub monotonic_ns: u128,
    pub audio_time: TimeUs,
}

pub trait InputTranslator: Send {
    fn translate(
        &mut self,
        event: DeviceInputEvent,
        ctx: &InputTimingContext<'_>,
    ) -> Option<InputEvent>;
}

#[derive(Debug, Clone)]
pub struct DefaultInputTranslator {
    pub binding: LaneBinding,
}

impl InputTranslator for DefaultInputTranslator {
    fn translate(
        &mut self,
        event: DeviceInputEvent,
        ctx: &InputTimingContext<'_>,
    ) -> Option<InputEvent> {
        let binding = self.binding.resolve_entry(event.device, &event.control)?;
        let time = estimate_audio_time(&event.timestamp, ctx);
        let device_kind = device_kind_for_control(&event.control);
        Some(InputEvent {
            lane: binding.lane,
            kind: event.kind,
            time,
            source: InputSource::Human,
            device_kind,
            scratch_direction: binding.scratch_direction,
        })
    }
}

fn device_kind_for_control(control: &PhysicalControl) -> InputDeviceKind {
    match control {
        PhysicalControl::KeyboardKey(_) => InputDeviceKind::Keyboard,
        PhysicalControl::GamepadButton(_) | PhysicalControl::HidButton(_) => {
            InputDeviceKind::Controller
        }
    }
}

fn estimate_audio_time(timestamp: &DeviceTimestamp, ctx: &InputTimingContext<'_>) -> TimeUs {
    let base = match (*timestamp, ctx.timestamp_anchor) {
        (DeviceTimestamp::MonotonicNs(event_ns), Some(anchor)) => {
            let wall_delta_us = if event_ns >= anchor.monotonic_ns {
                u128_to_i64_saturating((event_ns - anchor.monotonic_ns) / 1_000)
            } else {
                -u128_to_i64_saturating((anchor.monotonic_ns - event_ns) / 1_000)
            };
            let chart_delta_us = scale_wall_delta_to_chart_time(
                wall_delta_us,
                ctx.audio_clock.playback_rate_percent(),
            );
            TimeUs(anchor.audio_time.0.saturating_add(chart_delta_us))
        }
        _ => ctx.audio_clock.now(),
    };
    TimeUs(base.0.saturating_add(ctx.offsets.input_offset_us))
}

fn u128_to_i64_saturating(value: u128) -> i64 {
    value.min(i64::MAX as u128) as i64
}

fn scale_wall_delta_to_chart_time(delta_us: i64, playback_rate_percent: u16) -> i64 {
    ((i128::from(delta_us) * i128::from(playback_rate_percent)) / 100)
        .clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}

pub fn keyboard_control(name: impl Into<String>) -> PhysicalControl {
    PhysicalControl::KeyboardKey(name.into())
}

#[cfg(test)]
mod tests {
    use bmz_audio::clock::AudioClock;
    use bmz_core::input::{InputDeviceKind, InputKind};
    use bmz_core::lane::Lane;

    use super::super::backend::{DeviceId, DeviceInputEvent};
    use super::super::binding::{BindingEntry, LaneBinding};
    use super::*;

    #[test]
    fn translator_uses_monotonic_timestamp_anchor() {
        let clock = AudioClock::stopped(48_000);
        let ctx = InputTimingContext {
            audio_clock: &clock,
            offsets: PlayOffsets { input_offset_us: 500, visual_offset_us: 0 },
            timestamp_anchor: Some(InputTimestampAnchor {
                monotonic_ns: 1_000_000,
                audio_time: TimeUs(10_000),
            }),
        };
        let mut translator = DefaultInputTranslator {
            binding: LaneBinding {
                entries: vec![BindingEntry {
                    device: None,
                    control: keyboard_control("Z"),
                    lane: Lane::Key1,
                    scratch_direction: None,
                }],
            },
        };

        let input = translator
            .translate(
                DeviceInputEvent {
                    device: DeviceId(1),
                    control: keyboard_control("Z"),
                    kind: InputKind::Press,
                    timestamp: DeviceTimestamp::MonotonicNs(1_250_000),
                    bounce_policy: Default::default(),
                },
                &ctx,
            )
            .unwrap();

        assert_eq!(input.time, TimeUs(10_750));
        assert_eq!(input.device_kind, InputDeviceKind::Keyboard);
    }

    #[test]
    fn timestamp_anchor_scales_wall_time_at_practice_playback_rates() {
        for (rate, expected_after, expected_before) in [
            (50, TimeUs(11_000), TimeUs(9_000)),
            (100, TimeUs(12_000), TimeUs(8_000)),
            (200, TimeUs(14_000), TimeUs(6_000)),
        ] {
            let mut clock = AudioClock::stopped(48_000);
            let _ = clock.set_playback_rate_percent(rate);
            let ctx = InputTimingContext {
                audio_clock: &clock,
                offsets: PlayOffsets { input_offset_us: 0, visual_offset_us: 0 },
                timestamp_anchor: Some(InputTimestampAnchor {
                    monotonic_ns: 10_000_000,
                    audio_time: TimeUs(10_000),
                }),
            };

            assert_eq!(
                estimate_audio_time(&DeviceTimestamp::MonotonicNs(12_000_000), &ctx),
                expected_after,
                "rate={rate}"
            );
            assert_eq!(
                estimate_audio_time(&DeviceTimestamp::MonotonicNs(8_000_000), &ctx),
                expected_before,
                "rate={rate}"
            );
        }
    }

    #[test]
    fn translator_marks_gamepad_controls_as_controller() {
        let clock = AudioClock::stopped(48_000);
        let ctx = InputTimingContext {
            audio_clock: &clock,
            offsets: PlayOffsets { input_offset_us: 0, visual_offset_us: 0 },
            timestamp_anchor: None,
        };
        let mut translator = DefaultInputTranslator {
            binding: LaneBinding {
                entries: vec![BindingEntry {
                    device: None,
                    control: PhysicalControl::GamepadButton("South".to_string()),
                    lane: Lane::Key1,
                    scratch_direction: None,
                }],
            },
        };

        let input = translator
            .translate(
                DeviceInputEvent {
                    device: DeviceId(1),
                    control: PhysicalControl::GamepadButton("South".to_string()),
                    kind: InputKind::Press,
                    timestamp: DeviceTimestamp::Unknown,
                    bounce_policy: Default::default(),
                },
                &ctx,
            )
            .unwrap();

        assert_eq!(input.device_kind, InputDeviceKind::Controller);
    }
}
