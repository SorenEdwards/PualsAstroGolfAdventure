#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::common::*;
use crate::drawing::*;
use crate::game::*;
use crate::controls::*;
use error_iter::ErrorIter as _;
use crate::geo::Point;
use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize, event::VirtualKeyCode, event_loop::EventLoop, window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use game_loop::{game_loop, Time, TimeTrait as _};
use std::{env, time::Duration};

pub mod ball;
pub mod common;
pub mod drawing;
pub mod game;
pub mod geo;
pub mod helpers;
pub mod shapes;
pub mod sprites;
pub mod tiling;
pub mod controls;
pub const FPS: usize = 144;
pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);
// Internally, the game advances at 60 fps
const ONE_FRAME: Duration = Duration::from_nanos(1_000_000_000 / 60);

// const BOX_SIZE: i16 = 64;
// const static [u8] BALL = [0xff,0xff];

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}


pub struct GameScreen {
    pub pixels: Pixels,
    pub game: GameState,
    pub info: InfoScreen,
    pub controls: Controls,
    pub input: WinitInputHelper,
    pub paused: bool,
}

impl GameScreen {
    fn new(pixels: Pixels, debug: bool) -> Self {
        Self {
          pixels,
          game: GameState::new(), 
          info: InfoScreen::new(),
          controls: Controls::default(),
          input: WinitInputHelper::new(), 
          paused: false
        }
    }
    fn update_controls(&mut self) {
        // Pump the gilrs event loop and find an active gamepad
        self.controls = {
            // Keyboard controls
            let mut left = self.input.key_held(VirtualKeyCode::Left);
            let mut right = self.input.key_held(VirtualKeyCode::Right);
            let mut up = self.input.key_held(VirtualKeyCode::Up);
            let mut down = self.input.key_held(VirtualKeyCode::Down);
            let mut hit = self.input.key_pressed(VirtualKeyCode::Space);
            let mut pause = self.input.key_pressed(VirtualKeyCode::Pause)
                | self.input.key_pressed(VirtualKeyCode::P);

            if pause {
                self.paused = !self.paused;
            }

            let aiming = if left {
                Direction::Left
            } else if right {
                Direction::Right
            } else {
                Direction::Still
            };

            let power = if up {
                PowerLevel::Up
            } else if down {
                PowerLevel::Down
            } else {
                PowerLevel::Same
            };

            Controls { aiming, power, hit }
        };
    }

    fn reset_game(&mut self) {
        self.game.ball.move_to(Point::new(28,30));
    }
}




fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();

    // Enable debug mode with `DEBUG=true` environment variable
    let debug = env::var("DEBUG")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .unwrap_or(false);

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Pual's Astro Golf Adventure")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    let game = GameScreen::new(pixels, debug);

    game_loop(
        event_loop,
        window,
        game,
        FPS as u32,
        0.1,
        move |g| {
            // Update the world
            if !g.game.paused {
                g.game.game.update(&g.game.controls);
            }
        },
        move |g| {
            // Drawing
            g.game.game.draw(g.game.pixels.frame_mut());
            if let Err(err) = g.game.pixels.render() {
                log_error("pixels.render", err);
                g.exit();
            }

            // Sleep the main thread to limit drawing to the fixed time step.
            // See: https://github.com/parasyte/pixels/issues/174
            let dt = TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
            if dt > 0.0 {
                std::thread::sleep(Duration::from_secs_f64(dt));
            }
        },
        |g, event| {
            // Let winit_input_helper collect events to build its state.
            if g.game.input.update(event) {
                // Update controls
                g.game.update_controls();

                // Close events
                if g.game.input.key_pressed(VirtualKeyCode::Escape)
                    || g.game.input.close_requested()
                {
                    g.exit();
                    return;
                }

                // Reset game
                if g.game.input.key_pressed(VirtualKeyCode::R) {
                    g.game.reset_game();
                }

                // Resize the window
                if let Some(size) = g.game.input.window_resized() {
                    if let Err(err) = g.game.pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        g.exit();
                    }
                }
            }
        },
    );
}


// fn main() -> Result<(), Error> {
//     env_logger::init();
//     let event_loop = EventLoop::new();
//     let mut input = WinitInputHelper::new();
//     let window = {
//         let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
//         WindowBuilder::new()
//             .with_title("Hello Pixels")
//             .with_inner_size(size)
//             .with_min_inner_size(size)
//             .build(&event_loop)
//             .unwrap()
//     };

//     let mut pixels = {
//         let window_size = window.inner_size();
//         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
//         Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
//     };
//     // let tick_rate = Duration::from_nanos(50000);
//     let mut app = GameState::new();
//     // let mut ball = Ball::new();
//     // let tilegrid = TileGrid::new();
//     event_loop.run(move |event, _, control_flow| {
//         // Draw the current frame
//         if let Event::RedrawRequested(_) = event {
//             app.draw(pixels.frame_mut());
//             app.update();
//             window.request_redraw();
//             if let Err(err) = pixels.render() {
//                 log_error("pixels.render", err);
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }
//         }

//         // Handle input events
//         if input.update(&event) {
//             // Close events
//             if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }

//             if input.key_pressed(VirtualKeyCode::Q) || input.close_requested() {
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }
//             if input.key_held(VirtualKeyCode::Left) || input.key_pressed(VirtualKeyCode::Left) {
//                 if app.ball.theta + app.ball.theta_speed == 3.60 {
//                     app.ball.theta = 0.0;
//                 } else {
//                     app.ball.theta += app.ball.theta_speed;
//                 }
//                 window.request_redraw();
//             }
//             if input.key_held(VirtualKeyCode::Right) || input.key_pressed(VirtualKeyCode::Right) {
//                 if app.ball.theta - app.ball.theta_speed == 0.0 {
//                     app.ball.theta = 3.6;
//                 } else {
//                     app.ball.theta -= app.ball.theta_speed;
//                 }
//                 window.request_redraw();
//             }
//             if input.key_held(VirtualKeyCode::Up) || input.key_pressed(VirtualKeyCode::Up) {
//                 if app.ball.power + app.ball.power_step < 20.0 {
//                     app.ball.power += app.ball.power_step;
//                     println!("{}", app.ball.power);
//                 }
//                 window.request_redraw();
//             }
//             if input.key_held(VirtualKeyCode::Down) || input.key_pressed(VirtualKeyCode::Down) {
//                 if app.ball.power - app.ball.power_step > 1.0 {
//                     app.ball.power -= app.ball.power_step;
//                     println!("{}", app.ball.power);
//                 }
//                 window.request_redraw();
//             }
//             if input.key_pressed(VirtualKeyCode::Space) {
//                 if app.state == GolfState::Aiming {
//                     app.state = GolfState::Hitting;
//                 }
//                 window.request_redraw();
//             }
//             if input.key_pressed(VirtualKeyCode::Return) {
//                 if app.state == GolfState::Rolling {
//                     app.state = GolfState::Stop;
//                 }
//                 let ret_point = Point::new(320, 48);
//                 app.ball.move_to(ret_point);
//                 window.request_redraw();
//             }

//             // Resize the window
//             if let Some(size) = input.window_resized() {
//                 if let Err(err) = pixels.resize_surface(size.width, size.height) {
//                     log_error("pixels.resize_surface", err);
//                     *control_flow = ControlFlow::Exit;
//                     return;
//                 }
//             }
//             app.update();
//             app.draw(pixels.frame_mut());
//             window.request_redraw();
//             // Update internal state and request a redraw
//         }
//         app.update();
//         window.request_redraw();
//     });
// }
