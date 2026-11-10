use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Piece { color: Color },
}
