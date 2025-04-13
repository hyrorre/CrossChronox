use sdl3::ttf::Font;

use crate::scene::*;
// use crate::chart::*;

pub struct Select<'app> {
    pub font_select: Option<Font<'app, 'app>>,
}

impl<'app> Select<'app> {
    fn new(_app: &'app mut App) -> Self {
        Select { font_select: None }
    }
}

impl<'app> Scene<'app> for Select<'app> {
    fn init(&mut self) {
        // Initialize the select scene
    }

    fn update(&mut self) -> State {
        // Update the select scene
        State::Continue
    }

    fn render(&mut self) {
        // Render the select scene
    }

    fn quit(&mut self) {
        // Quit the select scene
    }
}
