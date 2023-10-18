#![deny(clippy::all)]
#![forbid(unsafe_code)]

use paga::common::*;
use paga::screens::game_screen::*;

use error_iter::ErrorIter as _;
use game_loop::{game_loop, Time, TimeTrait as _};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::{env, time::Duration};
use winit::{
    dpi::LogicalSize, event::VirtualKeyCode, event_loop::EventLoop, window::WindowBuilder,
};

pub const FPS: usize = 144;
pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);
// Internally, the game advances at 60 fps
// const ONE_FRAME: Duration = Duration::from_nanos(1_000_000_000 / 60);

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
