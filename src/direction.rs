#[derive(Debug, Clone, Copy, Default)]
pub enum Direction {
    East,
    West,
    South,
    #[default]
    North,
}
