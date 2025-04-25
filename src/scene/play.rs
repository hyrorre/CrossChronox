use macroquad::prelude::*;

use crate::chart::player::*;
use crate::scene::*;

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

pub struct Play {
    player: Player,
    font: Font,
    background: Texture2D,
    scr: Texture2D,
    white: Texture2D,
    black: Texture2D,
}

impl Play {
    pub async fn new(player: Player) -> Self {
        let font = load_ttf_font("assets/fonts/kazesawa/Kazesawa-Regular.ttf")
            .await
            .unwrap();

        let background = load_texture("assets/play.png").await.unwrap();

        let image = Image::gen_image_color(SCR_W as u16, NOTE_H as u16, RED);
        let scr = Texture2D::from_image(&image);

        let image = Image::gen_image_color(WHITE_W as u16, NOTE_H as u16, WHITE);
        let white = Texture2D::from_image(&image);

        let image = Image::gen_image_color(BLACK_W as u16, NOTE_H as u16, BLUE);
        let black = Texture2D::from_image(&image);

        Play {
            player,
            font,
            background,
            scr,
            white,
            black,
        }
    }
}

impl Scene for Play {
    fn init(&mut self) {
        // Initialize the play scene
    }

    fn update(&mut self) {
        self.player.update();
    }

    fn render(&mut self) {
        draw_texture(&self.background, 0., 0., WHITE);

        let infostr = [
            format!("fps : {}", get_fps()),
            format!("play_ms : {}", self.player.play_ms),
            format!("play_pulse : {}", self.player.now_pulse),
            format!("genre : {}", self.player.chart.info.genre),
            format!("title : {}", self.player.chart.info.title),
            format!("subtitle : {}", self.player.chart.info.subtitle),
            format!("artist : {}", self.player.chart.info.artist),
            format!("subartist : {}", self.player.chart.info.subartist),
            format!("level : {}", self.player.chart.info.level),
            format!("pgreat : {}", self.player.result.pgreat()),
            format!("great : {}", self.player.result.great()),
            format!("good : {}", self.player.result.good()),
            format!("bad : {}", self.player.result.bad()),
            format!("poor : {}", self.player.result.poor()),
            format!("kpoor : {}", self.player.result.kpoor()),
        ];

        let mut y = 220.0;
        for line in infostr {
            draw_text_ex(
                line.as_str(),
                900.0,
                y,
                TextParams {
                    font: Some(&self.font),
                    font_size: 19,
                    color: WHITE,
                    ..Default::default()
                },
            );
            y += 19.0 + 5.0;
        }

        for note in self.player.chart.notes.iter() {
            let mut lnstart_y = JUDGELINE_Y
                - (note.pulse as i128 - self.player.now_pulse as i128) as f32 * GLOBAL_SCROLL
                    / self.player.chart.info.resolution as f32;
            let note_y = JUDGELINE_Y
                - (note.pulse as i128 + note.len as i128 - self.player.now_pulse as i128) as f32
                    * GLOBAL_SCROLL
                    / self.player.chart.info.resolution as f32;
            if (JUDGELINE_Y as i32 + NOTE_H as i32) < note_y as i32 {
                continue;
            } else if (lnstart_y as i32 + NOTE_H as i32) < 0 {
                break;
            }
            let texture = match note.lane {
                8 => &self.scr,
                1 | 3 | 5 | 7 => &self.white,
                2 | 4 | 6 => &self.black,
                _ => continue,
            };
            let x = SCR_1P_X as i32
                + (SCR_W as i32
                    + (note.lane / 2 * WHITE_W as i32
                        + (note.lane - 1) / 2 * BLACK_W as i32
                        + note.lane * SPACE as i32))
                    * ((note.lane % 10 % 8) as f32 / 8.0).ceil() as i32;
            if note.len == 0 {
                draw_texture(texture, x as f32, note_y, WHITE);
            } else {
                if note.pulse < self.player.now_pulse {
                    lnstart_y = JUDGELINE_Y;
                }
                let lnend_y = note_y;

                draw_texture_ex(
                    texture,
                    x as f32,
                    note_y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2 {
                            x: texture.width(),
                            y: (lnstart_y - lnend_y).abs() + NOTE_H,
                        }),
                        ..Default::default()
                    },
                )
            }
        }
    }

    fn quit(&mut self) {
        // Quit the play scene
    }
}
