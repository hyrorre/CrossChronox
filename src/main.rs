#![windows_subsystem = "windows"]
#![allow(dead_code)]

pub mod chart;
pub mod scene;
pub mod system;

use scene::play::Play;
use scene::select::Select;
use scene::{Scene, State};
use sdl3::render::{TextureCreator, WindowCanvas};
use sdl3::ttf::Sdl3TtfContext;
use sdl3::video::{Window, WindowContext};
use sdl3::{Sdl, VideoSubsystem};
use system::config::Config;
use system::input::InputManager;
use system::time::TimeManager;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::messagebox::{MessageBoxFlag, show_simple_message_box};
use std::time::Duration;

pub struct App {
    pub config: Config,
    pub input_manager: InputManager,
    pub time_manager: TimeManager,
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub ttf_context: Sdl3TtfContext,
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>,
}

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
    let ttf_context = match sdl3::ttf::init() {
        Ok(ok) => ok,
        Err(e) => {
            show_simple_message_box(
                MessageBoxFlag::ERROR,
                "TTF Init Error",
                &e.to_string(),
                None,
            )
            .unwrap();
            panic!("TTF Init Error: {}", e);
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

    let canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    let mut app = App {
        config: Config::new(),
        input_manager: InputManager::new(),
        time_manager: TimeManager::new(),
        sdl_context,
        video_subsystem,
        ttf_context,
        canvas,
        texture_creator,
    };
    let mut scene = Box::new(Play::new(&mut app)) as Box<dyn Scene>;

    'running: loop {
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
        let state = scene.update();
        scene.render();

        // Handle state changes if necessary
        if matches!(state, State::Finish) {
            break 'running;
        }

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
