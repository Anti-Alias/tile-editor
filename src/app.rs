use winit::window::{Window, WindowBuilder};
use winit::dpi::PhysicalSize;
use winit::event::{WindowEvent, Event, KeyboardInput, ElementState, VirtualKeyCode};
use wgpu::*;
use log::info;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::{WindowState, GraphicsState};

use winit::event_loop::{EventLoop, ControlFlow};

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
            window_state: WindowState { window, surface, config },
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
            config: &window_state.config,
            device: &graphics_state.device,
            queue: &graphics_state.queue
        });

        // Starts event loop
        self.event_loop.run(move |event, window_target, control_flow| match event {
            Event::WindowEvent { window_id, event: window_event } if window_id == window_state.window.id() => {
                match window_event {
                    WindowEvent::Resized(new_size) => {
                        let new_size = PhysicalSize {
                            width: if new_size.width <= 0 { 1 } else { new_size.width },
                            height: if new_size.height <= 0 { 1 } else { new_size.height }
                        };
                        window_state.resize(&graphics_state.device, new_size);
                        listener.on_resize(new_size, &AppResources {
                            config: &window_state.config,
                            device: &graphics_state.device,
                            queue: &graphics_state.queue
                        });
                    },
                    _ => {}
                }
                window_state.handle_window_event(window_event, &graphics_state.device, control_flow);
            }
            Event::Suspended => { },
            Event::Resumed => { },
            Event::MainEventsCleared => { window_state.request_redraw(); }
            Event::RedrawRequested(_) => {

                // Gets texture from surface
                let tex = &window_state.surface.get_current_frame().unwrap().output.texture;
                let view = tex.create_view(&TextureViewDescriptor::default());

                // Clears screen and hands render pass to listener
                let desc = CommandEncoderDescriptor { label: Some("App Command Encoder") };
                let mut encoder = graphics_state.device.create_command_encoder(&desc);
                {
                    let mut render_pass = graphics_state.create_render_pass(&mut encoder, &view);
                    listener.on_draw(&mut render_pass);
                }

                // Executes draw commands
                let buffer = encoder.finish();
                graphics_state.queue.submit([buffer]);
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
}

/// Resource object passed into `AppListener` during its on-* methods
pub struct AppResources<'a> {
    pub config: &'a SurfaceConfiguration,
    pub device: &'a Device,
    pub queue: &'a Queue
}

/// Listener of events occurring in an `App` instance
pub trait AppListener: 'static {
    fn on_start(&self, app_resources: &AppResources);
    fn on_draw(&self, render_pass: &mut RenderPass);
    fn on_resize(&self, size: PhysicalSize<u32>, app_resources: &AppResources);
}