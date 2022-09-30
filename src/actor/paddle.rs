use crate::actor::{Actor, Drawable, Updatable};
use crate::InputState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Drawable for Paddle {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        let prev_color = canvas.draw_color();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(Rect::new(self.x, self.y, self.width, self.height))
            .unwrap();
        canvas.set_draw_color(prev_color);
    }
}

impl Updatable for Paddle {
    fn update(&mut self, input_state: &InputState, delta_time: f32) {
        todo!()
    }
}

impl<'a> Actor<'a> for Paddle {
    fn drawable(&'a self) -> Option<&'a dyn Drawable> {
        Some(self)
    }

    fn updatable(&'a mut self) -> Option<&'a mut dyn Updatable> {
        Some(self)
    }
}
