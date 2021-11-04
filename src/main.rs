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


use tile_editor::app::App;

fn main() {
    let app = App::new();
    app.start();
}