use crate::App;
use crate::chart::player::PlayState;
use crate::chart::*;
use crate::scene::*;
use sdl3::image::LoadTexture;
use sdl3::pixels::Color;
use sdl3::pixels::PixelFormat;
use sdl3::render::*;
use sdl3::surface::*;
use sdl3::sys::pixels::SDL_PixelFormat;
use sdl3::ttf::*;

const SCR_W: u32 = 89;
const WHITE_W: u32 = 51;
const BLACK_W: u32 = 39;
const NOTE_H: u32 = 10;
const SPACE: u32 = 3;
const SCR_X: f32 = 76.0;
const JUDGELINE_W: u32 = SCR_W + WHITE_W * 4 + BLACK_W * 3 + SPACE * 7;
const JUDGELINE_Y: f32 = 715.0;
const JUDGE_COMBO_X: f32 = SCR_X + 50.0;
const JUDGE_COMBO_Y: f32 = JUDGELINE_Y - 180.0;

pub struct Play<'app> {
    pub app: &'app mut App,
    pub play_states: [PlayState; 2],
    pub font_playinfo: Option<Font<'app, 'app>>,
    pub font_judge: Option<Font<'app, 'app>>,
    pub background: Option<Texture<'app>>,
    pub white: Option<Texture<'app>>,
    pub black: Option<Texture<'app>>,
    pub scr: Option<Texture<'app>>,
}

impl<'app> Scene<'app> for Play<'app> {
    fn new(app: &'app mut App) -> Self {
        let font_playinfo: Option<Font<'_, 'app>> = app
            .ttf_context
            .load_font("CrossChronoxData/Fonts/kazesawa/Kazesawa-Regular.ttf", 19.0)
            .ok();
        let font_judge: Option<Font<'_, 'app>> = app
            .ttf_context
            .load_font("CrossChronoxData/Fonts/kazesawa/Kazesawa-Regular.ttf", 68.0)
            .ok();

        let background = app
            .texture_creator
            .load_texture("CrossChronoxData/play.bmp")
            .ok();

        let pixel_format = PixelFormat::try_from(SDL_PixelFormat::RGBA8888).unwrap();
        let mut surface = Surface::new(WHITE_W, NOTE_H, pixel_format).unwrap();
        surface
            .fill_rect(None, Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let white: Option<Texture<'app>> =
            Texture::from_surface(&surface, &app.texture_creator).ok();

        let pixel_format = PixelFormat::try_from(SDL_PixelFormat::RGBA8888).unwrap();
        let mut surface = Surface::new(BLACK_W, NOTE_H, pixel_format).unwrap();
        surface
            .fill_rect(None, Color::RGBA(0, 174, 239, 255))
            .unwrap();
        let black: Option<Texture<'app>> =
            Texture::from_surface(&surface, &app.texture_creator).ok();

        let pixel_format = PixelFormat::try_from(SDL_PixelFormat::RGBA8888).unwrap();
        let mut surface = Surface::new(SCR_W, NOTE_H, pixel_format).unwrap();
        surface
            .fill_rect(None, Color::RGBA(0, 174, 239, 255))
            .unwrap();
        let scr: Option<Texture<'app>> = Texture::from_surface(&surface, &app.texture_creator).ok();

        Play {
            app,
            play_states: [PlayState::new(), PlayState::new()],
            font_playinfo,
            font_judge,
            background,
            white,
            black,
            scr,
        }
    }

    fn init(&mut self) {
        // Initialize the play scene
    }

    fn update(&mut self) -> State {
        // Update the play scene
        let mut continue_flag = false;
        for play_state in self.play_states.iter_mut() {
            if matches!(play_state.update(&self.app.time_manager), State::Continue) {
                continue_flag = true;
            }
        }

        if continue_flag {
            return State::Continue;
        }
        State::Finish
    }

    fn render(&mut self) {
        // Render the play scene
        if let Some(ref background) = self.background {
            self.app.canvas.copy(background, None, None).unwrap();
        }
        self.app.canvas.present();
        self.app.canvas.clear();
    }

    fn quit(&mut self) {
        // Quit the play scene
    }
}
