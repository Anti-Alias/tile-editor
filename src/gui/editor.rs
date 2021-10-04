use egui::{Widget, Ui, Response, SidePanel, CtxRef, CentralPanel, TopBottomPanel};

pub struct Editor {
    pub name: String,
    pub content: String
}

impl Editor {

    pub fn new(name: &str, content: &str) -> Editor {
        Editor {
            name: name.to_owned(),
            content: content.to_owned()
        }
    }

    pub fn left_panel(&self, ctx: &CtxRef) {
        SidePanel::left("left").resizable(false).show(ctx, |ui| {
            ui.label("Left Editor");
        });
    }

    pub fn right_panel(&self, ctx: &CtxRef) {
        SidePanel::right("right").resizable(false).show(ctx, |ui| {
            ui.label("Right Editor");
        });
    }

    pub fn bottom_panel(&self, ctx: &CtxRef) {
        TopBottomPanel::bottom("bottom").resizable(false).show(ctx, |ui| {
            ui.label("Bottom Editor");
        });
    }

    pub fn content_panel(&self, ctx: &CtxRef) {
        CentralPanel::default().show(ctx, |ui|{
            ui.label(self.content.as_str())
        });
    }
}