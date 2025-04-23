#![windows_subsystem = "windows"]
#![allow(dead_code)]

// pub mod chart;
// pub mod scene;
// pub mod system;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mover,
    ));
}

#[derive(Component)]
struct Mover;

fn update(time: Res<Time>, mut query: Query<&mut Transform, With<Mover>>) {
    for mut transform in &mut query {
        transform.translation.x += 100.0 * time.delta_secs();
    }
}
