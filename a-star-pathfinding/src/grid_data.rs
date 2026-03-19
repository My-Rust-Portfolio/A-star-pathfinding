use crate::terrain_type::TerrainType;

const GRID_WIDTH: usize = 30;
const GRID_HEIGHT: usize = 30;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;

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

    pub fn set_terrain(&mut self, x: usize, y: usize, terrain: TerrainType) {
        self.terrain[y * GRID_WIDTH + x] = terrain;
    }

    pub fn get_terrain_type(&self, x: usize, y: usize) -> TerrainType {
        self.terrain[y * GRID_WIDTH + x]
    }

    pub fn get_terrain(&self) -> &Vec<TerrainType> {
        &self.terrain
    }

    pub fn get_grid_width(&self) -> usize {
        self.width
    }
    pub fn get_grid_height(&self) -> usize {
        self.height
    }
}
