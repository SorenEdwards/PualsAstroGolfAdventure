use crate::ball::*;
use crate::geo::*;
use crate::tiling::*;
use crate::controls::*;

pub struct GameMap {
    pub point: Point,
    pub width: usize,
    pub height: usize,
    pub tile_grid: TileGrid,
}
impl GameMap {
    pub fn new() -> GameMap {
        let point = Point::new(0, 0);
        GameMap {
            point: point,
            width: 640,
            height: 368,
            tile_grid: TileGrid::new(),
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        self.tile_grid.draw(frame);
    }
    pub fn point(&self) -> Vec2<usize> {
        Vec2::new(self.point.x, self.point.y)
    }

    /// Returns the size (width and height) of the `Rect`.
    pub fn size(&self) -> Vec2<usize> {
        Vec2::new(self.width, self.height)
    }

    /// Returns the center position of the `Rect`.
    pub fn center(&self) -> Vec2<usize> {
        Vec2::new(
            self.point.x + self.width / 2,
            self.point.y + self.height / 2,
        )
    }

    /// Returns the left edge of the `Rect`
    pub fn left(&self) -> usize {
        self.point.x
    }

    /// Returns the right edge of the `Rect`
    pub fn right(&self) -> usize {
        self.point.x + self.width
    }

    /// Returns the top edge of the `Rect`
    pub fn top(&self) -> usize {
        self.point.y
    }

    /// Returns the bottom edge of the `Rect`
    pub fn bottom(&self) -> usize {
        self.point.y + self.height
    }
}

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
                if self.ball.theta + self.ball.theta_speed == 3.60 {
                    self.ball.theta = 0.0;
                } else {
                    self.ball.theta += self.ball.theta_speed;
                }
            }
            Direction::Right => {
                if self.ball.theta - self.ball.theta_speed == 0.0 {
                    self.ball.theta = 3.6;
                } else {
                    self.ball.theta -= self.ball.theta_speed;
                }
            }
            Direction::Still => {

            }
        }
    }
    fn update_power_level(&mut self, controls: &Controls) {
        match controls.power {
            PowerLevel::Up => {
                if self.ball.power + self.ball.power_step < 5.0 {
                                        self.ball.power += self.ball.power_step;
                                        // println!("{}", self.ball.power);
                                    }
            }
            PowerLevel::Down => {
                if self.ball.power - self.ball.power_step > 1.0 {
                    self.ball.power -= self.ball.power_step;
                    // println!("{}", app.ball.power);
                }
            }
            PowerLevel::Same => {

            }
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

                let is_ball_velocity_in_range =
                    f64::abs(self.ball.vx) <= 10.0 && f64::abs(self.ball.vy) <= 10.0;
                let is_ball_in_hole = self
                    .map
                    .tile_grid
                    .tile_at(x_pos as usize, y_pos as usize)
                    .get_type()
                    == &TileType::Hole
                    && is_ball_velocity_in_range;
                if is_ball_in_hole {
                    self.state = GolfState::InHole;
                }

                if self.ball.vx == 0.0 && self.ball.vy == 0.0 {
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
                let path = self.ball.calc_aim_path(&self.map);
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
}

pub struct InfoScreen {
    loc: Point
}

impl InfoScreen {
    pub fn new() -> Self{
        Self {
            loc: Point::new(0,0)
        }
    }
}


