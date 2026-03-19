use strum::{Display, EnumIter};

#[derive(Copy, Clone, PartialEq, Eq, EnumIter, Display)]

pub enum TerrainType {
    Empty,
    Water,
    Swamp,
    Wall,
}

impl TerrainType {
    pub fn cost(&self) -> usize {
        match self {
            TerrainType::Empty => 1,
            TerrainType::Water => 2,
            TerrainType::Swamp => 3,
            TerrainType::Wall => usize::MAX,
        }
    }
}
