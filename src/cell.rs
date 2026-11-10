use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Piece { color: Color, is_dropping: bool },
}
