use tile_editor::app::App;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;
const CAM_NEAR: f32 = 2.0;
const CAM_FAR: f32 = 8000.0;
const CAM_PERSPECTIVE_SCALE: f32 = (1.0/200.0) as f32;

fn main() {
    let mut app = App::new();
    app
        .title("GUI Example")
        .size(SCREEN_WIDTH, SCREEN_HEIGHT);
    app.start();
}