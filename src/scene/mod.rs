use kira::AudioManager;

pub mod play;
// pub mod select;

pub trait Scene {
    fn init(&mut self);
    fn update(&mut self, audio_manager: &mut AudioManager);
    fn render(&mut self);
    fn quit(&mut self);
}
