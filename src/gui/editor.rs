use egui::CtxRef;

/// Represents a generic editor
pub trait Editor {
    fn show(&self, ctx: &CtxRef);
}