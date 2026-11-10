use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tetrimino {
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
}

impl Tetrimino {
    pub fn shape(self) -> [Vec2; 4] {
        match self {
            Self::I => [
                vec2(0.0, 1.0),
                vec2(1.0, 1.0),
                vec2(2.0, 1.0),
                vec2(3.0, 1.0),
            ],
            Self::O => [
                vec2(0.0, 0.0),
                vec2(0.0, 1.0),
                vec2(1.0, 0.0),
                vec2(1.0, 1.0),
            ],
            Self::T => [
                vec2(1.0, 0.0),
                vec2(0.0, 1.0),
                vec2(1.0, 1.0),
                vec2(2.0, 1.0),
            ],
            Self::S => [
                vec2(1.0, 0.0),
                vec2(2.0, 0.0),
                vec2(0.0, 1.0),
                vec2(1.0, 1.0),
            ],
            Self::Z => [
                vec2(0.0, 0.0),
                vec2(1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(2.0, 1.0),
            ],
            Self::L => [
                vec2(2.0, 0.0),
                vec2(0.0, 1.0),
                vec2(1.0, 1.0),
                vec2(2.0, 1.0),
            ],
            Self::J => [
                vec2(0.0, 0.0),
                vec2(0.0, 1.0),
                vec2(1.0, 1.0),
                vec2(2.0, 1.0),
            ],
        }
    }
}
