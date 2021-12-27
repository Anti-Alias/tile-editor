use epi::egui::{CtxRef, SidePanel, TopBottomPanel};
use epi::TextureAllocator;
use crate::gui::Editor;

pub struct MapEditor {
    pub name: String,
    pub content: String
}

impl Editor for MapEditor {
    fn show(&mut self, ctx: &CtxRef, tex_alloc: &mut dyn TextureAllocator) {
        self.left_panel(ctx);
        self.right_panel(ctx);
        self.bottom_panel(ctx);
        self.content_panel(ctx);
    }
}

impl MapEditor {

    pub fn new(name: &str, content: &str) -> MapEditor {
        MapEditor {
            name: name.to_owned(),
            content: content.to_owned()
        }
    }

    fn left_panel(&self, ctx: &CtxRef) {
        SidePanel::left("left").resizable(false).show(ctx, |ui| {
            ui.label("Left Editor");
        });
    }

    fn right_panel(&self, ctx: &CtxRef) {
        SidePanel::right("right").resizable(false).show(ctx, |ui| {
            ui.label("Right Editor");
        });
    }

    fn bottom_panel(&self, ctx: &CtxRef) {
        TopBottomPanel::bottom("bottom").resizable(false).show(ctx, |ui| {
            ui.label("Bottom Editor");
        });
    }

    fn content_panel(&self, _ctx: &CtxRef) {
        /*
        CentralPanel::default().show(ctx, |ui|{
            ui.label(self.content.as_str())
        });
         */
    }
}