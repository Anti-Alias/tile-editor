use std::f32::consts::PI;
use std::iter;
use std::time::Instant;
use cgmath::{Deg, InnerSpace, Matrix4, Perspective, Point3, Rad, SquareMatrix, Vector3, VectorSpace};

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use pollster::block_on;


use wgpu::{Device, Queue, TextureFormat, SurfaceConfiguration, CommandEncoderDescriptor, RenderPassDescriptor};
use winit::event::Event::*;
use winit::event_loop::ControlFlow;

use crate::graphics::*;
use crate::graphics::gbuffer::{GBuffer};
use crate::graphics::light::{AmbientLight, LightMesh, PointLight, LightBundle, DirectionalLight};
use crate::graphics::screen::Screen;

use crate::gui::{GUI, Editor};
use crate::graphics::util::Matrix4Ext;

/// Represents the application as a whole.
/// Draws an EGUI interface on top of the map renderer
pub struct App {
    title: String,
    width: u32,
    height: u32,
    is_ui_enabled: bool,
    input_handler: Option<Box<dyn FnMut(App)>>
}

impl App {

    pub fn new() -> App {
        App {
            title: String::from("App"),
            width: 640,
            height: 480,
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

        // Creates GBuffer
        let mut gbuffer = GBuffer::new(&device, size.width, size.height);

        // Sets up model (with instances), camera and lights
        let model_instances = create_model_instances(&device, &queue);
        let floor_instance = create_wood_floor_instance(&device, &queue);
        let mut camera = create_camera(
            &device,
            size.width as f32,
            size.height as f32
        );
        let (mut light_bundle, light_mesh) = create_lights(&device, &queue);

        // Creates model-> gbuffer renderer, then primes it
        let mut model_renderer = gbuffer::ModelRenderer::new();
        model_renderer.prime(
            &device,
            &model_instances.model,
            camera.bind_group_layout()
        );
        model_renderer.prime(
            &device,
            &floor_instance.model,
            camera.bind_group_layout()
        );

        // Creates point_light -> screen renderer
        let point_light_renderer = screen::PointLightRenderer::new(
            &device,
            surface_format,
            gbuffer.bind_group_layout(),
            camera.bind_group_layout()
        );

        // Creates ambient/directional_light -> screen renderer
        let light_renderer = screen::LightRenderer::new(
            &device,
            surface_format,
            gbuffer.bind_group_layout(),
            light_bundle.bind_group_layout(),
            camera.bind_group_layout()
        );

        // Creates point light debug renderer
        let point_light_debug_renderer = screen::PointLightDebugRenderer::new(
            &device,
            5.0,
            surface_format,
            GBuffer::DEPTH_STENCIL_FORMAT,
            camera.bind_group_layout()
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

                    // Makes encoder and screen
                    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
                    let screen = Screen::new(surface_view);

                    // Renders models to gbuffer
                    {
                        let mut render_pass = gbuffer.begin_render_pass(&mut encoder, true);
                        camera.flush(&queue);
                        model_renderer.render(
                            &mut render_pass,
                            &model_instances,
                            &camera
                        );
                        model_renderer.render(
                            &mut render_pass,
                            &floor_instance,
                            &camera
                        );
                    }

                    // Draws and moves lights
                    {
                        let mut render_pass = screen.begin_render_pass(&mut encoder);
                        point_light_renderer.render(
                            &mut render_pass,
                            &gbuffer,
                            &light_bundle.point_lights,
                            &light_mesh,
                            &camera
                        );
                        light_renderer.render(
                            &mut render_pass,
                            &gbuffer,
                            &light_bundle,
                            &camera
                        );
                    }

                    // Draws debug lights
                    {
                        let mut render_pass = screen.begin_render_pass_with_depth(gbuffer.depth_stencil_view(), &mut encoder);
                        point_light_debug_renderer.render(
                            &mut render_pass,
                            &light_bundle.point_lights,
                            &light_mesh,
                            &camera
                        );
                    }

                    // Moves lights
                    move_lights(&mut light_bundle, 150.0, t*1.414);
                    light_bundle.flush(&queue);

                    // Moves camera
                    move_camera(&mut camera, 150.0, t, 300.0);

                    // Updates/draws EGUI
                    if self.is_ui_enabled {
                        platform.update_time(start_time.elapsed().as_secs_f64());
                        platform.begin_frame();
                        gui.update(&platform.context());
                        let (_output, paint_commands) = platform.end_frame(Some(&window));
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
                    t += 0.003;
                    *control_flow = ControlFlow::Poll;
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
                        //update_camera(&mut camera, size.width as f32, size.height as f32);
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

const CAM_NEAR: f32 = 2.0;
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

fn create_camera(device: &Device, width: f32, height: f32) -> Camera {
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

fn update_camera(camera: &mut Camera, width: f32, height: f32) {
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

fn move_lights(light_bundle: &mut LightBundle, radius: f32, t: f32) {
    let point_lights = &mut light_bundle.point_lights;
    for (i, light) in point_lights.lights.iter_mut().enumerate() {
        let theta = PI * t / (i+1) as f32;
        let light_pos = &mut light.position;
        light_pos[0] = f32::cos(theta / 2.0) * radius;
        light_pos[2] = f32::sin(theta / 2.0) * radius;
    }
}

fn move_camera(camera: &mut Camera, y: f32, t: f32, rad: f32) {
    let th = t * PI / 2.0;
    camera.move_to(Point3::new(
        f32::cos(th)*rad,
        f32::sin(th)*rad/2.0 + y,
        f32::sin(th)*rad)
    );
    camera.look_at(Point3::new(0.0, 0.0, 0.0));
}

fn create_lights(device: &Device, queue: &Queue) -> (LightBundle, LightMesh) {

    // Creates light mesh
    let light_mesh = LightMesh::new(&device, 8, 16);

    // Gets light sets
    let mut light_bundle = LightBundle::create(&device, 64, 64, 64);
    let point_lights = &mut light_bundle.point_lights;
    let ambient_lights = &mut light_bundle.ambient_lights;
    let _directional_lights = &mut light_bundle.directional_lights;

    // Adds point light(s)
    let intensity = 40000.0;
    point_lights.lights.push(PointLight::new(
        [0.0, 100.0, 250.0],                   // Position
        [intensity, intensity, intensity],     // Color
        [1.0, 0.0, 1.0]                        // Attenuation
    ));
    point_lights.compute_radiuses(5.0/256.0);

    // Adds directional light(s)
    //let db = 255.0/255.0;
    //directional_lights.lights.push(DirectionalLight::new([0.0, -1.0, 0.0], [db, db, db]));       // White light pointing left (illuminates right site)

    // Adds ambient light(s)
    let ab = 8.0/255.0;
    ambient_lights.lights.push(AmbientLight::new([ab, ab, ab]));

    // Done
    light_bundle.flush(queue);
    (light_bundle, light_mesh)
}


fn create_tex_from_file(file_name: &str, device: &Device, queue: &Queue, format: TextureFormat) -> Texture {
    log::info!("Loading texture '{}'...", file_name);
    use image::io::Reader as ImageReader;
    let img = ImageReader::open(file_name)
        .unwrap()
        .decode()
        .unwrap();
    let tex = Texture::from_image(device, queue, &img, format, None);
    log::info!("Finished loading texture '{}'...", file_name);
    tex
}

fn create_model_instances(device: &Device, queue: &Queue) -> ModelInstanceSet {

    // Creates texture from image
    let diffuse_tex = create_tex_from_file("assets/cubemap/diffuse.png", device, queue, TextureFormat::Rgba8UnormSrgb);
    let emissive_tex = create_tex_from_file("assets/cubemap/emissive.png", device, queue, TextureFormat::Rgba8UnormSrgb);
    let normal_tex = create_tex_from_file("assets/cubemap/normal.png", device, queue, TextureFormat::Rgba8Unorm);
    let specular_tex = create_tex_from_file("assets/cubemap/specular.png", device, queue, TextureFormat::Rgba8Unorm);
    let gloss_tex = create_tex_from_file("assets/cubemap/gloss.png", device, queue, TextureFormat::Rgba8Unorm);
    let material = MaterialBuilder::new()
        .diffuse(diffuse_tex)
        .emissive(emissive_tex)
        .normal(normal_tex)
        .specular(specular_tex)
        .gloss(gloss_tex)
        .build(&device);

    // Creates cube model
    let model = Model {
        meshes: vec![Mesh::cube(&device, Color::WHITE, Vector3::new(100.0, 100.0, 100.0))],
        materials: vec![material],
        associations: vec![(0, 0)]
    };

    // Joins model with instance data and returns
    let mut mis = ModelInstanceSet::new(device, model, 4);
    mis
        .push(ModelInstance::new(Matrix4::identity().translate(Vector3::new(100.0, 0.0, 0.0))))
        .push(ModelInstance::new(Matrix4::identity().translate(Vector3::new(-100.0, 0.0, 0.0))))
        .push(ModelInstance::new(Matrix4::identity().translate(Vector3::new(-100.0, 0.0, 0.0))))
        .push(ModelInstance::new(Matrix4::identity()
            .translate(Vector3::new(0.0, 100.0, 0.0))
            .rotate_degrees(Vector3::new(1.0, 0.0, 0.0).normalize(), 45.0)
            .rotate_degrees(Vector3::new(0.0, 0.0, 1.0).normalize(), 45.0)
            .into()
        ));
    mis.flush(queue);
    mis
}

fn create_wood_floor_instance(device: &Device, queue: &Queue) -> ModelInstanceSet {

    // Creates texture from image
    let diffuse_tex = create_tex_from_file("assets/cubemap/wood_diffuse.png", device, queue, TextureFormat::Rgba8UnormSrgb);
    let specular_tex = create_tex_from_file("assets/cubemap/wood_specular.png", device, queue, TextureFormat::Rgba8UnormSrgb);
    let gloss_tex = create_tex_from_file("assets/cubemap/wood_gloss.png", device, queue, TextureFormat::Rgba8UnormSrgb);
    let normal_tex = create_tex_from_file("assets/cubemap/wood_normal.png", device, queue, TextureFormat::Rgba8Unorm);
    let material = MaterialBuilder::new()
        .diffuse(diffuse_tex)
        .specular(specular_tex)
        .gloss(gloss_tex)
        .normal(normal_tex)
        .build(&device);

    // Creates cube model
    let model = Model {
        meshes: vec![Mesh::cube(&device, Color::WHITE, Vector3::new(100.0, 100.0, 100.0))],
        materials: vec![material],
        associations: vec![(0, 0)]
    };

    // Joins model with instance data and returns
    let mut mis = ModelInstanceSet::new(&device, model, 1);
    mis.push(ModelInstance::new(Matrix4::identity()
        .translate(Vector3::new(0.0, -200.0, -0.0))
        .scale(Vector3::new(10.0, 1.0, 10.0))
        .into()
    ));
    mis.flush(queue);
    mis
}