#![windows_subsystem = "windows"]
#![allow(dead_code)]

use sdl3::event::Event;
use sdl3::image::LoadTexture;
use sdl3::keyboard::Keycode;
use sdl3::messagebox::{MessageBoxFlag, show_simple_message_box};
use sdl3::pixels::Color;
use std::time::Duration;

pub mod chart;
pub mod scene;
pub mod system;

pub fn main() {
    let sdl_context = match sdl3::init() {
        Ok(ok) => ok,
        Err(e) => {
            show_simple_message_box(MessageBoxFlag::ERROR, "Init Error", &e.to_string(), None)
                .unwrap();
            panic!("Init Error: {}", e);
        }
    };
    let video_subsystem = match sdl_context.video() {
        Ok(ok) => ok,
        Err(e) => {
            show_simple_message_box(
                MessageBoxFlag::ERROR,
                "Video Subsystem Error",
                &e.to_string(),
                None,
            )
            .unwrap();
            panic!("Video Subsystem Error: {}", e);
        }
    };

    let window = match video_subsystem
        .window("rust-sdl3 demo", 1280, 720)
        .position_centered()
        .build()
    {
        Ok(ok) => ok,
        Err(e) => {
            show_simple_message_box(
                MessageBoxFlag::ERROR,
                "Window Create Error",
                &e.to_string(),
                None,
            )
            .unwrap();
            panic!("Window Create Error: {}", e);
        }
    };

    let mut event_pump = match sdl_context.event_pump() {
        Ok(ok) => ok,
        Err(e) => {
            show_simple_message_box(
                MessageBoxFlag::ERROR,
                "Event Pump Error",
                &e.to_string(),
                &window,
            )
            .unwrap();
            panic!("Event Pump Error: {}", e)
        }
    };

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let texture_play = texture_creator
        .load_texture("CrossChronoxData/play.bmp")
        .ok();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        if let Some(ref texture) = texture_play {
            canvas.copy(texture, None, None).ok();
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let filename = "CrossChronoxData/Songs/BOFU2017/Cagliostro_1011/_Cagliostro_7A.bml";
        let chart = chart::bms_loader::load_bms(filename, false).unwrap();
        println!("{:#?}", chart.info);
    }
}
