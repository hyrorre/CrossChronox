pub mod play;
pub mod select;

pub enum State {
    Finish,
    Continue,
}

pub trait Scene {
    fn init(&self);
    fn update(&self) -> State;
    fn render(&self);
    fn quit(&self);
}
