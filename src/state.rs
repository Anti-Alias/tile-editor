use winit::window::Window;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use wgpu::*;
use log::info;
use crate::Vertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

/// Represents entire graphics state (window, surface device, queue) all wrapped in one struct
pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    num_vertices: u32
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

        let render_pipeline = Self::create_render_pipeline(&device, &config);
        let (vertex_buffer, num_vertices) = Self::create_vertex_buffer(&device);

        // Return state
        State {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            num_vertices
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

        // Creates render pass and attaches pipeline.
        // Then, uses it to draw to teh screen!!1
        {
            let mut render_pass = self.create_render_pass(&mut encoder, &tex_view);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        let cmd_buffer = encoder.finish();
        self.queue.submit(std::iter::once(cmd_buffer));
        Ok(())
    }

    fn create_render_pass<'a>(&self, encoder: &'a mut CommandEncoder, texture_view: &'a TextureView) -> RenderPass<'a> {

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

    // ------------- Static -------------

    fn create_pipeline_layout(device: &Device) -> PipelineLayout {
        let desc = PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        };
        device.create_pipeline_layout(&desc)
    }

    fn create_shader_module(device: &Device) -> ShaderModule {
        let desc = ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: ShaderSource::Wgsl(include_str!("shader.wgsl").into())
        };
        device.create_shader_module(&desc)
    }

    fn create_vertex_state<'a>(module: &'a ShaderModule) -> VertexState<'a> {
        VertexState {
            module: &module,
            entry_point: "main",
            buffers: &[Vertex::DESC]
        }
    }

    fn create_fragment_state<'a>(
        module: &'a ShaderModule,
        targets: &'a [ColorTargetState]
    ) -> FragmentState<'a> {
        FragmentState {
            module: &module,
            entry_point: "main",
            targets
        }
    }

    fn create_color_target_state(config: &SurfaceConfiguration) -> ColorTargetState {
        ColorTargetState {
            format: config.format,
            blend: Some(BlendState::REPLACE),
            write_mask: ColorWrites::ALL
        }
    }

    fn get_primitive_state() -> PrimitiveState {
        PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            clamp_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false
        }
    }

    fn create_render_pipeline(device: &Device, config: &SurfaceConfiguration) -> RenderPipeline {
        let module = Self::create_shader_module(device);
        let vertex_state = Self::create_vertex_state(&module);
        let color_targets = [Self::create_color_target_state(&config)];
        let fragment_state = Self::create_fragment_state(&module, &color_targets);
        let pipeline_layout = Self::create_pipeline_layout(device);
        let desc = RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: vertex_state,
            fragment: Some(fragment_state),
            primitive: Self::get_primitive_state(),
            depth_stencil: None,
            multisample: MultisampleState::default()
        };
        device.create_render_pipeline(&desc)
    }

    fn create_vertex_buffer(device: &Device) -> (Buffer, u32) {
        let vertices = [
            Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
        ];
        let desc = BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::bytes_of(&vertices),
            usage: BufferUsages::VERTEX
        };
        (device.create_buffer_init(&desc), 3)
    }
}