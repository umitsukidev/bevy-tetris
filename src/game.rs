use crate::{dropping_mino::DroppingMino, field::Field};
use bevy::prelude::*;

#[derive(Resource)]
struct Game {
    field: Field,
}

#[derive(Debug, Clone)]
struct GameState {
    score: usize,
    dropping_mino: DroppingMino,
}
