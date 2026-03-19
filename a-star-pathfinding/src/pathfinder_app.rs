use crate::{grid_data::GridData, terrain_type::TerrainType};
use eframe::egui;

const WINDOW_WIDTH: f32 = 850.0;
const WINDOW_HEIGHT: f32 = 650.0;

pub struct PathFinderApp {
    grid: GridData,
    selected_terrain: TerrainType,
}

impl PathFinderApp {
    pub fn new() -> Self {
        Self {
            grid: GridData::new(),
            selected_terrain: TerrainType::Empty,
        }
    }
}

impl PathFinderApp {
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("tools_panel").show(ctx, |ui| {
            ui.heading("A* Pathfinding");
            ui.separator();

            ui.label("Select Terrain to Paint:");
            ui.radio_value(&mut self.selected_terrain, TerrainType::Empty, "Empty");
            ui.radio_value(&mut self.selected_terrain, TerrainType::Wall, "Wall");

            ui.separator();
            if ui.button("Clear").clicked() {
                self.grid.clear();
            }
        });

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
                }
            }
        });
    }
}

// ============ private helpers ============

fn terrain_type_to_color(terrain: &TerrainType) -> egui::Color32 {
    match terrain {
        TerrainType::Empty => egui::Color32::WHITE,
        TerrainType::Wall => egui::Color32::BLACK,
    }
}
