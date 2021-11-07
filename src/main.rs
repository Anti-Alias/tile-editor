use wgpu::{TextureFormat};
use tile_editor::app::App;

fn main() {
    App::new()
        .title("Tile Editor")
        .size(640, 480)
        .depth_stencil_format(TextureFormat::Depth32Float)
        .start();
}