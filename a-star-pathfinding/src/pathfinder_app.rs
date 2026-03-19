use crate::{grid_data::GridData, terrain_type::TerrainType};
use eframe::egui;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 800.0;

pub struct PathFinderApp {
    grid: GridData,
    texture: Option<egui::TextureHandle>,
}

impl PathFinderApp {
    pub fn new() -> Self {
        Self {
            grid: GridData::new(),
            texture: None,
        }
    }
}

impl PathFinderApp {
    pub fn run() -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
                .with_title("A Star Pathfinder"),
            ..Default::default()
        };

        eframe::run_native(
            "A Star Pathfinder",
            options,
            Box::new(|_cc| Ok(Box::new(Self::new()))),
        )
    }
}

impl eframe::App for PathFinderApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.texture == None {
            self.update_texture(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                let display_size = egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT);

                let response = ui.add(egui::Image::new(texture));
            }
        });
    }
}

// ============ private helpers ============
impl PathFinderApp {
    fn update_texture(&mut self, ctx: &egui::Context) {
        let pixels: Vec<egui::Color32> = self
            .grid
            .get_terrain()
            .iter()
            .map(|t| terrain_type_to_color(t))
            .collect();

        let image = egui::ColorImage::new(self.grid.get_grid_dimensions(), pixels);

        if let Some(texture) = &mut self.texture {
            texture.set(image, egui::TextureOptions::NEAREST);
        } else {
            self.texture =
                Some(ctx.load_texture("grid_texture", image, egui::TextureOptions::NEAREST));
        }
    }
}

fn terrain_type_to_color(terrain: &TerrainType) -> egui::Color32 {
    match terrain {
        TerrainType::Empty => egui::Color32::WHITE,
        TerrainType::Wall => egui::Color32::BLACK,
    }
}
