mod screen_buffer;
mod point_light_renderer;
mod light_renderer;
mod screen;

use egui_wgpu_backend::wgpu::RenderPassColorAttachment;
use wgpu::{Color, LoadOp, Operations, RenderPass};
pub use screen::*;
pub use screen_buffer::*;
pub use point_light_renderer::*;
pub use light_renderer::*;