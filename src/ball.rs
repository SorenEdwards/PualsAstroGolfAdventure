use crate::common::*;
use crate::game::*;
use crate::geo::*;
use crate::helpers::*;
use crate::sprites::*;
use crate::tiling::*;


static PI: f64 = 3.14159265359;

pub struct AimLine {
    p1: Point,
    p2: Point,
}
impl AimLine {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);
        Self { p1, p2 }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        line(frame, &self.p1, &self.p2, [0xff, 0xff, 0xff, 0xff]);
    }
}
pub struct Ball {
    old_point: Point,
    point: Point,
    sprite: Sprite,
    pub vx: f64,
    pub vy: f64,
    pub speed: f64,
    pub theta: f64,
    pub theta_speed: f64,
    pub power: f64,
    pub power_step: f64,
}
impl Ball {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new_at_loc(x: usize, y: usize) -> Self {
        let template = map_color_to_rgba(&BALL_ASSET);
        let point = Point::new(x, y);
        let old_point = point.clone();
        let sprite = Sprite {
            width: 8,
            height: 8,
            pixels: template,
        };

        Self {
            old_point,
            point,
            sprite,
            vx: 0.0,
            vy: 0.0,
            speed: 1.009,
            theta: 3.6,
            theta_speed: 0.06,
            power: 10.0,
            power_step: 1.0,
        }
    }
    fn new() -> Self {
        return Ball::new_at_loc(100, 100);
    }
    pub fn calc_aim_path(&self, playground: &GameMap) -> AimLine {
        // float starX = x+0.5+power/5*i*5*(float)sin(theta - PI/2);
        // float starY = y+0.5+power/5*i*5*(float)cos(theta + PI/2);
        let ball_center = self.center();
        let path_s_x =
            ball_center.x as f64 + self.power / 3.0 * 1.0 * 3.0 * f64::sin(self.theta - PI / 2.0);
        let path_s_y =
            ball_center.y as f64 + self.power / 3.0 * 1.0 * 3.0 * f64::cos(self.theta + PI / 2.0);
        let path_x =
            ball_center.x as f64 + self.power / 3.0 * 12.0 * 3.0 * f64::sin(self.theta - PI / 2.0);
        let path_y =
            ball_center.y as f64 + self.power / 3.0 * 12.0 * 3.0 * f64::cos(self.theta + PI / 2.0);
        let x1 = path_s_x; //self.x + (self.width / 2.0);
        let y1 = path_s_y; //self.y + (self.height / 2.0);
        let x2;
        let y2;
        let _x = match (
            (path_x > playground.point.x as f64),
            (path_x < playground.width as f64),
        ) {
            (true, true) => {
                x2 = path_x;
            }
            (true, false) => {
                x2 = playground.width as f64;
            }
            (false, true) => {
                x2 = playground.point.x as f64;
            }
            (false, false) => {
                x2 = self.point.x as f64;
            }
        };
        let _y = match (
            (path_y > playground.point.y as f64),
            (path_y < playground.height as f64),
        ) {
            (true, true) => {
                y2 = path_y;
            }
            (true, false) => {
                y2 = playground.height as f64;
            }
            (false, true) => {
                y2 = playground.point.y as f64;
            }
            (false, false) => {
                y2 = self.point.y as f64;
            }
        };
        let line = AimLine::new(x1 as usize, y1 as usize, x2 as usize, y2 as usize);
        return line;
    }

    pub fn hit(&mut self) {
        self.vx = 2.0 * self.power * self.speed * f64::sin(self.theta - PI / 2.0);
        self.vy = 2.0 * self.power * self.speed * f64::cos(self.theta + PI / 2.0);
    }
    pub fn roll(&mut self, playground: &GameMap) {
        self.handle_collision(playground);
        self.old_point.x = self.point.x;
        self.old_point.y = self.point.y;
        self.point.x = (self.point.x as f64 + (self.vx)) as usize;
        self.point.y = (self.point.y as f64 + (self.vy)) as usize;
        self.vx = self.vx * 0.99110;
        self.vy = self.vy * 0.99110;
        if f64::abs(self.vx) < 1.00 {
            self.vx = 0.0;
        }
        if f64::abs(self.vy) < 1.00 {
            self.vy = 0.0;
        }
    }
    pub fn handle_collision(&mut self, map: &GameMap) {
        let ball_center = self.center();
        let x_pos = (ball_center.x as i32) >> 4;
        let y_pos = (ball_center.y as i32) >> 4;

        let x_upper = self.left() + 10;
        let x_lower = self.right() - 10;

        let y_upper = self.top() + 10;
        let y_lower = self.bottom() - 10;

        let x_pos_upper = (x_upper as i32) >> 4;
        let x_pos_lower = (x_lower as i32) >> 4;

        let y_pos_upper = (y_upper as i32) >> 4;
        let y_pos_lower = (y_lower as i32) >> 4;
        println!("X UP: {} X LOWER: {} Y UP: {} Y: LOWER{}",x_pos_upper,x_pos_lower,y_pos_upper,y_pos_lower);
        let x_upper_collides_wall = map
            .tile_grid
            .tile_at(x_pos_upper as usize, y_pos as usize)
            .get_type()
            == &TileType::Wall
            && self.vx > 0.0;
        let x_lower_collides_wall = map
            .tile_grid
            .tile_at(x_pos_lower as usize, y_pos as usize)
            .get_type()
            == &TileType::Wall
            && self.vx < 0.0;
        let y_upper_collides_wall = map
            .tile_grid
            .tile_at(x_pos as usize, y_pos_upper as usize)
            .get_type()
            == &TileType::Wall
            && self.vy > 0.0;
        let y_lower_collides_wall = map
            .tile_grid
            .tile_at(x_pos as usize, y_pos_lower as usize)
            .get_type()
            == &TileType::Wall
            && self.vy < 0.0;

        if x_upper_collides_wall || x_lower_collides_wall || !self.is_within_x_bounds(map) {
            self.point.x = self.old_point.x;
            self.vx = -self.vx;
        }

        if y_upper_collides_wall || y_lower_collides_wall || !self.is_within_y_bounds(map) {
            self.point.y = self.old_point.y;
            self.vy = -self.vy;
        }

    }
    pub fn is_within_x_bounds(&self, playground: &GameMap) -> bool {
        if self.point.x < playground.left() || self.point.x + self.sprite.width > playground.right()
        {
            return false;
        } else {
            return true;
        }
    }
    pub fn is_within_y_bounds(&self, playground: &GameMap) -> bool {
        if self.point.y < playground.top()
            || self.point.y + self.sprite.height > playground.bottom()
        {
            return false;
        } else {
            return true;
        }
    }
    pub fn point(&self) -> Point {
        return self.point;
    }

    pub fn loc_x(&mut self, x: usize) {
        self.point.x = x;
    }
    pub fn loc_y(&mut self, y: usize) {
        self.point.y = y;
    }
    /// Returns the size (width and height) of the `Rect`.
    pub fn size(&self) -> (usize, usize) {
        (self.sprite.width, self.sprite.height)
    }

    /// Returns the center position of the `Rect`.
    pub fn center(&self) -> Vec2<usize> {
        Vec2::new(
            self.point.x + (self.sprite.width / 2),
            self.point.y + (self.sprite.height / 2),
        )
    }

    /// Returns the left edge of the `Rect`
    pub fn left(&self) -> usize {
        self.point.x
    }

    /// Returns the right edge of the `Rect`
    pub fn right(&self) -> usize {
        self.point.x + self.sprite.width
    }

    /// Returns the top edge of the `Rect`
    pub fn top(&self) -> usize {
        self.point.y
    }

    /// Returns the bottom edge of the `Rect`
    pub fn bottom(&self) -> usize {
        self.point.y + self.sprite.height
    }

    /// Moves the `Rect`'s origin to (x, y)
    pub fn move_to(&mut self, destination: Point) {
        self.point.x = destination.x;
        self.point.y = destination.y;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        blit(frame, &self.point, &self.sprite);
    }
}
