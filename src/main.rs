extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::actor::Actor;
use chrono::Utc;
use sdl2::mouse::MouseState;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use std::time::Duration;

pub(crate) mod actor;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const SPEED: f32 = 10f32;
const TICK_RATE: i64 = 60;
const TARGET_FRAME_TIME: i64 = 1_000i64 / TICK_RATE;

struct GameState<'a> {
    _sdl_context: Sdl,
    _video_system: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
    last_frame_millis: i64,
    actors: Vec<Box<dyn Actor<'a>>>,
}

pub struct InputState<'a> {
    keyboard_state: KeyboardState<'a>,
    mouse_state: MouseState,
}

impl<'a> GameState<'a> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_system = sdl_context.video().unwrap();

        let window = video_system
            .window("sdl test", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        GameState {
            _sdl_context: sdl_context,
            _video_system: video_system,
            canvas,
            event_pump,
            last_frame_millis: 0,
            actors: vec![],
        }
    }

    pub fn run(&mut self) {
        self.last_frame_millis = Utc::now().timestamp_millis();
        'game: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'game,
                    _ => {}
                }
            }

            self.update();
            self.render();
        }
    }

    fn init(&mut self) {
        // init stuff ...
    }

    fn update(&'a mut self) {
        let input_state = InputState {
            keyboard_state: self.event_pump.keyboard_state(),
            mouse_state: self.event_pump.mouse_state(),
        };
        let delta_time = self.delta_time();

        for actor in &mut self.actors {
            if let Some(updatable) = actor.updatable() {
                updatable.update(&input_state, delta_time);
            }
        }
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // render stuff ...

        self.canvas.present();
    }

    fn delta_time(&mut self) -> f32 {
        let current_millis = Utc::now().timestamp_millis();
        let delta_time =
            (current_millis - self.last_frame_millis) as f32 / TARGET_FRAME_TIME as f32;
        self.last_frame_millis = current_millis;
        delta_time
    }
}

fn main() {
    GameState::new().run();
}
