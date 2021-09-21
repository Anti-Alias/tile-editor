use winit::window::Window;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use wgpu::{Instance, Backends, DeviceDescriptor, SurfaceConfiguration, TextureFormat, PresentMode, TextureUsages};

/// Represents entire graphics state (window, surface device, queue) all wrapped in one struct
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
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
            size
        }
    }

    /// Resizes surface the new size specified
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn input(&mut self, event: &WindowEvent) {
        todo!()
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}