use crate::{
    field::{CellPos, Field},
    tetrimino::Tetrimino,
};
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::{rng, seq::SliceRandom};
use std::time::Duration;

#[derive(Resource, Default)]
struct Game {
    field: Field,
}

#[derive(Resource, Debug, Clone)]
struct GameState {
    _score: usize,
    mino_index: usize,
    current_tetriminos: [Tetrimino; 7],
    next_tetriminos: [Tetrimino; 7],
}

impl Default for GameState {
    fn default() -> Self {
        let bag = [
            Tetrimino::I,
            Tetrimino::O,
            Tetrimino::T,
            Tetrimino::S,
            Tetrimino::Z,
            Tetrimino::L,
            Tetrimino::J,
        ];

        let mut rng = rng();

        let mut current_tetriminos = bag;
        let mut next_tetriminos = bag;

        current_tetriminos.shuffle(&mut rng);
        next_tetriminos.shuffle(&mut rng);

        Self {
            _score: 0,
            mino_index: 0,
            current_tetriminos,
            next_tetriminos,
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0))
            .init_resource::<Game>()
            .init_resource::<GameState>()
            .add_systems(Startup, spawn_field)
            .add_systems(
                Update,
                (
                    handle_input,
                    gravity.run_if(on_timer(Duration::from_millis(750))),
                    supply_mino,
                    render,
                )
                    .chain(),
            );
    }
}

fn spawn_field(game: Res<Game>, mut commands: Commands) {
    game.field.spawn_sprites(&mut commands);
}

fn render(game: Res<Game>, mut sprites: Query<(&CellPos, &mut Sprite)>) {
    game.field.update_sprites(&mut sprites);
}

fn supply_mino(mut game: ResMut<Game>, mut state: ResMut<GameState>) {
    if game.field.needs_mino() {
        let current_mino = state.current_tetriminos[state.mino_index];
        game.field.spawn_mino(current_mino);
        if state.mino_index < 6 {
            state.mino_index += 1;
        } else {
            state.mino_index = 0;

            let state = state.into_inner();
            std::mem::swap(&mut state.current_tetriminos, &mut state.next_tetriminos);

            state.next_tetriminos.shuffle(&mut rng());
        }
    }
}

fn handle_input(mut game: ResMut<Game>, keyboard: Res<ButtonInput<KeyCode>>) {
    for &key in keyboard.get_just_pressed() {
        match key {
            KeyCode::ArrowLeft => game.field.move_mino(ivec2(-1, 0)),
            KeyCode::ArrowRight => game.field.move_mino(ivec2(1, 0)),
            KeyCode::ArrowDown => game.field.move_mino(ivec2(0, 1)),
            KeyCode::ArrowUp => game.field.hard_drop_mino(),
            _ => {}
        }
    }
}

fn gravity(mut game: ResMut<Game>) {
    if game.field.must_lock_mino() {
        game.field.lock_mino();
    }
    game.field.move_mino(ivec2(0, 1));
}
