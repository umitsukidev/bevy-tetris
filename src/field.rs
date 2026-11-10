use crate::cell::Cell;
use bevy::{color::palettes::css::DARK_GRAY, prelude::*};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Debug, Clone, Default)]
pub struct Field {
    cells: [[Option<Cell>; WIDTH]; HEIGHT],
}

impl Field {
    pub fn spawn(&self, commands: &mut Commands) {
        for row in 0..HEIGHT {
            for column in 0..WIDTH {
                commands.spawn((
                    Sprite::from_color(
                        match self.cells[row][column] {
                            Some(Cell::Piece { color }) => color,
                            None => DARK_GRAY.into(),
                        },
                        vec2(25.0, 25.0),
                    ),
                    Transform::from_xyz(
                        (column as f32 + 0.5 - WIDTH as f32 / 2.0) * 30.0,
                        (HEIGHT as f32 / 2.0 - row as f32 - 0.5) * 30.0,
                        0.0,
                    ),
                    CellPos { row, column },
                ));
            }
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct CellPos {
    column: usize,
    row: usize,
}
