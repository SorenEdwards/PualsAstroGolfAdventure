pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    #[inline(always)]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// A tiny position vector.
#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

/// A tiny rectangle based on two absolute `Point`s.
#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Rect {
    pub(crate) p1: Point,
    pub(crate) p2: Point,
}

impl Point {
    /// Create a new point.
    pub(crate) const fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl core::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl core::ops::Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}