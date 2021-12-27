use egui::{CtxRef, Frame};
use epi::TextureAllocator;

/// Represents a generic editor
pub trait Editor: 'static {
    fn show(&mut self, ctx: &CtxRef, tex_alloc: &mut dyn TextureAllocator);
}