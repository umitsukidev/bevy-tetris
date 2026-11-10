use crate::{
    cell::Cell, direction::Direction::North, dropping_mino::DroppingMino, tetrimino::Tetrimino,
};
use bevy::{color::palettes::css::DARK_GRAY, prelude::*};

const WIDTH: usize = 10;
const HEIGHT: usize = 22;

#[derive(Debug, Clone, Default)]
pub struct Field {
    cells: [[Option<Cell>; WIDTH]; HEIGHT],
    dropping_mino: Option<DroppingMino>,
}

impl Field {
    pub fn render_mino(&mut self) {
        if self.dropping_mino.is_none() {
            self.dropping_mino = Some(DroppingMino {
                kind: Tetrimino::I,
                position: vec2(3.0, 0.0),
                face: North,
            })
        }

        if let Some(dropping_mino) = self.dropping_mino {
            for piece in dropping_mino.kind.shape() {
                let pos = piece + dropping_mino.position;
                let (column, row) = (pos.x as usize, pos.y as usize);
                self.cells[row][column] = Some(Cell::Piece {
                    color: dropping_mino.kind.color(),
                    is_dropping: true,
                });
            }
        }
    }

    pub fn spawn(&self, commands: &mut Commands) {
        for row in 0..HEIGHT {
            for column in 0..WIDTH {
                commands.spawn((
                    Sprite::from_color(
                        match self.cells[row][column] {
                            Some(Cell::Piece { color, .. }) => color,
                            None => DARK_GRAY.into(),
                        },
                        vec2(25.0, 25.0),
                    ),
                    Transform::from_xyz(
                        (column as f32 + 0.5 - WIDTH as f32 / 2.0) * 30.0,
                        (HEIGHT as f32 / 2.0 - row as f32 - 0.5) * 30.0,
                        0.0,
                    ),
                    CellPos { column, row },
                ));
            }
        }
    }

    pub fn update_sprites(&self, sprites: &mut Query<(&CellPos, &mut Sprite)>) {
        for (pos, mut sprite) in sprites.iter_mut() {
            sprite.color = match self.cells[pos.row][pos.column] {
                Some(Cell::Piece { color, .. }) => color,
                None => DARK_GRAY.into(),
            };
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct CellPos {
    column: usize,
    row: usize,
}
