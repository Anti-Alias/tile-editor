use wgpu::{Color, LoadOp, Operations, RenderPass, RenderPassColorAttachment, TextureView, CommandEncoder, RenderPassDescriptor, RenderPassDepthStencilAttachment};

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

    pub fn begin_render_pass_with_depth<'a>(
        &'a self,
        depth_stencil_view: &'a TextureView,
        encoder: &'a mut CommandEncoder
    ) -> RenderPass<'a> {
        let color_attachments = [
            RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true
                }
            }
        ];
        let depth_stencil_attachment = RenderPassDepthStencilAttachment {
            view: depth_stencil_view,
            depth_ops: Some(Operations {
                load: LoadOp::Load,
                store: true
            }),
            stencil_ops: None
        };
        encoder.begin_render_pass(
            &RenderPassDescriptor {
                label: None,
                color_attachments: &color_attachments,
                depth_stencil_attachment: Some(depth_stencil_attachment)
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