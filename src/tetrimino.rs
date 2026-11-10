use bevy::{
    color::palettes::css::{BLUE, ORANGE, PURPLE, RED, SKY_BLUE, YELLOW, YELLOW_GREEN},
    prelude::*,
};

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
    pub fn shape(self) -> [IVec2; 4] {
        match self {
            Self::I => [ivec2(0, 1), ivec2(1, 1), ivec2(2, 1), ivec2(3, 1)],
            Self::O => [ivec2(1, 0), ivec2(2, 0), ivec2(1, 1), ivec2(2, 1)],
            Self::T => [ivec2(1, 0), ivec2(0, 1), ivec2(1, 1), ivec2(2, 1)],
            Self::S => [ivec2(1, 0), ivec2(2, 0), ivec2(0, 1), ivec2(1, 1)],
            Self::Z => [ivec2(0, 0), ivec2(1, 0), ivec2(1, 1), ivec2(2, 1)],
            Self::L => [ivec2(2, 0), ivec2(0, 1), ivec2(1, 1), ivec2(2, 1)],
            Self::J => [ivec2(0, 0), ivec2(0, 1), ivec2(1, 1), ivec2(2, 1)],
        }
    }

    pub fn color(self) -> Color {
        match self {
            Self::I => SKY_BLUE.into(),
            Self::O => YELLOW.into(),
            Self::T => PURPLE.into(),
            Self::S => YELLOW_GREEN.into(),
            Self::Z => RED.into(),
            Self::L => ORANGE.into(),
            Self::J => BLUE.into(),
        }
    }
}
