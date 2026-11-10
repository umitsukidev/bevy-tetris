use crate::{direction::Direction, tetrimino::Tetrimino};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct DroppingMino {
    pub kind: Tetrimino,
    pub position: IVec2,
    pub face: Direction,
}
