use egui_wgpu_backend::wgpu::TextureView;

/// Main frame buffer to be rendered to after the geometry pass.
/// Does not own texture views as they tend to change frequently each frame.
pub struct ScreenBuffer<'a> {
    pub color: &'a TextureView,
    pub depth_stencil: &'a TextureView
}