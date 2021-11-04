use wgpu::{TextureFormat};
use tile_editor::app::App;

fn main() {
    App::new()
        .title("Tile Editor")
        .size(1920, 1080)
        .depth_stencil_format(TextureFormat::Depth32Float)
        .start();
}