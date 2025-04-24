#![windows_subsystem = "windows"]
#![allow(dead_code)]

pub mod chart;
pub mod state;
pub mod system;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use chart::player::*;
use state::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CrossChronox".to_string(),
                resolution: (1280., 720.).into(),
                present_mode: bevy::window::PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, startup)
        .insert_state(AppState::Play)
        .init_resource::<Player>()
        .add_systems(OnEnter(AppState::Play), play::on_enter)
        .add_systems(Update, play::update.run_if(in_state(AppState::Play)))
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                width: 1920.0,
                height: 1080.0,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}
