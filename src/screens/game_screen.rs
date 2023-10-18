use crate::controls::*;
use crate::game::*;
use crate::geo::*;
use pixels::Pixels;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;
pub struct GameScreen {
    pub pixels: Pixels,
    pub game: GameState,
    // pub info: InfoScreen,
    pub controls: Controls,
    pub input: WinitInputHelper,
    pub paused: bool,
}

impl GameScreen {
    pub fn new(pixels: Pixels, debug: bool) -> Self {
        let game = GameState::new();
        Self {
            pixels,
            game: game,
            //   info: InfoScreen::new(),
            controls: Controls::default(),
            input: WinitInputHelper::new(),
            paused: false,
        }
    }
    pub fn update_controls(&mut self) {
        // Pump the gilrs event loop and find an active gamepad
        self.controls = {
            // Keyboard controls
            let shift = self.input.key_held(VirtualKeyCode::LShift);
            let left = self.input.key_held(VirtualKeyCode::Left);
            let right = self.input.key_held(VirtualKeyCode::Right);
            let up = self.input.key_pressed(VirtualKeyCode::Up);
            let down = self.input.key_pressed(VirtualKeyCode::Down);
            let hit = self.input.key_pressed(VirtualKeyCode::Space);
            let pause = self.input.key_pressed(VirtualKeyCode::Pause)
                | self.input.key_pressed(VirtualKeyCode::P);
            let restart = self.input.key_pressed(VirtualKeyCode::Back);
            if pause {
                self.paused = !self.paused;
            }
            if restart {
                self.reset_game();
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

            let adj = if shift {
                AdjustmentType::Min
            } else {
                AdjustmentType::Max
            };

            Controls {
                aiming,
                power,
                adj,
                hit,
            }
        };
    }

    pub fn reset_game(&mut self) {
        self.game.state = GolfState::Aiming;
        self.game.ball.reset_at(Point::new(28, 30));
    }
}
