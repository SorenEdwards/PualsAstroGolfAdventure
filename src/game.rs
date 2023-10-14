use crate::ball::*;
use crate::geo::*;
use crate::tiling::*;
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
    pub fn update(&mut self) {
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
