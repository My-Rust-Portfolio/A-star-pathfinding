mod grid_data;
mod pathfinder_app;
mod terrain_type;

fn main() -> eframe::Result<()> {
    pathfinder_app::PathFinderApp::run()
}
