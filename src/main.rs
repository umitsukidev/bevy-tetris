use bevy::prelude::*;

mod cell;
mod direction;
mod dropping_mino;
mod field;
mod game;
mod tetrimino;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".into(),
                resolution: (600, 800).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
