use crate::field::{CellPos, Field};
use bevy::{prelude::*, time::common_conditions::on_timer};
use std::time::Duration;

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
        app.insert_resource(Time::<Fixed>::from_hz(60.0))
            .init_resource::<Game>()
            .add_systems(Startup, spawn_field)
            .add_systems(
                Update,
                (
                    handle_input,
                    gravity.run_if(on_timer(Duration::from_millis(750))),
                    render,
                )
                    .chain(),
            );
    }
}

fn spawn_field(game: Res<Game>, mut commands: Commands) {
    game.field.spawn_sprites(&mut commands);
}

fn render(mut game: ResMut<Game>, mut sprites: Query<(&CellPos, &mut Sprite)>) {
    game.field.spawn_mino();
    game.field.update_sprites(&mut sprites);
}

fn handle_input(mut game: ResMut<Game>, keyboard: Res<ButtonInput<KeyCode>>) {
    for &key in keyboard.get_just_pressed() {
        match key {
            KeyCode::ArrowLeft => game.field.move_mino(ivec2(-1, 0)),
            KeyCode::ArrowRight => game.field.move_mino(ivec2(1, 0)),
            _ => {}
        }
    }
}

fn gravity(mut game: ResMut<Game>) {
    game.field.move_mino(ivec2(0, 1))
}
