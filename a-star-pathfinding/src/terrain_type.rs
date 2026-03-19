use strum::{Display, EnumIter};

#[derive(Copy, Clone, PartialEq, Eq, EnumIter, Display)]

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
