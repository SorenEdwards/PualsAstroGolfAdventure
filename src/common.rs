use std::rc::Rc;
pub(crate) type CachedSprite = (usize, usize, Rc<[u8]>);
pub const BALL_WIDTH: usize = 10;
pub const BALL_HEIGHT: usize = 8;

// const WIDTH: u32 = 500;
// const HEIGHT: u32 = 500;

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 400;

pub const TILE_SIZE: usize = 256;
pub const WALL_TILE_ASSET: [u8; TILE_SIZE] = [1; TILE_SIZE];
pub const GROUND_TILE_ASSET: [u8; TILE_SIZE] = [0; TILE_SIZE];
pub const HOLE_TILE_ASSET: [u8; TILE_SIZE] = [3; TILE_SIZE];
pub const PORTAL_TILE_ASSET: [u8; TILE_SIZE] = [2; TILE_SIZE];

pub const BALL_SIZE: usize = 64;
pub const BALL_ASSET: [u8; BALL_SIZE] = [
    0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1,
    1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0,
];

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
pub(crate) enum Frame {
    Ball,
    Wall,
    Hole,
}
