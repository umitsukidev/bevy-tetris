use crate::{direction::Direction, tetrimino::Tetrimino};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct DroppingMino {
    pub kind: Tetrimino,
    pub position: Vec2,
    pub face: Direction,
}
