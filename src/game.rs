use crate::field::{CellPos, Field};
use bevy::prelude::*;

#[derive(Resource, Default)]
struct Game {
    field: Field,
}

#[derive(Debug, Clone)]
struct GameState {
    score: usize,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>()
            .add_systems(Startup, spawn_field)
            .add_systems(Update, render);
    }
}

fn spawn_field(game: Res<Game>, mut commands: Commands) {
    game.field.spawn(&mut commands);
}

fn render(mut game: ResMut<Game>, mut sprites: Query<(&CellPos, &mut Sprite)>) {
    game.field.render_mino();
    game.field.update_sprites(&mut sprites);
}
