// #![windows_subsystem = "windows"]
#![allow(dead_code)]

pub mod chart;
pub mod scene;
pub mod system;

use chart::{
    bms_loader::load_bms,
    db::{clear_cache, init_songs_db, load_folder},
    player::Player,
};
use kira::{backend::cpal::CpalBackend, *};
use macroquad::prelude::*;
use scene::{Scene, play::Play};

#[macroquad::main(window_conf)]
async fn main() {
    let mut audio_manager =
        AudioManager::new(AudioManagerSettings::<CpalBackend>::default()).unwrap();

    let connection = init_songs_db().unwrap();
    if std::env::args().any(|arg| arg == "--clear") {
        info!("Clear song cache");
        clear_cache(&connection).ok();
    }

    if std::env::args().any(|arg| arg == "-l" || arg == "--load") {
        info!("Loading songs from folder");
        load_folder(&connection, "assets/songs").ok();
    }

    let filename = "assets/songs/BOFU2017/Cagliostro_1011/_Cagliostro_7A.bml";
    let chart = load_bms(filename, false).unwrap();

    let mut player = Player::default();
    player.init(chart);

    let mut scene = Play::new(player).await;
    scene.init();

    loop {
        scene.update(&mut audio_manager);

        scene.render();

        next_frame().await;
        clear_background(BLACK);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "CrossChronox".to_string(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}
