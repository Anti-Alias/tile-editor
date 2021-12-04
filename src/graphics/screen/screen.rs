use wgpu::{Color, LoadOp, Operations, RenderPass, RenderPassColorAttachment, TextureView, CommandEncoder, RenderPassDescriptor};

pub struct Screen {
    pub view: TextureView,
}
impl Screen {

    pub fn new(view: TextureView) -> Self {
        Self { view }
    }

    pub fn begin_render_pass<'a>(&'a self, encoder: &'a mut CommandEncoder) -> RenderPass<'a> {
        let color_attachments = self.color_attachments();
        encoder.begin_render_pass(
            &RenderPassDescriptor {
                label: None,
                color_attachments: &color_attachments,
                depth_stencil_attachment: None
            }
        )
    }

    /// Gets the color attachments necessary for a render pass
    pub fn color_attachments(&self) ->[RenderPassColorAttachment; 1] {
        [
            RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }),
                    store: true
                }
            }
        ]
    }
}