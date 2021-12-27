use std::hash::Hash;
use egui::{Align, Button, CursorIcon, Direction, Grid, Layout, ScrollArea, Style, TextEdit, Vec2, Window};
use egui::{CtxRef, SidePanel, TopBottomPanel, Frame};
use egui_wgpu_backend::RenderPass;
use epi::TextureAllocator;
use crate::gui::{Editor, GUIMaterial, GUITexture, Input, GUITextureType};

pub struct VoxelEditor {
    name: String,                   // Name of the editor
    material: GUIMaterial,          // Material the voxels use
    texture_state: TextureState     // GUI state for texture selection
}

impl Editor for VoxelEditor {

    /// Draws all panels of editor
    fn show(&mut self, ctx: &CtxRef, tex_alloc: &mut dyn TextureAllocator) {
        self.show_left_panel(ctx);
        self.show_right_panel(ctx);
        self.show_bottom_panel(ctx);
        self.show_content_panel(ctx);
        self.show_windows(ctx);
    }
}

impl VoxelEditor {

    /// Creates a new named editor
    pub fn new(name: &str) -> VoxelEditor {
        VoxelEditor {
            name: name.to_owned(),
            texture_state: TextureState::default(),
            material: GUIMaterial::default()
        }
    }

    // Draws left panel
    fn show_left_panel(&self, ctx: &CtxRef) {
        let name = concat(&self.name, "_left");
        SidePanel::left(&name).resizable(true).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Left");
            });
        });
    }

    // Draws right panel
    fn show_right_panel(&mut self, ctx: &CtxRef) {
        let name = concat(&self.name, "_right");
        SidePanel::right(&name).show(ctx, |ui| {

            // ----- Materials -----
            ui.vertical_centered(|ui| {
                ui.heading("Material");
            });
            let height = ui.available_height()/2.0 - 15.0;
            ui.separator();
            ScrollArea::from_max_height(height).show(ui, |ui| {
                ui.set_min_height(height);
            });
            ui.vertical_centered_justified(|ui| {
                ui.set_width_range(80.0..=80.0);
                if ui.button("Add Texture").clicked() {
                    self.texture_state.show_window = true;
                }
            });
            ui.separator();

            // ----- Voxels -----
            ui.vertical_centered(|ui| {
                ui.heading("Voxels");
            });
        });
    }

    // Draws bottom panel
    fn show_bottom_panel(&self, ctx: &CtxRef) {
        let name = concat(&self.name, "_bottom");
        TopBottomPanel::bottom(&name).resizable(true).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Bottom");
            });
        });
    }

    // Draws content panel
    fn show_content_panel(&self, _ctx: &CtxRef) {
        /*
        CentralPanel::default().show(ctx, |ui|{
            ui.label(self.content.as_str())
        });
         */
    }

    fn show_windows(&mut self, ctx: &CtxRef) {
        if self.texture_state.show_window {
            self.show_texture_window(ctx);
        }
    }

    fn show_texture_window(&mut self, ctx: &CtxRef) {
        Window::new("Add Texture").show(ctx, move |ui| {
            Grid::new("grid")
                .num_columns(3)
                .spacing([10.0, 10.0])
                .striped(true)
                .show(ui, |ui| {

                    // Texture file selection
                    ui.label("File");
                    ui.text_edit_singleline(&mut self.texture_state.filename_input.data);
                    ui.button("Browse");
                    ui.end_row();

                    // Texture type selection
                    ui.label("Type");
                    let texture_type_choice = &mut self.texture_state.type_choice;
                    Grid::new("choice_grid").num_columns(3).show(ui, |ui| {
                        ui.radio_value(texture_type_choice, GUITextureType::DIFFUSE, "Diffuse");
                        ui.radio_value(texture_type_choice, GUITextureType::NORMAL, "Normal");
                        ui.radio_value(texture_type_choice, GUITextureType::AMBIENT, "Ambient");
                        ui.end_row();
                        ui.radio_value(texture_type_choice, GUITextureType::SPECULAR, "Specular");
                        ui.radio_value(texture_type_choice, GUITextureType::GLOSS, "Gloss");
                        ui.radio_value(texture_type_choice, GUITextureType::EMISSIVE, "Emissive");
                    });
                    ui.end_row();
                });
            ui.separator();

            // Add/cancel
            ui.horizontal(move |ui| {
                if ui.button("Add").clicked() {
                    let filename = self.texture_state.filename_input.consume();
                    println!("Filename is {:?}", filename);
                    if let Some(filename) = filename {
                        let gui_tex = Self::load_gui_texture(&filename);
                        self.material.set_texture(self.texture_state.type_choice, Some(gui_tex));
                        self.texture_state.show_window = false;
                    }
                }
                if ui.button("Cancel").clicked() {
                    self.texture_state.show_window = false;
                }
            });
        });
    }

    fn add_texture(&mut self, filename: &str, typ: GUITextureType) {
        match typ {
            GUITextureType::NORMAL => {

            }
            GUITextureType::AMBIENT => {}
            GUITextureType::DIFFUSE => {}
            GUITextureType::SPECULAR => {}
            GUITextureType::GLOSS => {}
            GUITextureType::EMISSIVE => {}
        }
    }

    fn load_gui_texture(filename: &str) -> GUITexture {
        todo!()
    }
}

fn concat(str: &str, to_push: &str) -> String {
    let mut new = str.to_owned();
    new.push_str(to_push);
    new
}

// Represents a window that allows for selecting a texture for a material
#[derive(Default)]
struct TextureState {
    filename_input: Input<String>,
    type_choice: GUITextureType,
    show_window: bool
}

impl Default for GUITextureType {
    fn default() -> Self { GUITextureType::DIFFUSE }
}