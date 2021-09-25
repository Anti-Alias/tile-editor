use winit::window::{Window, WindowBuilder};
use winit::dpi::PhysicalSize;
use winit::event::{WindowEvent, Event, KeyboardInput, ElementState, VirtualKeyCode};
use wgpu::*;
use log::info;
use crate::Vertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::event_loop::{EventLoop, ControlFlow};

/// Represents entire graphics state (window, surface device, queue) all wrapped in one struct
pub struct App {
    event_loop: EventLoop<()>,
    window_state: WindowState,
    graphics_state: GraphicsState
}

impl App {

    /// Asynchronously creates State using window
    pub async fn new() -> Self {

        // Creates window and event loop
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Map Editor")
            .build(&event_loop).unwrap();
        info!("Created window!");

        // Gets window inner size
        let size = window.inner_size();

        // Creates instance, gets surface and selects adapter
        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
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

        let surface_frame = surface.get_current_frame()?.output;
        surface_frame.texture.create_view(&TextureViewDescriptor::default());

        // Return state
        App {
            event_loop,
            window_state: WindowState { window, surface, size, config },
            graphics_state: GraphicsState { device, queue, render_pipeline }
        }
    }

    pub fn start(mut self) {

        // Creates event loop and window
        info!("Running event loop!");
        let mut window_state = self.window_state;
        let mut graphics_state = self.graphics_state;

        // Starts event loop
        self.event_loop.run(move |event, window_target, control_flow| match event {
            Event::WindowEvent { window_id, event: window_event } if window_id == window_state.window.id() => {
                window_state.handle_window_event(window_event, &graphics_state.device, control_flow);
            }
            Event::Suspended => { },
            Event::Resumed => { },
            Event::MainEventsCleared => { window_state.request_redraw(); }
            Event::RedrawRequested(_) =>{ graphics_state.render(); }
            _ => {}
        });
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

    fn create_vertex_state(module: &ShaderModule) -> VertexState {
        VertexState {
            module: &module,
            entry_point: "main",
            buffers: &[Vertex::BUFFER_LAYOUT]
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
}

struct WindowState {
    window: Window,
    surface: Surface,
    size: PhysicalSize<u32>,
    config: SurfaceConfiguration
}

impl WindowState {

    /// Handles window event.
    /// Returns true if event was processed.
    ///
    /// # Arguments
    ///
    /// * `event` - Window event to consider
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        return false;
    }

    fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn handle_window_event(
        &mut self,
        event: WindowEvent,
        device: &Device,
        control_flow: &mut ControlFlow
    ) {
        if !self.input(&event) {
            match event {
                WindowEvent::CloseRequested => close(control_flow),
                WindowEvent::KeyboardInput { input, .. } => self.handle_key(input, control_flow),
                WindowEvent::Resized(new_size) => self.resize(device, new_size),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self.resize(device, *new_inner_size),
                _ => {}
            }
        }
    }

    /// Resizes surface the new size specified
    pub fn resize(&mut self, device: &Device, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(device, &self.config);
    }

    fn handle_key(&self, input: KeyboardInput, control_flow: &mut ControlFlow) {
        if input.state == ElementState::Pressed && input.virtual_keycode == Some(VirtualKeyCode::Escape) {
            close(control_flow);
        }
    }

    fn handle_suspend(&self) {
        println!("Suspended");
    }

    fn handle_resume(&self) {
        println!("Resuming");
    }
}

struct GraphicsState {
    pub device: Device,
    pub queue: Queue,
    pub texture_view: TextureView,
    render_pipeline: RenderPipeline
}

impl GraphicsState {

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        // Gets texture of surface and defines a view
        let tex_view = &self.texture_view;

        // Creates an encoder
        let command_desc = CommandEncoderDescriptor { label: Some("Render Encoder") };
        let mut encoder = self.device.create_command_encoder(&command_desc);
        //let tex = self.device.create_texture();

        // Creates render pass and attaches pipeline.
        // Then, uses it to draw to teh screen!!1
        {
            let mut render_pass = self.create_render_pass(&mut encoder, &tex_view);
            /*
            let mesh = &self.mesh;
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint32);
            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
            */
        }

        let cmd_buffer = encoder.finish();
        self.queue.submit(std::iter::once(cmd_buffer));
        Ok(())
    }
}

fn close(control_flow: &mut ControlFlow) {
    *control_flow = ControlFlow::Exit;
    info!("See ya!");
}