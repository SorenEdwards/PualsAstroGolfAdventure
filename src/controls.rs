#[derive(Clone, Debug, Default)]
pub struct Controls {
    pub aiming: Direction,
    pub power: PowerLevel,
    pub adj: AdjustmentType,
    pub hit: bool,
}

#[derive(Clone, Debug, Default)]
pub enum AdjustmentType {
    #[default]
    Min,
    Max,
}

/// The player can only move left or right, but can also be stationary.
#[derive(Clone, Debug, Default)]
pub enum Direction {
    /// Do not move the player.
    #[default]
    Still,
    /// Move to the left.
    Left,
    /// Move to the right.
    Right,
}

#[derive(Clone, Debug, Default)]
pub enum PowerLevel {
    /// Do not move the player.
    #[default]
    Same,
    /// Move to the left.
    Up,
    /// Move to the right.
    Down,
}
