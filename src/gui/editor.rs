use egui::CtxRef;

/// Represents a generic editor
pub trait Editor: 'static {
    fn show(&self, ctx: &CtxRef);
}