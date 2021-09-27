use wgpu::*;

pub struct GraphicsState {
    pub device: Device,
    pub queue: Queue
}

impl GraphicsState {

    pub fn render(&mut self, surface: &Surface) -> Result<(), wgpu::SurfaceError> {

        let tex = &surface.get_current_frame()?.output.texture;
        let texture_view = tex.create_view(&TextureViewDescriptor::default());

        // Creates an encoder
        let command_desc = CommandEncoderDescriptor { label: Some("Render Encoder") };
        let mut encoder = self.device.create_command_encoder(&command_desc);



        let cmd_buffer = encoder.finish();
        self.queue.submit(std::iter::once(cmd_buffer));
        Ok(())
    }

    pub fn create_render_pass<'a>(&self, encoder: &'a mut CommandEncoder, texture_view: &'a TextureView) -> RenderPass<'a> {

        // Creates color attachment
        let color_attachment = RenderPassColorAttachment {
            view: texture_view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0
                }),
                store: true
            }
        };

        // Creates render pass
        let render_desc = RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[color_attachment],
            depth_stencil_attachment: None
        };
        encoder.begin_render_pass(&render_desc)
    }
}