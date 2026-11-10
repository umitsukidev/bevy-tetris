use crate::cell::Cell;
use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Field {
    cells: [[Option<Cell>; 10]; 20],
}
