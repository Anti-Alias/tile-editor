
use std::iter;
use std::time::Instant;
use cgmath::{Deg, Ortho, Perspective, PerspectiveFov, Point3, Rad, Vector3};
use egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use futures_lite::future::block_on;
use wgpu::{Device, Queue, TextureFormat, TextureViewDescriptor};
use winit::event::Event::*;
use winit::event_loop::{ControlFlow};
use crate::graphics::*;
use crate::gui::{GUI, Editor};

/// Represents the application as a whole.
/// Draws an EGUI interface on top of the map renderer
pub struct App {
    title: String,
    width: u32,
    height: u32,
    depth_stencil_format: TextureFormat
}

impl App {

    pub fn new() -> App {
        App {
            title: String::from("App"),
            width: 640,
            height: 480,
            depth_stencil_format: TextureFormat::Depth32Float
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

    pub fn depth_stencil_format(mut self, format: TextureFormat) -> Self {
        self.depth_stencil_format = format;
        return self
    }

    pub fn start(self) {

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
        let mut surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        // Creates depth buffer
        let mut depth_stencil = create_surface_depth_texture(&device, &self.depth_stencil_format, &surface_config);
        let mut depth_stencil_view = depth_stencil.create_view(&TextureViewDescriptor::default());

        // Sets up environment to render (models, camera, lights, etc)
        let mut camera = create_camera(&device, size.width, size.height);
        let model_instances = create_model_and_instances(&device, &queue);

        // Creates model renderer, then primes it with the environment
        let mut renderer = ModelRenderer::new(surface_config.format, self.depth_stencil_format);
        renderer.prime(
            &device,
            &ModelEnvironment {
                instance_set: &model_instances,
                camera: &camera,
                point_lights: &[],
                directional_lights: &[]
            }
        );

        // Sets up EGUI
        let mut gui = GUI::new(Editor::new("Default Editor", "Default Editor"));
        let mut platform = Platform::new(PlatformDescriptor {
            physical_width: size.width as u32,
            physical_height: size.height as u32,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });
        let mut egui_rpass = RenderPass::new(&device, surface_format, 1);
        let start_time = Instant::now();

        // ---------- Main loop ----------
        let mut t: f32 = 0.0;
        event_loop.run(move |event, _, control_flow| {

            // Pass the winit events to the platform integration.
            platform.handle_event(&event);

            match event {
                RedrawRequested(..) => {

                    // Gets texture view of surface
                    let surface_frame = match surface.get_current_frame() {
                        Ok(frame) => frame,
                        Err(_) => { return }
                    };
                    let surface_view = surface_frame
                        .output
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    // Flushes environment uniforms, then renders model environment
                    camera.flush(&queue);
                    renderer.render(
                        &device,
                        &queue,
                        &ModelEnvironment {
                            instance_set: &model_instances,
                            camera: &camera,
                            point_lights: &[],
                            directional_lights: &[]
                        },
                        &ModelFrameBuffer {
                            color: &surface_view,
                            depth_stencil: &depth_stencil_view
                        }
                    );

                    // Moves camera
                    let theta = std::f32::consts::PI * t;
                    let rad = 300.0_f32;
                    camera.move_to(Point3::new(
                        f32::cos(theta)*rad,
                        f32::sin(theta*2.0)*180.0_f32,
                        f32::sin(theta)*rad)
                    );
                    camera.look_at(Point3::new(0.0, 0.0, 0.0));

                    // Updates/draws EGUI
                    platform.update_time(start_time.elapsed().as_secs_f64());
                    platform.begin_frame();
                    gui.update(&platform.context());
                    let (_output, paint_commands) = platform.end_frame(Some(&window));
                    let paint_jobs = platform.context().tessellate(paint_commands);
                    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    let screen_descriptor = ScreenDescriptor {
                        physical_width: surface_config.width,
                        physical_height: surface_config.height,
                        scale_factor: window.scale_factor() as f32,
                    };
                    egui_rpass.update_texture(&device, &queue, &platform.context().texture());
                    egui_rpass.update_user_textures(&device, &queue);
                    egui_rpass.update_buffers(&mut device, &mut queue, &paint_jobs, &screen_descriptor);
                    egui_rpass.execute(
                        &mut encoder,
                        &surface_view,
                        &paint_jobs,
                        &screen_descriptor,
                        None,
                    ).unwrap();

                    // Submit the commands.
                    queue.submit(iter::once(encoder.finish()));

                    // Done with current loop
                    *control_flow = ControlFlow::Poll;

                    // Update t
                    t += 0.003;
                }
                MainEventsCleared => {
                    window.request_redraw();
                }
                WindowEvent { event, .. } => match event {

                    // Screen resized
                    winit::event::WindowEvent::Resized(size) => {

                        // Updates surface and depth_stencil
                        if size.width != 0 { surface_config.width = size.width; }
                        if size.height != 0 { surface_config.height = size.height; }
                        surface.configure(&device, &surface_config);
                        depth_stencil = create_surface_depth_texture(&device, &self.depth_stencil_format, &surface_config);
                        depth_stencil_view = depth_stencil.create_view(&TextureViewDescriptor::default());

                        // Updates camera
                        update_camera(&mut camera, size.width, size.height);
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => (),
            }
        });
    }
}

const CAM_NEAR: f32 = 1.0;
const CAM_FAR: f32 = 1000.0;
const CAM_PERSPECTIVE_SCALE: f32 = (1.0/200.0) as f32;

/*
fn create_camera(device: &Device, width: u32, height: u32) -> Camera {
    let sw = width as f32;
    let sh = height as f32;
    let hw = sw / 2.0;
    let hh = sh / 2.0;
    let mut cam = Camera::create_ortho(
        &device,
        Point3::<f32>::new(0.0, 0.0, 0.0),
        Vector3::<f32>::new(0.0, 0.0, -1.0),
        Vector3::<f32>::unit_y(),
        Ortho { left: -hw, right: hw, bottom: -hh, top: hh, near: CAM_NEAR, far: CAM_FAR }
    );
    cam.set_coordinate_system(Camera::OPENGL_COORDINATE_SYSTEM);
    cam
}

fn update_camera(camera: &mut Camera, width: u32, height: u32) {
    let sw = width as f32;
    let sh = height as f32;
    let hw = sw / 2.0;
    let hh = sh / 2.0;
    camera.set_ortho(Ortho { left: -hw, right: hw, bottom: -hh, top: hh, near: CAM_NEAR, far: CAM_FAR });
}
*/

fn create_camera(device: &Device, width: u32, height: u32) -> Camera {
    let sw = width as f32;
    let sh = height as f32;
    let hw = sw / 2.0;
    let hh = sh / 2.0;
    let mut cam = Camera::create_perspective(
        &device,
        Point3::<f32>::new(0.0, 0.0, 0.0),
        Vector3::<f32>::new(0.0, 0.0, -1.0),
        Vector3::<f32>::unit_y(),
        Perspective {
            left: -hw * CAM_PERSPECTIVE_SCALE,
            right: hw * CAM_PERSPECTIVE_SCALE,
            bottom: -hh * CAM_PERSPECTIVE_SCALE,
            top: hh * CAM_PERSPECTIVE_SCALE,
            near: CAM_NEAR,
            far: CAM_FAR
        }
    );
    cam.set_coordinate_system(Camera::OPENGL_COORDINATE_SYSTEM);
    cam
}

fn update_camera(camera: &mut Camera, width: u32, height: u32) {
    let sw = width as f32;
    let sh = height as f32;
    let hw = sw / 2.0;
    let hh = sh / 2.0;
    camera.set_perspective(Perspective {
        left: -hw * CAM_PERSPECTIVE_SCALE,
        right: hw * CAM_PERSPECTIVE_SCALE,
        bottom: -hh * CAM_PERSPECTIVE_SCALE,
        top: hh * CAM_PERSPECTIVE_SCALE,
        near: CAM_NEAR,
        far: CAM_FAR
    });
}

fn create_model_and_instances(device: &Device, queue: &Queue) -> ModelInstanceSet {

    // Creates texture from image
    use image::io::Reader as ImageReader;
    let diffuse_img = ImageReader::open("assets/cubemap.png")
        .unwrap()
        .decode()
        .unwrap();
    let diffuse_tex = Texture::from_image(device, queue, &diffuse_img, None);
    let material = MaterialBuilder::new()
        .diffuse(diffuse_tex)
        .build(&device);

    // Creates cube model
    let model = Model {
        meshes: vec![Mesh::cube(&device, Color::WHITE, Vector3::new(100.0, 100.0, 100.0))],
        materials: vec![material],
        associations: vec![(0, 0)]
    };

    // Creates model instances and returns instance set
    ModelInstanceSet::new(&device, model, vec![
        ModelInstance {
            world: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [100.0, 0.0, 0.0, 1.0]
            ]
        },
        ModelInstance {
            world: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [-100.0, 0.0, 0.0, 1.0]
            ]
        }
    ])
}