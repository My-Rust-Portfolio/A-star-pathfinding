#[derive(Copy, Clone, PartialEq, Eq)]

pub enum TerrainType {
    Empty,
    Wall,
}

impl TerrainType {
    pub fn cost(&self) -> usize {
        match self {
            TerrainType::Empty => 1,
            TerrainType::Wall => usize::MAX,
        }
    }
}
