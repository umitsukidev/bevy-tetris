use crate::{dropping_mino::DroppingMino, field::Field};
use bevy::prelude::*;

#[derive(Resource, Default)]
struct Game {
    field: Field,
}

#[derive(Debug, Clone)]
struct GameState {
    score: usize,
    dropping_mino: DroppingMino,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>()
            .add_systems(Startup, spawn_field);
    }
}

fn spawn_field(game: Res<Game>, mut commands: Commands) {
    game.field.spawn(&mut commands);
}
