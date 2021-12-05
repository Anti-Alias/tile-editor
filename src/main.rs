use wgpu::{TextureFormat};
use tile_editor::app::App;

fn main() {
    App::new()
        .title("Tile Editor")
        .size(1280, 720)
        .gui_enabled(false)
        .input_handler(|_app| {})
        .start();
}