use std::clone;
use std::rc::Rc;

use crate::controls::*;
use crate::entities::ball::*;
use crate::entities::tiles::*;
use crate::geo::*;
use crate::map::map::*;

#[derive(PartialEq)]
pub enum GolfState {
    Aiming,
    Rolling,
    Hitting,
    InHole,
    Stop,
}
pub struct GameState {
    pub map: GameMap,
    pub state: GolfState,
    pub ball: Ball,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            state: GolfState::Aiming,
            ball: Ball::new_at_loc(320, 48),
            map: GameMap::new(),
        }
    }
    fn update_aiming(&mut self, controls: &Controls) {
        match controls.aiming {
            Direction::Left => {
                self.ball.angle.increase(controls.adj.clone());
                // self.ball.theta += self.ball.theta_speed;
            }
            Direction::Right => {
                self.ball.angle.decrease(controls.adj.clone());
                // self.ball.theta -= self.ball.theta_speed;
            }
            Direction::Still => {}
        }
    }
    fn update_power_level(&mut self, controls: &Controls) {
        match controls.power {
            PowerLevel::Up => {
                self.ball.power.increase(controls.adj.clone());
            }
            PowerLevel::Down => {
                self.ball.power.decrease(controls.adj.clone());
            }
            PowerLevel::Same => {}
        }
    }
    fn update_hitting(&mut self, controls: &Controls) {
        if controls.hit == true && self.state == GolfState::Aiming {
            self.state = GolfState::Hitting;
        }
    }
    fn update_controls(&mut self, controls: &Controls) {
        self.update_aiming(controls);
        self.update_power_level(controls);
        self.update_hitting(controls);
    }
    pub fn update(&mut self, controls: &Controls) {
        self.update_state();
        self.update_controls(controls);
    }
    pub fn update_state(&mut self) {
        match self.state {
            GolfState::Aiming => {}
            GolfState::Rolling => {
                self.ball.roll(&self.map);
                let ball_center = self.ball.center();
                let x_pos = (ball_center.x as i32) >> 4;
                let y_pos = (ball_center.y as i32) >> 4;

                let is_ball_velocity_in_range = f64::abs(self.ball.velocity.x()) <= 10.0
                    && f64::abs(self.ball.velocity.y()) <= 10.0;
                let is_ball_in_hole = self
                    .map
                    .tile_grid
                    .tile_at(x_pos as usize, y_pos as usize)
                    .get_type()
                    == &TileType::Hole
                    && is_ball_velocity_in_range;
                if is_ball_in_hole {
                    self.state = GolfState::InHole;
                    let reset_point = Point::new(28, 30);
                    self.ball.reset_at(reset_point);
                }
                if self.ball.velocity.x() == 0.0 || self.ball.velocity.y() == 0.0 {
                    self.state = GolfState::Aiming;
                }
            }
            GolfState::Hitting => {
                self.ball.hit();
                self.state = GolfState::Rolling;
            }
            GolfState::Stop => {
                self.state = GolfState::Aiming;
            }
            GolfState::InHole => {
                self.ball.loc_x(32);
                self.ball.loc_y(48);
                self.state = GolfState::Aiming;
            }
        }
    }
    pub fn draw(&self, frame: &mut [u8]) {
        match self.state {
            GolfState::Aiming => {
                self.map.draw(frame);
                self.ball.draw(frame);
                let path = self.ball.aim_path();
                path.draw(frame);
            }
            GolfState::Hitting => {
                self.map.draw(frame);
                self.ball.draw(frame);
            }
            GolfState::Rolling => {
                self.map.draw(frame);
                self.ball.draw(frame);
            }
            GolfState::Stop => {
                self.map.draw(frame);
                self.ball.draw(frame);
            }
            GolfState::InHole => {
                self.map.draw(frame);
                self.ball.draw(frame);
            }
        }
    }
    pub fn ball_ref(&self) -> Rc<&Ball> {
        return Rc::new(&self.ball);
    }
}
