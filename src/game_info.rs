use crate::entities::ball::*;
pub struct InfoScreen {
    ball: Ball,
}

impl InfoScreen {
    pub fn new() -> Self {
        Self { ball: Ball::new() }
    }
}
