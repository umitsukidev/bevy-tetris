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
        if let Some(dropping_mino) = self.dropping_mino {
            let next_mino_position = dropping_mino.position + offset;

            let next_mino_cells = dropping_mino
                .kind
                .shape()
                .map(|offset| next_mino_position + offset);

            if self.can_place(next_mino_cells) {
                self.dropping_mino.as_mut().unwrap().position = next_mino_position
            }
        }
    }

    pub fn lock_mino(&mut self) {
        if let Some(dropping_mino) = self.dropping_mino {
            for offset in dropping_mino.kind.shape() {
                let mino_cell = dropping_mino.position + offset;

                self.cells[mino_cell.y as usize][mino_cell.x as usize] = Some(Cell::Piece {
                    color: dropping_mino.kind.color(),
                });
            }
        }
        self.dropping_mino = None;
    }

    pub fn hard_drop_mino(&mut self) {
        if self.dropping_mino.is_none() {
            return;
        }

        while !self.must_lock_mino() {
            self.move_mino(ivec2(0, 1));
        }

        self.lock_mino();
    }

    fn can_place(&self, mino_cells: [IVec2; 4]) -> bool {
        mino_cells.iter().all(|mino_cell| {
            (0 <= mino_cell.x && mino_cell.x < WIDTH as i32)
                && (0 <= mino_cell.y && mino_cell.y < HEIGHT as i32)
                && self.cells[mino_cell.y as usize][mino_cell.x as usize].is_none()
        })
    }

    pub fn must_lock_mino(&self) -> bool {
        if let Some(dropping_mino) = self.dropping_mino {
            let next_mino_position = dropping_mino.position + ivec2(0, 1);

            let next_mino_cells = dropping_mino
                .kind
                .shape()
                .map(|offset| next_mino_position + offset);

            !self.can_place(next_mino_cells)
        } else {
            false
        }
    }

    pub fn needs_mino(&self) -> bool {
        self.dropping_mino.is_none()
    }

    pub fn spawn_mino(&mut self, kind: Tetrimino) {
        if self.dropping_mino.is_none() {
            self.dropping_mino = Some(DroppingMino {
                kind,
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
