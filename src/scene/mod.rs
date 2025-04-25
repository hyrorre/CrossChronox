pub mod play;
// pub mod select;

pub trait Scene {
    fn init(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn quit(&mut self);
}
