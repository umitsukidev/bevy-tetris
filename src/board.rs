use crate::cell::Cell;
use bevy::prelude::*;

#[derive(Resource, Default, Clone)]
pub struct Board {
    cells: [[Option<Cell>; 10]; 20],
}
