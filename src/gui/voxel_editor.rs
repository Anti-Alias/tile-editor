use std::fs::File;
use std::hash::Hash;
use std::path::Path;
use egui::{Align, Button, Color32, CursorIcon, Direction, Grid, Label, Layout, ScrollArea, Style, TextEdit, Vec2, Window};
use egui::{CtxRef, SidePanel, TopBottomPanel, Frame};
use egui_wgpu_backend::RenderPass;
use epi::TextureAllocator;
use crate::gui::{Editor, GUIMaterial, GUITexture, Input, GUITextureType};

pub struct VoxelSetEditor {
    name: String,                               // Name of the editor
    material: GUIMaterial,                      // Material the voxels use
    texture_window_state: TextureWindowState    // GUI state for texture selection
}

impl Editor for VoxelSetEditor {

    /// Draws all panels of editor
    fn show(&mut self, ctx: &CtxRef, tex_alloc: &mut dyn TextureAllocator) {

        // Shows panels
        self.show_left_panel(ctx);
        self.show_right_panel(ctx);
        self.show_bottom_panel(ctx);
        self.show_content_panel(ctx);

        // Shows windows on top of panels
        self.show_windows(ctx, tex_alloc);
    }
}

impl VoxelSetEditor {

    /// Creates a new named editor
    pub fn new(name: &str) -> VoxelSetEditor {
        VoxelSetEditor {
            name: name.to_owned(),
            texture_window_state: TextureWindowState::default(),
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
            self.material.show(ui);
            ui.vertical_centered_justified(|ui| {
                ui.set_width_range(80.0..=80.0);
                if ui.button("Add Texture").clicked() {
                    self.texture_window_state.is_open = true;
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

    fn show_windows(&mut self, ctx: &CtxRef, tex_alloc: &mut dyn TextureAllocator) {
        if self.texture_window_state.is_open {
            self.show_texture_window(ctx, tex_alloc);
        }
    }

    fn show_texture_window(&mut self, ctx: &CtxRef, tex_alloc: &mut dyn TextureAllocator) {
        Window::new("Add Texture").show(ctx, move |ui| {
            Grid::new("grid")
                .num_columns(3)
                .spacing([10.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    // Texture file selection
                    ui.label("File");
                    ui.text_edit_singleline(&mut self.texture_window_state.filename_input);
                    ui.button("Browse");
                    ui.end_row();

                    // Texture type selection
                    ui.label("Type");
                    let texture_type_choice = &mut self.texture_window_state.type_choice;
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
                let button = Button::new("Add").enabled(self.texture_window_state.is_valid());
                if ui.add(button).clicked() {
                    let filename = self.texture_window_state.filename_input.trim();
                    match GUITexture::from_file(filename, tex_alloc) {
                        Ok(gui_tex) => {
                            let choice = self.texture_window_state.type_choice;
                            self.material.set_texture(choice, Some(gui_tex));
                            self.material.selected = choice;
                            self.texture_window_state.close();
                        }
                        Err(_) => {
                            self.texture_window_state.error = Some(String::from("Failed to open file"))
                        }
                    }
                }
                if ui.button("Cancel").clicked() {
                    self.texture_window_state.close();
                }
                if let Some(ref mut error) = self.texture_window_state.error {
                    let label = Label::new(error).text_color(Color32::RED);
                    ui.add(label);
                }
            });
        });
    }
}

fn concat(str: &str, to_push: &str) -> String {
    let mut new = str.to_owned();
    new.push_str(to_push);
    new
}

// Represents a window that allows for selecting a texture for a material
#[derive(Default)]
struct TextureWindowState {
    filename_input: String,
    type_choice: GUITextureType,
    is_open: bool,
    is_ready: bool,
    error: Option<String>
}

impl TextureWindowState {
    fn is_valid(&self) -> bool {
        !self.filename_input.is_empty()
    }
    fn close(&mut self) {
        self.error = None;
        self.is_open = false;
    }
}

impl Default for GUITextureType {
    fn default() -> Self { GUITextureType::DIFFUSE }
}