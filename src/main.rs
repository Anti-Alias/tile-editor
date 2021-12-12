use wgpu::{TextureFormat};
use tile_editor::app::{App, AppState, AppEvent};
use tile_editor::graphics::scene::Scene;

fn main() {
    let mut app = App::new()
        .title("Tile Editor")
        .size(1280, 720)
        .gui_enabled(false)
        .event_handler(|event, state| {
            match event {
                AppEvent::STARTED => { on_start(state.scene) }
                AppEvent::STOPPED => { on_stop(state.scene) }
                AppEvent::RESIZED { width, height } => { on_resize(width, height, state.scene) }
                _ => {}
            }
        });
    app.start();
}

fn on_start(scene: &mut Scene) {
    log::info!("Application started!")
}

fn on_stop(scene: &mut Scene) {
    log::info!("Application stopped!")
}

fn on_resize(width: u32, height: u32, scene: &mut Scene) {
    log::info!("Application resized to dimensions: {}x{}", width, height)
}