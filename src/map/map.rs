use crate::entities::tiles::*;
use crate::geo::*;

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
