use std::iter;
use std::time::Instant;

use chrono::Timelike;
use egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use futures_lite::future::block_on;
use winit::event::Event::*;
use winit::event_loop::ControlFlow;
use tile_editor::GUI;

const INITIAL_WIDTH: u32 = 640;
const INITIAL_HEIGHT: u32 = 480;

enum Event {
    RequestRedraw
}

struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);
impl epi::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}

fn main() {

    // Creates winit event loop and window
    let event_loop = winit::event_loop::EventLoop::with_user_event();
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

    // Creates WPU instance and gets surface
    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };

    // Selects GPU adapter (physical device)
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
    }))
    .unwrap();

    // Selects device (logical device) and queue
    let (mut device, mut queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None,
    ))
    .unwrap();

    // Configures surface
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

    // We use the egui_winit_platform crate as the platform.
    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: size.width as u32,
        physical_height: size.height as u32,
        scale_factor: window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    // We use the egui_wgpu_backend crate as the render backend.
    let mut egui_rpass = RenderPass::new(&device, surface_format, 1);
    let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal(std::sync::Mutex::new(
        event_loop.create_proxy(),
    )));

    // Makes GUI instance
    let mut gui = GUI::default();

    let start_time = Instant::now();
    let mut previous_frame_time = None;
    event_loop.run(move |event, _, control_flow| {

        // Pass the winit events to the platform integration.
        platform.handle_event(&event);

        match event {
            RedrawRequested(..) => {

                // Gets texture view of surface for drawing on
                let output_frame = match surface.get_current_frame() {
                    Ok(frame) => frame,
                    Err(_) => { return }
                };
                let output_view = output_frame
                    .output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                // Begin EGUI frame
                let egui_start = Instant::now();
                platform.update_time(start_time.elapsed().as_secs_f64());   // Tells platform start time
                platform.begin_frame();                                     // Begins frame (???)

                // Draw GUI to frame
                let mut app_output = epi::backend::AppOutput::default();
                let mut frame = epi::backend::FrameBuilder {
                    info: epi::IntegrationInfo {
                        web_info: None,
                        cpu_usage: previous_frame_time,
                        seconds_since_midnight: Some(seconds_since_midnight()),
                        native_pixels_per_point: Some(window.scale_factor() as _),
                        prefer_dark_mode: Some(false),
                    },
                    tex_allocator: &mut egui_rpass,
                    output: &mut app_output,
                    repaint_signal: repaint_signal.clone(),
                }.build();
                gui.update(&platform.context(), &mut frame);

                // End EGUI frame
                let (_output, paint_commands) = platform.end_frame(Some(&window));
                let paint_jobs = platform.context().tessellate(paint_commands);
                let frame_time = (Instant::now() - egui_start).as_secs_f64() as f32;
                previous_frame_time = Some(frame_time);

                // Encode frame draws to render pass
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
                    &output_view,
                    &paint_jobs,
                    &screen_descriptor,
                    Some(wgpu::Color::BLACK),
                ).unwrap();

                // Submit the commands.
                queue.submit(iter::once(encoder.finish()));
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