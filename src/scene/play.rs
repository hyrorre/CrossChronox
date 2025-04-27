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

        let infostr = format!("fps : {}\n", get_fps())
            + format!("play_ms : {}\n", self.player.play_ms).as_str()
            + format!("play_pulse : {}\n", self.player.now_pulse).as_str()
            + format!("genre : {}\n", self.player.chart.info.genre).as_str()
            + format!("title : {}\n", self.player.chart.info.title).as_str()
            + format!("subtitle : {}\n", self.player.chart.info.subtitle).as_str()
            + format!("artist : {}\n", self.player.chart.info.artist).as_str()
            + format!("subartist : {}\n", self.player.chart.info.subartist).as_str()
            + format!("level : {}\n", self.player.chart.info.level).as_str()
            + format!("pgreat : {}\n", self.player.result.pgreat()).as_str()
            + format!("great : {}\n", self.player.result.great()).as_str()
            + format!("good : {}\n", self.player.result.good()).as_str()
            + format!("bad : {}\n", self.player.result.bad()).as_str()
            + format!("poor : {}\n", self.player.result.poor()).as_str()
            + format!("kpoor : {}\n", self.player.result.kpoor()).as_str();

        draw_multiline_text_ex(
            infostr.as_str(),
            900.0,
            240.0,
            None,
            TextParams {
                font: Some(&self.font),
                font_size: 19,
                color: WHITE,
                ..Default::default()
            },
        );

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
