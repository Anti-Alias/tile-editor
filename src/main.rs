use std::iter;
use std::time::Instant;
use chrono::Timelike;
use egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use futures_lite::future::block_on;
use wgpu::{TextureFormat, TextureViewDescriptor};

use winit::event::Event::*;
use winit::event_loop::{ControlFlow};


use tile_editor::graphics::{Color, create_surface_depth_texture, get_texture_view_of_surface, Material, Mesh, Model, ModelFrameBuffer, ModelRenderer};
use tile_editor::gui::{GUI, Editor};

const INITIAL_WIDTH: u32 = 640;
const INITIAL_HEIGHT: u32 = 480;
const DEPTH_STENCIL_FORMAT: TextureFormat = TextureFormat::Depth32Float;

fn main() {

    // Initializes logger
    env_logger::init();

    // Creates WINIT window and event loop
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("Tile Editor")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
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

    // Sets up model renderer and model
    let mut renderer = ModelRenderer::new(surface_config.format, DEPTH_STENCIL_FORMAT);
    let model = Model {
        meshes: vec![Mesh::cube(&device, Color::RED)],
        materials: vec![Material::empty()],
        associations: vec![(0, 0)]
    };
    renderer.prepare_for_model(&device, &model);
    let mut depth_stencil = create_surface_depth_texture(&device, &DEPTH_STENCIL_FORMAT, &surface_config);
    let mut depth_stencil_view = depth_stencil.create_view(&TextureViewDescriptor::default());

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

    // Main loop
    event_loop.run(move |event, _, control_flow| {

        // Pass the winit events to the platform integration.
        platform.handle_event(&event);

        match event {
            RedrawRequested(..) => {

                // Gets texture view of surface for drawing on
                let surface_frame = match surface.get_current_frame() {
                    Ok(frame) => frame,
                    Err(_) => { return }
                };
                let surface_view = surface_frame
                    .output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                // Draws with renderer
                let fbo = ModelFrameBuffer {
                    color: &surface_view,
                    depth_stencil: &depth_stencil_view
                };
                renderer.render(&model, &device, &queue, &fbo);

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
            }
            MainEventsCleared => {
                window.request_redraw();
            }
            WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    if size.width != 0 { surface_config.width = size.width; }
                    if size.height != 0 { surface_config.height = size.height; }
                    surface.configure(&device, &surface_config);
                    depth_stencil = create_surface_depth_texture(&device, &DEPTH_STENCIL_FORMAT, &surface_config);
                    depth_stencil_view = depth_stencil.create_view(&TextureViewDescriptor::default());
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

/// Time of day as seconds since midnight. Used for clock in demo app.
pub fn seconds_since_midnight() -> f64 {
    let time = chrono::Local::now().time();
    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}