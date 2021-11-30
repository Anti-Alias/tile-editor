use std::f32::consts::PI;
use std::iter;
use std::time::Instant;
use cgmath::{Perspective, Point3, Vector3};

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use pollster::block_on;
use wgpu::{Device, Queue, TextureFormat, TextureViewDescriptor, SurfaceConfiguration};
use winit::event::Event::*;
use winit::event_loop::ControlFlow;

use crate::graphics::*;
use crate::graphics::gbuffer::{GBuffer, ModelEnvironment};
use crate::graphics::light::{AmbientLight, LightMesh, LightSet, PointLight, LightBundle, DirectionalLight};

use crate::graphics::screen;
use crate::graphics::screen::ScreenBuffer;
use crate::gui::{GUI, Editor};

/// Represents the application as a whole.
/// Draws an EGUI interface on top of the map renderer
pub struct App {
    title: String,
    width: u32,
    height: u32,
    depth_stencil_format: TextureFormat,
    is_ui_enabled: bool,
    input_handler: Option<Box<dyn FnMut(App)>>
}

impl App {

    pub fn new() -> App {
        App {
            title: String::from("App"),
            width: 640,
            height: 480,
            depth_stencil_format: TextureFormat::Depth32Float,
            is_ui_enabled: true,
            input_handler: None
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
        self
    }

    pub fn input_handler(mut self, handler: impl FnMut(App) + 'static) -> Self {
        self.input_handler = Some(Box::new(handler));
        self
    }

    pub fn gui_enabled(mut self, enabled: bool) -> Self {
        self.is_ui_enabled = enabled;
        self
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

        // Creates GBuffer and loads model
        let mut gbuffer = GBuffer::new(&device, size.width, size.height);

        // Sets up model (with instances), camera and lights
        let model_instances = create_model_instances(&device, &queue);
        let mut camera = create_camera(&device, size.width, size.height);
        let (mut light_bundle, mut light_mesh) = create_lights(&device, &queue);

        // Creates model-> gbuffer renderer, then primes it
        let mut model_renderer = gbuffer::ModelRenderer::new();
        model_renderer.prime(
            &device,
            &model_instances.model,
            camera.bind_group_layout()
        );

        // Creates point_light -> screen renderer
        let mut point_light_renderer = screen::PointLightRenderer::new(
            &device,
            surface_format,
            gbuffer.bind_group_layout(),
            camera.bind_group_layout()
        );

        // Creates ambient/directional_light -> screen renderer
        let mut light_renderer = screen::LightRenderer::new(
            &device,
            surface_format,
            gbuffer.bind_group_layout(),
            light_bundle.bind_group_layout()
        );

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
        let start_time = Instant::now();

        // ---------- Main loop ----------
        let mut t: f32 = 0.0;
        event_loop.run(move |event, _, control_flow| {

            // Pass the winit events to the platform integration.
            platform.handle_event(&event);

            match event {
                RedrawRequested(..) => {


                    // Gets texture view of surface
                    let surface_tex = match surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(_) => { return }
                    };
                    let surface_view = surface_tex
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    // Renders models to gbuffer
                    camera.flush(&queue);
                    model_renderer.render(
                        &device,
                        &queue,
                        &model_instances,
                        &camera,
                        &gbuffer
                    );

                    // Renders point lights to screen using gbuffer
                    point_light_renderer.render(
                        &device,
                        &queue,
                        &surface_view,
                        &gbuffer,
                        &light_bundle.point_lights,
                        &light_mesh,
                        &camera
                    );

                    light_renderer.render(
                        &device,
                        &queue,
                        &surface_view,
                        gbuffer.bind_group(),
                        light_bundle.bind_group()
                    );

                    // Moves lights
                    let point_lights = &mut light_bundle.point_lights;
                    for (i, light) in point_lights.lights.iter_mut().enumerate() {
                        let theta = PI * t / (i+1) as f32;
                        let light_pos = &mut light.position;
                        light_pos[0] = f32::cos(theta / 2.0) * 200.0;
                        light_pos[2] = f32::sin(theta / 2.0) * 200.0;
                    }
                    point_lights.flush(&queue);

                    // Moves camera
                    //move_camera(&mut camera, t);
                    move_camera(&mut camera, 1.5);


                    // Updates/draws EGUI
                    if self.is_ui_enabled {
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
                        egui_rpass.update_texture(&device, &queue, platform.context().texture().as_ref());
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
                    }

                    surface_tex.present();

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
                        gbuffer = GBuffer::new(&device, size.width, size.height);

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
const CAM_FAR: f32 = 8000.0;
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

fn move_camera(camera: &mut Camera, t: f32) {
    let rad = 300.0_f32;
    let th = t * PI / 2.0;
    camera.move_to(Point3::new(
        f32::cos(th)*rad,
        f32::sin(th)*180.0_f32,
        f32::sin(th)*rad)
    );
    camera.look_at(Point3::new(0.0, 0.0, 0.0));
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

fn create_lights(device: &Device, queue: &Queue) -> (LightBundle, LightMesh) {

    // Creates light mesh
    let light_mesh = LightMesh::new(&device, 8, 16);

    // Gets light sets
    let mut light_bundle = LightBundle::create(&device, 64, 64, 64);
    let point_lights = &mut light_bundle.point_lights;
    let ambient_lights = &mut light_bundle.ambient_lights;
    let directional_lights = &mut light_bundle.directional_lights;

    // Adds point light(s)
    let intensity = 16000.0;
    point_lights.lights.push(PointLight::new(
        [0.0, 100.0, 0.0],                  // Position
        [intensity, intensity, intensity],  // Color
        [1.0, 0.0, 1.0]                     // Attenuation
    ));
    point_lights.compute_radiuses(5.0/256.0);

    // Adds directional light(s)
    let bri = 60.0/255.0;
    directional_lights.lights.push(DirectionalLight::new([-1.0, 0.0, 0.0], [bri, 0.0, 0.0]));       // Red light pointing left (illuminates right site)
    directional_lights.lights.push(DirectionalLight::new([1.0, 0.0, 0.0], [0.0, 0.0, bri*3.0]));    // Blue light pointing right (illuminates left side)

    // Adds ambient light(s)
    ambient_lights.lights.push(AmbientLight::new([0.05, 0.05, 0.05]));

    // Done
    light_bundle.flush(queue);
    (light_bundle, light_mesh)
}


fn create_tex_from_file(file_name: &str, device: &Device, queue: &Queue) -> Texture {
    use image::io::Reader as ImageReader;
    let img = ImageReader::open(file_name)
        .unwrap()
        .decode()
        .unwrap();
    Texture::from_image(device, queue, &img, None)
}

fn create_model_instances(device: &Device, queue: &Queue) -> ModelInstanceSet {

    // Creates texture from image
    let diffuse_tex = create_tex_from_file("assets/cubemap/diffuse.png", device, queue);
    let specular_tex = create_tex_from_file("assets/cubemap/specular.png", device, queue);
    let material = MaterialBuilder::new()
        .diffuse(diffuse_tex)
        .specular(specular_tex)
        .build(&device);

    // Creates cube model
    let model = Model {
        meshes: vec![Mesh::cube(&device, Color::WHITE, Vector3::new(100.0, 100.0, 100.0))],
        materials: vec![material],
        associations: vec![(0, 0)]
    };

    // Joins model with instance data and returns
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