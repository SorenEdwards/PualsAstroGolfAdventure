#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::ball::*;
use crate::common::*;
use crate::drawing::*;
use crate::game::*;
use crate::geo::*;
use crate::helpers::*;
use crate::shapes::*;
use crate::sprites::*;
use crate::tiling::*;

use core::cmp::min;
use core::time::Duration;
use error_iter::ErrorIter as _;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::rc::Rc;
use std::vec::Vec;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
pub mod ball;
pub mod common;
pub mod drawing;
pub mod game;
pub mod geo;
pub mod helpers;
pub mod shapes;
pub mod sprites;
pub mod tiling;
// const BOX_SIZE: i16 = 64;
// const static [u8] BALL = [0xff,0xff];

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };
    // let tick_rate = Duration::from_nanos(50000);
    let mut app = GameState::new();
    // let mut ball = Ball::new();
    // let tilegrid = TileGrid::new();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            app.draw(pixels.frame_mut());
            app.update();
            window.request_redraw();
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Q) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_held(VirtualKeyCode::Left) || input.key_pressed(VirtualKeyCode::Left) {
                if app.ball.theta + app.ball.theta_speed == 3.60 {
                    app.ball.theta = 0.0;
                } else {
                    app.ball.theta += app.ball.theta_speed;
                }
                window.request_redraw();
            }
            if input.key_held(VirtualKeyCode::Right) || input.key_pressed(VirtualKeyCode::Right) {
                if app.ball.theta - app.ball.theta_speed == 0.0 {
                    app.ball.theta = 3.6;
                } else {
                    app.ball.theta -= app.ball.theta_speed;
                }
                window.request_redraw();
            }
            if input.key_held(VirtualKeyCode::Up) || input.key_pressed(VirtualKeyCode::Up) {
                if app.ball.power + app.ball.power_step < 20.0 {
                    app.ball.power += app.ball.power_step;
                    println!("{}", app.ball.power);
                }
                window.request_redraw();
            }
            if input.key_held(VirtualKeyCode::Down) || input.key_pressed(VirtualKeyCode::Down) {
                if app.ball.power - app.ball.power_step > 1.0 {
                    app.ball.power -= app.ball.power_step;
                    println!("{}", app.ball.power);
                }
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::Space) {
                if app.state == GolfState::Aiming {
                    app.state = GolfState::Hitting;
                }
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::Return) {
                if app.state == GolfState::Rolling {
                    app.state = GolfState::Stop;
                }
                let ret_point = Point::new(320, 48);
                app.ball.move_to(ret_point);
                window.request_redraw();
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            app.update();
            app.draw(pixels.frame_mut());
            window.request_redraw();
            // Update internal state and request a redraw
        }
        app.update();
        window.request_redraw();
    });
}
