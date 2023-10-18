use crate::common::*;
use crate::drawing::*;
use core::time::Duration;
use std::rc::Rc;
#[derive(Clone, Debug)]
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}


#[derive(Clone, Debug)]
pub struct SpriteRef {
    width: usize,
    height: usize,
    pixels: Rc<[u8]>,
    frame: Frame,
    duration: Duration,
    dt: Duration,
}

impl Drawable for Sprite {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

impl Drawable for SpriteRef {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}
