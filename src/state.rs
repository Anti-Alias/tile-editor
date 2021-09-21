use winit::window::Window;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use wgpu::*;
use log::info;

/// Represents entire graphics state (window, surface device, queue) all wrapped in one struct
pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
    //render_pipeline: RenderPipeline
}

impl State {

    /// Asynchronously creates State using window
    pub async fn new(window: &Window) -> Self {

        // Gets window inner size
        let size = window.inner_size();

        // Creates instance, gets surface and selects adapter
        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.enumerate_adapters(Backends::all())
            .filter(|adapter| surface.get_preferred_format(&adapter).is_some() )
            .next()
            .unwrap();

        // With adapter, gets device and queue
        let descriptor = DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None
        };
        let (device, queue) = adapter.request_device(&descriptor, None).await.unwrap();

        // Configures surface and associates it with the device
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo
        };
        surface.configure(&device, &config);

        // Return state
        State {
            surface,
            device,
            queue,
            config,
            size,
            //render_pipeline
        }
    }

    /// Resizes surface the new size specified
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Handles window event.
    /// Returns true if event was processed.
    ///
    /// # Arguments
    ///
    /// * `event` - Window event to consider
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        return false;
    }

    pub fn update(&mut self) {
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        // Gets texture of surface and defines a view
        let surface_frame = self.surface.get_current_frame()?.output;
        let tex_view = surface_frame.texture.create_view(&TextureViewDescriptor::default());

        // Creates an encoder
        let command_desc = CommandEncoderDescriptor { label: Some("Render Encoder") };
        let mut encoder = self.device.create_command_encoder(&command_desc);

        // Creates color attachment
        let color_attachment = RenderPassColorAttachment {
            view: &tex_view,
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
        encoder.begin_render_pass(&render_desc);
        let cmd_buffer = encoder.finish();
        self.queue.submit(std::iter::once(cmd_buffer));
        Ok(())
    }
}