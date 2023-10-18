use crate::ball::*;
use std::rc::Rc;
pub struct InfoScreen {
    ball: Ball
}

impl InfoScreen {
    pub fn new() -> Self{
        Self {
            ball: Ball::new(),
        }
    }
}