use crate::geo::*;
use crate::drawing::*;
#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Rect {
    pub(crate) p1: Point,
    pub(crate) p2: Point,
}

impl Rect {
    /// Create a rectangle from two `Point`s.
    pub(crate) fn new(p1: &Point, p2: &Point) -> Rect {
        let p1 = *p1;
        let p2 = *p2;

        Rect { p1, p2 }
    }

    /// Create a rectangle from a `Point` and a `Drawable`.
    pub(crate) fn from_drawable<D>(pos: &Point, drawable: &D) -> Rect
    where
        D: Drawable,
    {
        let p1 = *pos;
        let p2 = p1 + Point::new(drawable.width(), drawable.height());

        Rect { p1, p2 }
    }

    /// Test for intersections between two rectangles.
    ///
    /// Rectangles intersect when the geometry of either overlaps.
    pub(crate) fn intersects(&self, other: &Rect) -> bool {
        let (top1, right1, bottom1, left1) = self.get_bounds();
        let (top2, right2, bottom2, left2) = other.get_bounds();

        bottom1 > top2 && bottom2 > top1 && right1 > left2 && right2 > left1
    }

    /// Compute the bounding box for this rectangle.
    ///
    /// # Returns
    ///
    /// Tuple of `(top, right, bottom, left)`, e.g. in CSS clockwise order.
    fn get_bounds(&self) -> (usize, usize, usize, usize) {
        (self.p1.y, self.p2.x, self.p2.y, self.p1.x)
    }
}
