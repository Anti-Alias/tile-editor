use cgmath::{Deg, InnerSpace, Matrix4, Perspective, Point3, Rad, SquareMatrix, Vector3, VectorSpace};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use pollster::block_on;
use wgpu::{Device, Queue, TextureFormat, SurfaceConfiguration, CommandEncoderDescriptor, RenderPassDescriptor};
use winit::event::Event::*;
use winit::event_loop::ControlFlow;
use crate::graphics::*;
use crate::graphics::light::{AmbientLight, LightMesh, PointLight, LightBundle, DirectionalLight, LightSet};
use crate::graphics::scene::{DebugConfig, Scene};
use crate::graphics::screen::Screen;
use crate::gui::{GUI, Editor};


/// Represents the application as a whole.
/// Draws an EGUI interface on top of the map renderer
pub struct App {
    title: String,
    width: u32,
    height: u32,
    is_ui_enabled: bool,
    event_handler: Option<Box<dyn FnMut(AppEvent, AppState, &mut AppControlFlow)>>
}

impl App {
    pub fn new() -> App {
        App {
            title: String::from("App"),
            width: 640,
            height: 480,
            is_ui_enabled: true,
            event_handler: None
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn event_handler(mut self, handler: impl FnMut(AppEvent, AppState, &mut AppControlFlow) + 'static) -> Self {
        self.event_handler = Some(Box::new(handler));
        self
    }

    pub fn gui_enabled(mut self, enabled: bool) -> Self {
        self.is_ui_enabled = enabled;
        self
    }

    pub fn start(mut self) {

        // Initializes logger
        env_logger::init();

        // Creates WINIT window and event loop
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_title(self.title.clone())
            .with_inner_size(winit::dpi::PhysicalSize {
                width: self.width,
                height: self.height,
            })
            .build(&event_loop)
            .unwrap();

        // Creates WGPU instance and friends
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })).unwrap();
        let (mut device, mut queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )).unwrap();

        // Applies initial WGPU surface configuration
        let size = window.inner_size();
        let surface_format = surface.get_preferred_format(&adapter).unwrap();
        let mut surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        // Sets up models and the scene
        let camera = create_camera(&device);
        let light_bundle = LightBundle::new(
            &device,
            LightSet::new(&device, 32),
            LightSet::new(&device, 32),
            LightSet::new(&device, 32)
        );
        let mut scene = Scene::new(
            &device,
            camera,
            light_bundle,
            &surface_config,
            &DebugConfig {
                render_lights: true
            }
        );

        // Control flow
        let mut flow = AppControlFlow::CONTINUE;

        // Fires "start" event
        if let Some(ref mut event_handler) = self.event_handler {
            let mut flow = AppControlFlow::CONTINUE;
            event_handler(
                AppEvent::STARTED,
                AppState {
                    scene: &mut scene,
                    device: &device,
                    queue: &queue
                },
                &mut flow
            );
        }

        // Sets up EGUI
        let mut gui = GUI::new(Editor::new("Default Editor", "Default Editor"));
        let mut platform = Platform::new(PlatformDescriptor {
            physical_width: size.width as u32,
            physical_height: size.height as u32,
            scale_factor: window.scale_factor(),
            font_definitions: egui::FontDefinitions::default(),
            style: Default::default(),
        });
        let mut egui_rpass = RenderPass::new(&device, surface_format, 1);

        // ---------- Main loop ----------
        let start_time = std::time::Instant::now();
        let mut now = start_time;
        event_loop.run(move |event, _, control_flow| {

            // Pass the winit events to the platform integration.
            platform.handle_event(&event);

            match event {

                RedrawRequested(..) => {

                    // Time since last update
                    let duration = now.elapsed();
                    let dt = now.elapsed().as_secs_f32();
                    now += duration;

                    // Fires event
                    if let Some(ref mut event_handler) = self.event_handler {
                        event_handler(
                            AppEvent::UPDATE { dt },
                            AppState {
                                scene: &mut scene,
                                device: &device,
                                queue: &queue
                            },
                            &mut flow
                        );
                    }

                    // Gets texture view of surface
                    let surface_tex = match surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(_) => { return }
                    };
                    let surface_view = surface_tex
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    // Makes encoder and screen
                    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
                    let screen = Screen::new(surface_view);

                    // Renders scene
                    scene.flush(&queue);
                    scene.render(&screen, &mut encoder);

                    // Updates/draws EGUI
                    if self.is_ui_enabled {

                        // Renders GUI to platform
                        platform.update_time(start_time.elapsed().as_secs_f64());
                        platform.begin_frame();
                        gui.update(&platform.context());
                        let (_output, paint_commands) = platform.end_frame(Some(&window));

                        // Renders tesselated gui to render pass
                        let paint_jobs = platform.context().tessellate(paint_commands);
                        let screen_descriptor = ScreenDescriptor {
                            physical_width: surface_config.width,
                            physical_height: surface_config.height,
                            scale_factor: window.scale_factor() as f32,
                        };
                        egui_rpass.update_texture(&device, &queue, platform.context().texture().as_ref());
                        egui_rpass.update_user_textures(&device, &queue);
                        egui_rpass.update_buffers(&mut device, &mut queue, &paint_jobs, &screen_descriptor);
                        egui_rpass.execute(
                            &mut encoder,
                            &screen.view,
                            &paint_jobs,
                            &screen_descriptor,
                            None,
                        ).unwrap();
                    }

                    // Submits all draw commands and presents screen
                    let commands = encoder.finish();
                    queue.submit(std::iter::once(commands));
                    surface_tex.present();

                    // Finish
                    *control_flow = ControlFlow::Poll;
                }
                MainEventsCleared => {
                    window.request_redraw();
                }
                WindowEvent { event, .. } => match event {

                    // Screen resized
                    winit::event::WindowEvent::Resized(size) => {
                        if size.width != 0 { surface_config.width = size.width; }
                        if size.height != 0 { surface_config.height = size.height; }
                        surface.configure(&device, &surface_config);
                        scene.resize(&device, size.width, size.height);
                        if let Some(ref mut event_handler) = self.event_handler {
                            let mut flow = AppControlFlow::CONTINUE;
                            event_handler(
                                AppEvent::RESIZED {
                                    width: size.width,
                                    height: size.height
                                },
                                AppState {
                                    scene: &mut scene,
                                    device: &device,
                                    queue: &queue
                                },
                                &mut flow
                            );
                        }
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => (),
            }

            // Forces control flow exit if requested
            if flow == AppControlFlow::EXIT {
                *control_flow = ControlFlow::Exit
            }
        });
    }
}

/// Represents a change in the app
pub enum AppEvent {
    STARTED,
    UPDATE {
        dt: f32
    },
    RESIZED {
        width: u32,
        height: u32
    }
}

/// Determines if application should continue running
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AppControlFlow {
    CONTINUE,
    EXIT
}

/// State of the application.
/// Helpful, right???
pub struct AppState<'a> {
    pub scene: &'a mut Scene,
    pub device: &'a Device,
    pub queue: &'a Queue
}

fn create_camera(device: &Device) -> Camera {
    let mut cam = Camera::create_perspective(
        &device,
        Point3::<f32>::new(0.0, 0.0, 0.0),
        Vector3::<f32>::new(0.0, 0.0, -1.0),
        Vector3::<f32>::unit_y(),
        Perspective {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1.0
        }
    );
    cam
}