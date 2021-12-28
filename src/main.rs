use tile_editor::app::App;
use tile_editor::gui::{GUI, VoxelSetEditor};

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;
const CAM_NEAR: f32 = 2.0;
const CAM_FAR: f32 = 8000.0;
const CAM_PERSPECTIVE_SCALE: f32 = (1.0/200.0) as f32;

fn main() {
    let mut app = App::new();
    app
        .title("GUI Example")
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .gui(GUI::new_with_editor(VoxelSetEditor::new("test_voxel_editor"), "test_voxel_editor"))
    ;
    app.start();
}