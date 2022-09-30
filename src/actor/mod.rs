mod ball;
mod paddle;

use crate::InputState;
pub use ball::Ball;
pub use paddle::Paddle;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Actor<'a> {
    fn drawable(&'a self) -> Option<&'a dyn Drawable>;
    fn updatable(&'a mut self) -> Option<&'a mut dyn Updatable>;
}

pub trait Updatable {
    fn update(&mut self, input_state: &InputState, delta_time: f32);
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}
