use crate::App;
use crate::chart::player::PlayState;
use crate::scene::*;
use crate::system::time::TimeManager;
use sdl3::image::LoadTexture;
use sdl3::pixels::Color;
use sdl3::pixels::PixelFormat;
use sdl3::rect::Rect;
use sdl3::render::*;
use sdl3::surface::*;
use sdl3::sys::pixels::SDL_PixelFormat;
use sdl3::ttf::*;
use sdl3::video::WindowContext;

const SCR_W: u32 = 89;
const WHITE_W: u32 = 51;
const BLACK_W: u32 = 39;
const NOTE_H: u32 = 10;
const SPACE: u32 = 3;
const SCR_1P_X: f32 = 76.0;
const JUDGELINE_W: u32 = SCR_W + WHITE_W * 4 + BLACK_W * 3 + SPACE * 7;
const JUDGELINE_Y: f32 = 715.0;
const JUDGE_COMBO_1P_X: f32 = SCR_1P_X + 50.0;
const JUDGE_COMBO_Y: f32 = JUDGELINE_Y - 180.0;
const GLOBAL_SCROLL: f32 = 0.7 * 480.0;

pub struct Play<'app> {
    canvas: &'app mut WindowCanvas,
    texture_creator: &'app TextureCreator<WindowContext>,
    ttf_context: &'app Sdl3TtfContext,
    time_manager: &'app TimeManager,
    play_states: [PlayState; 2],
    font_playinfo: Option<Font<'app, 'app>>,
    font_judge: Option<Font<'app, 'app>>,
    background: Option<Texture<'app>>,
    white: Option<Texture<'app>>,
    black: Option<Texture<'app>>,
    scr: Option<Texture<'app>>,
    frame_count: i32,
    fps: i32,
}

impl<'app> Play<'app> {
    pub fn new(app: &'app mut App, play_states: [PlayState; 2]) -> Self {
        let font_playinfo = app
            .ttf_context
            .load_font("CrossChronoxData/Fonts/kazesawa/Kazesawa-Regular.ttf", 19.0)
            .ok();
        let font_judge = app
            .ttf_context
            .load_font("CrossChronoxData/Fonts/kazesawa/Kazesawa-Regular.ttf", 68.0)
            .ok();

        let background = app
            .texture_creator
            .load_texture("CrossChronoxData/play.bmp")
            .ok();

        let pixel_format = PixelFormat::try_from(SDL_PixelFormat::RGBA8888).unwrap();
        let mut surface = Surface::new(WHITE_W, NOTE_H, pixel_format).unwrap();
        surface.fill_rect(None, Color::WHITE).unwrap();
        let white = Texture::from_surface(&surface, &app.texture_creator).ok();

        let pixel_format = PixelFormat::try_from(SDL_PixelFormat::RGBA8888).unwrap();
        let mut surface = Surface::new(BLACK_W, NOTE_H, pixel_format).unwrap();
        surface.fill_rect(None, Color::CYAN).unwrap();
        let black = Texture::from_surface(&surface, &app.texture_creator).ok();

        let pixel_format = PixelFormat::try_from(SDL_PixelFormat::RGBA8888).unwrap();
        let mut surface = Surface::new(SCR_W, NOTE_H, pixel_format).unwrap();
        surface.fill_rect(None, Color::MAGENTA).unwrap();
        let scr = Texture::from_surface(&surface, &app.texture_creator).ok();

        Play {
            canvas: &mut app.canvas,
            texture_creator: &app.texture_creator,
            ttf_context: &app.ttf_context,
            time_manager: &app.time_manager,
            play_states,
            font_playinfo,
            font_judge,
            background,
            white,
            black,
            scr,
            frame_count: 0,
            fps: 0,
        }
    }
}

impl<'app> Scene<'app> for Play<'app> {
    fn init(&mut self) {
        // Initialize the play scene
    }

    fn update(&mut self) -> State {
        let mut continue_flag = false;
        for play_state in self.play_states.iter_mut() {
            if matches!(play_state.update(self.time_manager), State::Continue) {
                continue_flag = true;
            }
        }

        if continue_flag {
            return State::Continue;
        }
        State::Finish
    }

    fn render(&mut self) {
        if let Some(ref background) = self.background {
            self.canvas.copy(background, None, None).unwrap();
        }

        for (i, play_state) in self.play_states.iter().enumerate() {
            if i == 0 {
                self.frame_count += 1;
                if (play_state.play_ms as f32 / 1000.0) as i32
                    - (play_state.last_play_ms as f32 / 1000.0) as i32
                    != 0
                {
                    self.fps = self.frame_count;
                    self.frame_count = 0;
                }
            }

            let chart = &play_state.chart;
            let infostr = [
                &format!("fps : {}", self.fps),
                &format!("play_ms : {}", play_state.play_ms),
                &format!("play_pulse : {}", play_state.now_pulse),
                &chart.info.genre,
                &chart.info.title,
                &chart.info.subtitle,
                &chart.info.artist,
                &chart.info.subartist,
            ];

            if let Some(font) = &self.font_playinfo {
                let mut y = 220;
                for line in infostr {
                    if let Ok(surface) = font.render(line.as_str()).blended(Color::WHITE) {
                        if let Ok(texture) =
                            self.texture_creator.create_texture_from_surface(surface)
                        {
                            self.canvas
                                .copy(
                                    &texture,
                                    None,
                                    Rect::new(900, y, texture.width(), texture.height()),
                                )
                                .ok();
                            y += font.recommended_line_spacing();
                        }
                    }
                }
            }

            for note in chart.notes.iter() {
                let mut lnstart_y = JUDGELINE_Y
                    - (note.pulse as i128 - play_state.now_pulse as i128) as f32 * GLOBAL_SCROLL
                        / chart.info.resolution as f32;
                let note_y = JUDGELINE_Y
                    - (note.pulse as i128 + note.len as i128 - play_state.now_pulse as i128) as f32
                        * GLOBAL_SCROLL
                        / chart.info.resolution as f32;
                if (JUDGELINE_Y as i32 + NOTE_H as i32) < note_y as i32 {
                    continue;
                } else if (lnstart_y as i32 + NOTE_H as i32) < 0 {
                    break;
                }
                if let Some(texture) = match note.lane {
                    8 => &self.scr,
                    1 | 3 | 5 | 7 => &self.white,
                    2 | 4 | 6 => &self.black,
                    _ => &None,
                } {
                    if note.len == 0 {
                        let x = SCR_1P_X as i32
                            + (SCR_W as i32
                                + (note.lane / 2 * WHITE_W as i32
                                    + (note.lane - 1) / 2 * BLACK_W as i32
                                    + note.lane * SPACE as i32))
                                * ((note.lane % 10 % 8) as f32 / 8.0).ceil() as i32;
                        self.canvas
                            .copy(
                                texture,
                                None,
                                Rect::new(x, note_y as i32, texture.width(), texture.height()),
                            )
                            .ok();
                    } else {
                        if note.pulse < play_state.now_pulse {
                            lnstart_y = JUDGELINE_Y;
                        }
                        let lnend_y = note_y;
                        let x = SCR_1P_X as i32
                            + (SCR_W as i32
                                + (note.lane / 2 * WHITE_W as i32
                                    + (note.lane - 1) / 2 * BLACK_W as i32
                                    + note.lane * SPACE as i32))
                                * ((note.lane % 10 % 8) as f32 / 8.0).ceil() as i32;
                        self.canvas
                            .copy(
                                texture,
                                None,
                                Rect::new(
                                    x,
                                    note_y as i32,
                                    texture.width(),
                                    (lnstart_y - lnend_y).abs() as u32 + NOTE_H as u32,
                                ),
                            )
                            .ok();
                    }
                }
            }
        }

        self.canvas.present();
        self.canvas.clear();
    }

    fn quit(&mut self) {
        // Quit the play scene
    }
}
