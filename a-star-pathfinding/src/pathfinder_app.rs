use crate::astar;
use crate::{grid_data::GridData, terrain_type::TerrainType};
use eframe::egui;
use strum::IntoEnumIterator;

const WINDOW_WIDTH: f32 = 850.0;
const WINDOW_HEIGHT: f32 = 650.0;

pub struct PathFinderApp {
    grid: GridData,
    selected_terrain: TerrainType,
    current_path: Option<Vec<(usize, usize)>>,
}

impl PathFinderApp {
    pub fn new() -> Self {
        Self {
            grid: GridData::new(),
            selected_terrain: TerrainType::Start,
            current_path: None,
        }
    }

    pub fn run() -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
                .with_title("A* Pathfinder"),
            ..Default::default()
        };

        eframe::run_native(
            "A* Pathfinder",
            options,
            Box::new(|_cc| Ok(Box::new(Self::new()))),
        )
    }
}

impl eframe::App for PathFinderApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.draw_tools_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            let cell_size = 20.0;

            let grid_size = egui::vec2(
                self.grid.get_width() as f32 * cell_size,
                self.grid.get_height() as f32 * cell_size,
            );

            let (response, painter) = ui.allocate_painter(grid_size, egui::Sense::click_and_drag());

            let border = egui::Stroke::new(1.0, egui::Color32::from_gray(200));

            for y in 0..self.grid.get_height() {
                for x in 0..self.grid.get_width() {
                    let min_pos =
                        response.rect.min + egui::vec2(x as f32 * cell_size, y as f32 * cell_size);
                    let max_pos = min_pos + egui::vec2(cell_size, cell_size);
                    let cell_rect = egui::Rect::from_min_max(min_pos, max_pos);

                    let terrain = self.grid.get_terrain_type(x, y);

                    painter.rect(
                        cell_rect,
                        0.0,
                        terrain_type_to_color(&terrain),
                        border,
                        egui::StrokeKind::Outside,
                    );
                }
            }

            if let Some(path) = &self.current_path {
                let mut screen_points = Vec::with_capacity(path.len());

                for (grid_x, grid_y) in path {
                    let min_pos = response.rect.min
                        + egui::vec2(*grid_x as f32 * cell_size, *grid_y as f32 * cell_size);
                    let center_pos = min_pos + egui::vec2(cell_size / 2.0, cell_size / 2.0);
                    screen_points.push(center_pos);
                }

                let path_stroke = egui::Stroke::new(3.0, egui::Color32::from_rgb(50, 150, 255));
                painter.line(screen_points, path_stroke);
            }

            // paint the grid
            if response.is_pointer_button_down_on() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    // mouse position
                    let local_x = pointer_pos.x - response.rect.min.x;
                    let local_y = pointer_pos.y - response.rect.min.y;

                    // find exact cell
                    let grid_x = (local_x / cell_size) as usize;
                    let grid_y = (local_y / cell_size) as usize;

                    if grid_x < self.grid.get_width() && grid_y < self.grid.get_height() {
                        self.grid.set_terrain(grid_x, grid_y, self.selected_terrain);
                    }

                    // grid changed, recomputation needed
                    self.current_path = None;
                }
            }
        });
    }
}

// ============ private helpers ============
impl PathFinderApp {
    fn draw_tools_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("tools_panel").show(ctx, |ui| {
            ui.label("Select Terrain to Paint:");

            for tt in TerrainType::iter() {
                ui.radio_value(&mut self.selected_terrain, tt, tt.to_string());
            }

            ui.separator();
            if ui.button("Find Path").clicked() {
                self.current_path = astar::find_path(&self.grid);
            }
            if ui.button("Clear").clicked() {
                self.current_path = None;
                self.grid.clear();
            }
        });
    }
}

fn terrain_type_to_color(terrain: &TerrainType) -> egui::Color32 {
    match terrain {
        TerrainType::Start => egui::Color32::YELLOW,
        TerrainType::Finish => egui::Color32::RED,
        TerrainType::Empty => egui::Color32::WHITE,
        TerrainType::Water => egui::Color32::BLUE,
        TerrainType::Swamp => egui::Color32::BROWN,
        TerrainType::Wall => egui::Color32::BLACK,
    }
}
