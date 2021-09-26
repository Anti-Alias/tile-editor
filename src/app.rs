use winit::window::{Window, WindowBuilder};
use winit::dpi::PhysicalSize;
use winit::event::{WindowEvent, Event, KeyboardInput, ElementState, VirtualKeyCode};
use wgpu::*;
use log::info;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::{WindowState, GraphicsState};

use winit::event_loop::{EventLoop, ControlFlow};

pub struct AppResources<'a> {
    pub device: &'a Device,
    pub queue: &'a Queue
}

pub trait AppListener: 'static {
    fn on_start(&self, resources: &AppResources);
    fn on_draw(&self, resources: &AppResources);
}

/// Represents tile application as a whole
pub struct App<L: AppListener> {
    listener: L,
    event_loop: EventLoop<()>,
    window_state: WindowState,
    graphics_state: GraphicsState
}

impl<L : AppListener> App<L> {

    /// Asynchronously creates State using window
    pub async fn new(listener: L) -> Self {

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
        //let render_pipeline = Self::create_render_pipeline(&device, &config);

        // Return state
        App {
            listener,
            event_loop,
            window_state: WindowState { window, surface, size, config },
            graphics_state: GraphicsState { device, queue }
        }
    }

    pub fn start(mut self) {

        // Creates event loop and window
        info!("Running event loop!");
        let mut window_state = self.window_state;
        let mut graphics_state = self.graphics_state;
        let listener = self.listener;

        // Alerts listener of starting
        listener.on_start(&AppResources {
            device: &graphics_state.device,
            queue: &graphics_state.queue
        });

        // Starts event loop
        self.event_loop.run(move |event, window_target, control_flow| match event {
            Event::WindowEvent { window_id, event: window_event } if window_id == window_state.window.id() => {
                match window_event {
                    WindowEvent::Resized(new_size) => {
                        window_state.resize(&graphics_state.device, new_size);
                    },
                    _ => {}
                }
                window_state.handle_window_event(window_event, &graphics_state.device, control_flow);
            }
            Event::Suspended => { },
            Event::Resumed => { },
            Event::MainEventsCleared => { window_state.request_redraw(); }
            Event::RedrawRequested(_) => {
                listener.on_draw(&AppResources{
                    device: &graphics_state.device,
                    queue: &graphics_state.queue
                });
                graphics_state.render(&window_state.surface);
            }
            _ => {}
        });
    }

    pub fn device(&self) -> &Device {
        &self.graphics_state.device
    }

    pub fn queue(&self) -> &Queue {
        &self.graphics_state.queue
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

    /*
    fn create_vertex_state(module: &ShaderModule) -> VertexState {
        VertexState {
            module: &module,
            entry_point: "main",
            buffers: &[ModelVertex::BUFFER_LAYOUT]
        }
    }
     */

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

    /*
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
     */
}