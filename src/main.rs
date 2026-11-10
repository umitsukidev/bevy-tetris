use crate::board::Board;
use bevy::prelude::*;

mod board;
mod cell;
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
        .init_resource::<Board>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
