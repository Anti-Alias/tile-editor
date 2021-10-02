use egui::CtxRef;
use egui_wgpu_backend::epi::Frame;

pub struct GUI {

}

impl GUI {
    pub fn new() -> GUI {
        GUI {}
    }
}

impl epi::App for GUI {

    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {

    }

    fn name(&self) -> &str {
        "Tile Editor"
    }
}