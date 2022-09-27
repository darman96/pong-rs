use std::time::Duration;

use sdl2::keyboard::Keycode;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const SPEED: f32 = 10f32;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("sdl test", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut paddle = Paddle {
        x: 5f32,
        y: (HEIGHT / 2) as f32,
        width: 10f32,
        height: 125f32,
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let keystate = event_pump.keyboard_state();

        match keystate.is_scancode_pressed(sdl2::keyboard::Scancode::Up) {
            true => {
                paddle.y -= SPEED;
                paddle.y = paddle.y.clamp(5f32, ((HEIGHT - 5) as f32) - paddle.height);
            },
            false => {}
        };

        match keystate.is_scancode_pressed(sdl2::keyboard::Scancode::Down) {
            true => {
                paddle.y += SPEED;
                paddle.y = paddle.y.clamp(5f32, ((HEIGHT - 5) as f32) - paddle.height);
            },
            false => {}
        };

        draw_paddle(&paddle, &mut canvas);
        draw_ball((WIDTH as i32) / 2i32, (HEIGHT as i32) / 2i32, 5i32, &mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_paddle(paddle: &Paddle, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(Rect::new(paddle.x as i32, paddle.y as i32, paddle.width as u32, paddle.height as u32))
        .unwrap();
}

fn draw_ball(x: i32, y: i32, r: i32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(Rect::new(x - r, y - r, 2 * (r as u32), 2 * (r as u32)))
        .unwrap();
}

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}