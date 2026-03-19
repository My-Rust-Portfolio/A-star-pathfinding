use crate::terrain_type::TerrainType;

const GRID_WIDTH: usize = 600;
const GRID_HEIGHT: usize = 600;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;

pub struct Coordinate {
    x: usize,
    y: usize,
}

pub struct GridData {
    width: usize,
    height: usize,
    terrain: Vec<TerrainType>,
}

impl GridData {
    pub fn new() -> Self {
        Self {
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            terrain: vec![TerrainType::Empty; GRID_SIZE],
        }
    }

    pub fn set_terrain(&mut self, p: Coordinate, terrain: TerrainType) {
        self.terrain[p.y * GRID_WIDTH + p.x] = terrain;
    }

    pub fn get_terrain_type(&self, p: Coordinate) -> TerrainType {
        self.terrain[p.y * GRID_WIDTH + p.x]
    }

    pub fn get_terrain(&self) -> &Vec<TerrainType> {
        &self.terrain
    }

    pub fn get_grid_dimensions(&self) -> [usize; 2] {
        [self.width, self.height]
    }
}
