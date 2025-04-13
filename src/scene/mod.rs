use crate::App;

pub mod play;
pub mod select;

pub enum State {
    Finish,
    Continue,
}

pub trait Scene<'app> {
    fn new(app: &'app mut App) -> Self;
    fn init(&mut self);
    fn update(&mut self) -> State;
    fn render(&mut self);
    fn quit(&mut self);
}
