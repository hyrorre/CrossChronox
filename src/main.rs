#![windows_subsystem = "windows"]
#![allow(dead_code)]

pub mod chart;
pub mod state;
pub mod system;

use bevy::prelude::*;
use chart::player::*;
use state::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .insert_state(AppState::Play)
        .init_resource::<Player>()
        .add_systems(OnEnter(AppState::Play), play::on_enter)
        .run();
}

fn startup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
}
