use bevy::color::palettes::css;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::text::FontSmoothing;

use crate::chart::bms_loader::*;
use crate::chart::player::*;
// use crate::state::*;

const GLOBAL_SCROLL: f32 = 0.7 * 480.0;
const SCR_W: f32 = 89.0;
const WHITE_W: f32 = 51.0;
const BLACK_W: f32 = 39.0;
const NOTE_H: f32 = 10.0;
const SPACE: f32 = 3.0;
const SCR_1P_X: f32 = 76.0;
const JUDGELINE_Y: f32 = 715.0;
// const JUDGE_COMBO_1P_X: f32 = -400.0;
// const JUDGE_COMBO_Y: f32 = JUDGELINE_Y + 180.0;
// const JUDGELINE_W: u32 = SCR_W + WHITE_W * 4 + BLACK_W * 3 + SPACE * 7;

// fn convert_x(x: f32) -> f32 {
//     x - 1920.0 / 2.0
// }

const fn convert_x_w(x: f32, w: f32) -> f32 {
    x - 1920.0 / 2.0 + w / 2.0
}

const fn convert_y_h(y: f32, h: f32) -> f32 {
    1080.0 / 2.0 - y - h / 2.0
}

// fn convert_y(y: f32) -> f32 {
//     1080.0 / 2.0 - y
// }

const SCALE: f32 = 1.0; // 1280.0 / 1920.0;
const UI_SCALE: f32 = 1.0; // 1280.0 / 1920.0;

#[derive(Component)]
pub struct InfoText;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct NoteSprite {
    pulse: u64,
    len: u64,
}

pub fn on_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<Player>,
    time: Res<Time>,
) {
    let filename = "assets/songs/BOFU2017/Cagliostro_1011/_Cagliostro_7N.bml";
    let chart = match load_bms(filename, false) {
        Ok(chart) => chart,
        Err(e) => {
            error!("Failed to load chart: {}", e);
            return;
        }
    };
    player.init(chart, &time);

    commands.spawn(Sprite::from_image(asset_server.load("play.png")));

    commands.spawn((
        Text::default(),
        TextFont {
            font: asset_server.load("fonts/kazesawa/Kazesawa-Regular.ttf"),
            font_size: 19.0 * UI_SCALE,
            font_smoothing: FontSmoothing::AntiAliased,
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(900.0 * UI_SCALE),
            top: Val::Px(220.0 * UI_SCALE),
            ..default()
        },
        InfoText,
    ));

    for note in player.chart.notes.iter() {
        let sprite = match note.lane {
            8 => Sprite {
                color: Color::from(css::MAGENTA),
                custom_size: Some(Vec2::new(SCR_W as f32, NOTE_H as f32)),
                ..default()
            },
            1 | 3 | 5 | 7 => Sprite {
                color: Color::from(css::WHITE),
                custom_size: Some(Vec2::new(WHITE_W as f32, NOTE_H as f32)),
                ..default()
            },
            2 | 4 | 6 => Sprite {
                color: Color::from(css::AQUA),
                custom_size: Some(Vec2::new(BLACK_W as f32, NOTE_H as f32)),
                ..default()
            },
            _ => continue,
        };

        let x = SCR_1P_X as i32
            + (SCR_W as i32
                + (note.lane / 2 * WHITE_W as i32
                    + (note.lane - 1) / 2 * BLACK_W as i32
                    + note.lane * SPACE as i32))
                * ((note.lane % 10 % 8) as f32 / 8.0).ceil() as i32;

        commands.spawn((
            Transform::from_translation(Vec3::new(
                convert_x_w(x as f32, sprite.custom_size.unwrap().x),
                convert_y_h(-99999.0, NOTE_H as f32),
                0.0,
            )),
            sprite,
            Visibility::Hidden,
            NoteSprite {
                pulse: note.pulse,
                len: note.len,
            },
        ));
    }
}

pub fn update(
    diagnostics: Res<DiagnosticsStore>,
    mut info_texts: Query<&mut Text, With<InfoText>>,
    mut note_sprites: Query<(&Sprite, &NoteSprite, &mut Transform, &mut Visibility)>,
    mut player: ResMut<Player>,
    time: Res<Time>,
) {
    player.update(&time);

    let fps = match diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        Some(fps) => fps.smoothed().unwrap_or(0.0),
        None => 0.0,
    };

    let info_string = format!(
        r"fps : {:.0}
play_ms : {}
play_pulse : {}
genre : {}
title : {}
subtitle : {}
artist : {}
subartist : {}
difficulty : {}
level : {}
pgreat : {}
great : {}
good : {}
bad : {}
poor : {}
kpoor : {}
max_combo : {}
",
        fps.round(),
        player.play_ms,
        player.now_pulse,
        player.chart.info.genre,
        player.chart.info.title,
        player.chart.info.subtitle,
        player.chart.info.artist,
        player.chart.info.subartist,
        player.chart.info.difficulty,
        player.chart.info.level,
        player.result.pgreat(),
        player.result.great(),
        player.result.good(),
        player.result.bad(),
        player.result.poor(),
        player.result.kpoor(),
        player.result.max_combo,
    );

    for mut text in info_texts.iter_mut() {
        *text = Text::from(info_string.as_str());
    }

    for (sprite, note, mut transform, mut visibility) in note_sprites.iter_mut() {
        let mut lnstart_y = JUDGELINE_Y
            - (note.pulse as i128 - player.now_pulse as i128) as f32 * GLOBAL_SCROLL
                / player.chart.info.resolution as f32;

        let note_y = JUDGELINE_Y
            - (note.pulse as i128 + note.len as i128 - player.now_pulse as i128) as f32
                * GLOBAL_SCROLL
                / player.chart.info.resolution as f32;

        if (JUDGELINE_Y + NOTE_H) < convert_y_h(note_y, sprite.custom_size.unwrap().y) {
            // 判定線を下回ったとき
            *visibility = Visibility::Hidden;
            continue;
        } else if convert_y_h(lnstart_y + NOTE_H, NOTE_H) < 0.0 {
            // まだ画面内に入っていない場合
            *visibility = Visibility::Hidden;
            continue;
        } else {
            // 画面内のノーツ
            *visibility = Visibility::Visible;
        }

        if note.len == 0 {
            transform.translation.y = convert_y_h(note_y, sprite.custom_size.unwrap().y);
        } else {
            if note.pulse < player.now_pulse {
                lnstart_y = JUDGELINE_Y;
            }
            let lnend_y = note_y;
            transform.translation.y = convert_y_h(
                note_y + ((lnstart_y - lnend_y).abs() + NOTE_H),
                sprite.custom_size.unwrap().y,
            );
            transform.scale.y = ((lnstart_y - lnend_y).abs() + NOTE_H) / NOTE_H;
        }
    }
}
