use crate::{cell::Cell, direction::Direction, dropping_mino::DroppingMino, tetrimino::Tetrimino};
use bevy::{color::palettes::css::DARK_GRAY, prelude::*};

const WIDTH: usize = 10;
const HEIGHT: usize = 22;

#[derive(Debug, Clone, Default)]
pub struct Field {
    cells: [[Option<Cell>; WIDTH]; HEIGHT],
    dropping_mino: Option<DroppingMino>,
}

impl Field {
    pub fn move_mino(&mut self, offset: IVec2) {
        if let Some(dropping_mino) = &mut self.dropping_mino {
            dropping_mino.position += offset;
        }
    }

    pub fn spawn_mino(&mut self) {
        if self.dropping_mino.is_none() {
            self.dropping_mino = Some(DroppingMino {
                kind: Tetrimino::I,
                position: ivec2(3, 0),
                face: Direction::North,
            })
        }
    }

    pub fn spawn_sprites(&self, commands: &mut Commands) {
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
                None => {
                    if let Some(mino) = self.dropping_mino
                        && mino.kind.shape().into_iter().any(|offset| {
                            mino.position + offset == ivec2(pos.column as i32, pos.row as i32)
                        })
                    {
                        mino.kind.color()
                    } else {
                        DARK_GRAY.into()
                    }
                }
            };
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct CellPos {
    column: usize,
    row: usize,
}
