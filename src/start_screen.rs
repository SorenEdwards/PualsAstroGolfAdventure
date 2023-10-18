use crate::drawing::*;
use crate::menus::*;



pub struct StartMenu {
    state: MenuOptions,
    width: usize,
    height: usize,
    pixels: [u8;1024000]
}

impl StartMenu {
    pub fn new() -> Self {
        Self {
            state: MenuOptions::NoSelection,
            width: 640,
            height: 400,
            pixels: [128;1024000]
        }
    }
}

impl Drawable for StartMenu {
    fn width(&self) -> usize {
        return self.width;
    }
    fn height(&self) -> usize {
        return self.height;
    }
    fn pixels(&self) -> &[u8] {
        return &self.pixels;
    }
}