use strum::{Display, EnumIter};

#[derive(Copy, Clone, PartialEq, Eq, EnumIter, Display)]

pub enum TerrainType {
    Start,
    Finish,
    Empty,
    Water,
    Swamp,
    Wall,
}

impl TerrainType {
    pub fn cost(&self) -> usize {
        match self {
            TerrainType::Start => 0,
            TerrainType::Finish => 1,
            TerrainType::Empty => 1,
            TerrainType::Water => 2,
            TerrainType::Swamp => 3,
            TerrainType::Wall => usize::MAX,
        }
    }
}
