use crate::terrain_type::TerrainType;

const GRID_WIDTH: usize = 30;
const GRID_HEIGHT: usize = 30;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;

pub struct GridData {
    start_point: Option<(usize, usize)>,
    end_point: Option<(usize, usize)>,
    width: usize,
    height: usize,
    terrain: Vec<TerrainType>,
}

impl GridData {
    pub fn new() -> Self {
        Self {
            start_point: None,
            end_point: None,
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            terrain: vec![TerrainType::Empty; GRID_SIZE],
        }
    }

    pub fn clear(&mut self) {
        self.terrain.fill(TerrainType::Empty);
        self.start_point = None;
        self.end_point = None;
    }

    pub fn set_terrain(&mut self, x: usize, y: usize, terrain: TerrainType) {
        if terrain == TerrainType::Start {
            if let Some((old_x, old_y)) = self.start_point {
                // in case it was overriden before
                if self.terrain[old_y * GRID_WIDTH + old_x] == TerrainType::Start {
                    self.terrain[old_y * GRID_WIDTH + old_x] = TerrainType::Empty;
                }
            }
            self.start_point = Some((x, y));
        } else if terrain == TerrainType::Finish {
            if let Some((old_x, old_y)) = self.end_point {
                // in case it was overriden before
                if self.terrain[old_y * GRID_WIDTH + old_x] == TerrainType::Finish {
                    self.terrain[old_y * GRID_WIDTH + old_x] = TerrainType::Empty;
                }
            }
            self.end_point = Some((x, y));
        }

        self.terrain[y * GRID_WIDTH + x] = terrain;
    }

    pub fn get_terrain_type(&self, x: usize, y: usize) -> TerrainType {
        self.terrain[y * GRID_WIDTH + x]
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
}
