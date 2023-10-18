use crate::common::*;
use crate::controls::AdjustmentType;
use crate::entities::tiles::*;
use crate::geo::*;
use crate::map::map::*;
use crate::render::helpers::*;
use crate::render::sprites::*;

static PI: f64 = 3.14159265359;
const POWER_MAX: f64 = 10.3;
const POWER_MIN: f64 = 0.7;

pub fn plan_ball_velocity(angle: BallAngle, power: BallPower) -> Vec2<f64> {
    let scalers = power.scalers();
    let x_vel = scalers * angle.xv();
    let y_vel = scalers * angle.yv();
    return Vec2::new(x_vel, y_vel);
}

pub trait InputAdjustment {
    fn increase(&mut self, by: AdjustmentType);
    fn decrease(&mut self, by: AdjustmentType);
}
pub trait MinMaxAdj {
    fn min_increase(&mut self);
    fn max_increase(&mut self);
    fn min_decrease(&mut self);
    fn max_decrease(&mut self);
}

impl<I> InputAdjustment for I
where
    I: MinMaxAdj,
{
    fn increase(&mut self, by: AdjustmentType) {
        match by {
            AdjustmentType::Max => {
                self.max_increase();
            }
            AdjustmentType::Min => {
                self.min_increase();
            }
        }
    }
    fn decrease(&mut self, by: AdjustmentType) {
        match by {
            AdjustmentType::Max => {
                self.max_decrease();
            }
            AdjustmentType::Min => {
                self.min_decrease();
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct BallVelocity {
    vec: Vec2<f64>,
    friction: f64,
}

#[derive(Copy, Clone, Debug)]
pub enum BallState {
    Rolling,
    Stopped,
}
#[derive(Copy, Clone, Debug)]
pub struct AimLine {
    p1: Point,
    p2: Point,
}

#[derive(Clone, Debug)]
pub struct BallPower {
    pub speed: f64,
    pub power: f64,
    pub min_power_step: f64,
    pub max_power_step: f64,
}

impl MinMaxAdj for BallPower {
    fn min_increase(&mut self) {
        if self.power + self.min_power_step < POWER_MAX {
            self.power += self.min_power_step;
        }
    }
    fn max_increase(&mut self) {
        if self.power + self.max_power_step < POWER_MAX {
            self.power += self.max_power_step;
        }
    }
    fn min_decrease(&mut self) {
        if self.power - self.min_power_step > POWER_MIN {
            self.power -= self.min_power_step;
        }
    }
    fn max_decrease(&mut self) {
        if self.power - self.max_power_step > POWER_MIN {
            self.power -= self.max_power_step;
        }
    }
}

impl BallPower {
    pub fn new() -> Self {
        Self {
            speed: 0.90,
            power: 10.0,
            min_power_step: 0.5,
            max_power_step: 1.0,
        }
    }
    pub fn scalers(self) -> f64 {
        return (self.power / 4.0) * self.speed;
    }
}

#[derive(Clone, Debug)]
pub struct BallAngle {
    pub theta: f64,
    pub min_theta_step: f64,
    pub max_theta_step: f64,
}
impl MinMaxAdj for BallAngle {
    fn min_increase(&mut self) {
        self.theta += self.min_theta_step;
    }
    fn max_increase(&mut self) {
        self.theta += self.max_theta_step;
    }
    fn min_decrease(&mut self) {
        self.theta -= self.min_theta_step;
    }
    fn max_decrease(&mut self) {
        self.theta -= self.max_theta_step;
    }
}

impl BallAngle {
    pub fn new() -> Self {
        Self {
            theta: 3.60,
            min_theta_step: 0.01,
            max_theta_step: 0.05,
        }
    }
    pub fn xv(&self) -> f64 {
        return (self.theta - PI / 2.0).sin();
    }
    pub fn yv(&self) -> f64 {
        return (self.theta + PI / 2.0).cos();
    }
}
#[derive(Clone, Debug)]
pub struct Ball {
    old_point: Point,
    point: Point,
    sprite: Sprite,
    pub velocity: BallVelocity,
    pub fpos: Vec2<f64>,
    pub power: BallPower,
    pub angle: BallAngle,
}

impl BallVelocity {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            vec: Vec2::new(x, y),
            friction: 0.9980,
        }
    }
    pub fn x(&self) -> f64 {
        return self.vec.x;
    }
    pub fn y(&self) -> f64 {
        return self.vec.y;
    }
    pub fn update(&mut self, vel: Vec2<f64>) {
        self.vec = vel;
    }
    pub fn invert_x(&mut self) {
        self.vec.x = -self.vec.x;
    }
    pub fn invert_y(&mut self) {
        self.vec.y = -self.vec.y;
    }
    pub fn stop(&mut self) {
        self.vec.x = 0.0;
        self.vec.y = 0.0;
    }
    pub fn slow(&mut self) {
        self.vec.x = self.vec.x * self.friction;
        self.vec.y = self.vec.y * self.friction;
        println!(
            "SLOWWED Ball X Velocity: {} Ball Y Velocity: {}",
            self.vec.x, self.vec.y
        );
    }
    pub fn velocity_below(&self, threshold: f64) -> bool {
        return f64::abs(self.vec.x) + f64::abs(self.vec.y) < threshold;
    }
    pub fn to_map_point(self) -> Point {
        return Point::new(self.vec.x as usize, self.vec.y as usize);
    }
    pub fn vec2(&self) -> Vec2<f64> {
        return self.vec.clone();
    }
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
            velocity: BallVelocity::new(0.0, 0.0),
            fpos: Vec2::new(point.x as f64, point.y as f64),
            power: BallPower::new(),
            angle: BallAngle::new(),
        }
    }

    pub fn new() -> Self {
        return Ball::new_at_loc(100, 100);
    }

    pub fn aim_path(&self) -> AimLine {
        let ball_center = self.center();
        let theta_sin = self.angle.xv(); //f64::sin(self.theta - PI / 2.0);
        let theta_cos = self.angle.yv(); //f64::cos(self.theta + PI / 2.0);
        let x1 = ball_center.x as f64 + 0.5 + self.power.power / 5.0 * 1.0 * 5.0 * theta_sin;
        let y1 = ball_center.y as f64 + 0.5 + self.power.power / 5.0 * 1.0 * 5.0 * theta_cos;
        let x2 = ball_center.x as f64 + 0.5 + self.power.power / 5.0 * 12.0 * 5.0 * theta_sin;
        let y2 = ball_center.y as f64 + 0.5 + self.power.power / 5.0 * 12.0 * 5.0 * theta_cos;
        let line = AimLine::new(x1 as usize, y1 as usize, x2 as usize, y2 as usize);
        return line;
    }

    pub fn hit(&mut self) {
        self.velocity
            .update(plan_ball_velocity(self.angle.clone(), self.power.clone()));
    }

    pub fn roll(&mut self, playground: &GameMap) {
        self.handle_collision(playground);
        self.old_point = self.point;
        self.fpos = self.fpos + self.velocity.vec2();
        self.point = Point::new(self.fpos.x as usize, self.fpos.y as usize);
        self.velocity.slow();
        if self.velocity.velocity_below(0.1) {
            self.velocity.stop();
        }
    }
    pub fn handle_collision(&mut self, map: &GameMap) {
        if self.x_collides(map) {
            self.point.x = self.old_point.x;
            self.velocity.invert_x();
        }
        if self.y_collides(map) {
            self.point.y = self.old_point.y;
            self.velocity.invert_y();
        }
        // println!("Ball X Velocity: {} Ball Y Velocity: {}", self.vx, self.vy);
    }
    fn x_collides(&self, map: &GameMap) -> bool {
        let y_pos = (self.center().y as i32) >> 4;
        let x_upper = self.left() + 8;
        let x_lower = self.right() - 8;
        let x_pos_upper = (x_upper as i32) >> 4;
        let x_pos_lower = (x_lower as i32) >> 4;
        let x_upper_collides_wall = map
            .tile_grid
            .tile_at(x_pos_upper as usize, y_pos as usize)
            .get_type()
            == &TileType::Wall
            && self.velocity.x() > 0.0;
        let x_lower_collides_wall = map
            .tile_grid
            .tile_at(x_pos_lower as usize, y_pos as usize)
            .get_type()
            == &TileType::Wall
            && self.velocity.x() < 0.0;

        return x_upper_collides_wall || x_lower_collides_wall || !self.is_within_x_bounds(map);
    }
    fn y_collides(&self, map: &GameMap) -> bool {
        let x_pos = (self.center().x as i32) >> 4;
        let y_upper = self.top() + 8;
        let y_lower = self.bottom() - 8;
        let y_pos_upper = (y_upper as i32) >> 4;
        let y_pos_lower = (y_lower as i32) >> 4;
        let y_upper_collides_wall = map
            .tile_grid
            .tile_at(x_pos as usize, y_pos_upper as usize)
            .get_type()
            == &TileType::Wall
            && self.velocity.y() > 0.0;
        let y_lower_collides_wall = map
            .tile_grid
            .tile_at(x_pos as usize, y_pos_lower as usize)
            .get_type()
            == &TileType::Wall
            && self.velocity.y() < 0.0;
        return y_upper_collides_wall || y_lower_collides_wall || !self.is_within_y_bounds(map);
    }

    fn is_within_x_bounds(&self, playground: &GameMap) -> bool {
        if self.point.x < playground.left() || self.point.x + self.sprite.width > playground.right()
        {
            return false;
        } else {
            return true;
        }
    }
    fn is_within_y_bounds(&self, playground: &GameMap) -> bool {
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

    pub fn reset_at(&mut self, point: Point) {
        self.old_point = point.clone();
        self.point = point;
        self.velocity = BallVelocity::new(0.0, 0.0);
        self.fpos = Vec2::new(point.x as f64, point.y as f64);
    }

    pub fn draw(&self, frame: &mut [u8]) {
        blit(frame, &self.point, &self.sprite);
    }
}
