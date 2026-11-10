use crate::{direction::Direction, tetrimino::Tetrimino};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct DroppingMino {
    kind: Tetrimino,
    position: Vec2,
    face: Direction,
}
