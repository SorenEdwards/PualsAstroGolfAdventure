use crate::controls::*;
use crate::game::*;



pub trait GameSectionState {}
impl GameSectionState for GameState {}

// pub trait StateDriven for S Where S: GameSectionState {
//     fn update_controls(&mut self, controls: &Controls);
// }