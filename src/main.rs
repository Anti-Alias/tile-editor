use wgpu::{TextureFormat};
use tile_editor::app::App;

fn main() {
    App::new()
        .title("Tile Editor")
        .size(1280, 720)
        .depth_stencil_format(TextureFormat::Depth32Float)
        .gui_enabled(false)
        .input_handler(|app| {})
        .start();
}