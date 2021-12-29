use std::fs::File;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use egui::{Align, Button, Color32, CursorIcon, Direction, Grid, Label, Layout, ScrollArea, Style, TextEdit, Vec2, Window};
use egui::{CtxRef, SidePanel, TopBottomPanel, Frame};
use egui_wgpu_backend::RenderPass;
use epi::TextureAllocator;
use native_dialog::FileDialog;
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
            ui.separator();
            let height = ui.available_height() / 2.0 - 25.0;
            self.material.max_height = height;
            self.material.max_width = ui.available_width();
            self.material.show(ui);
            ui.vertical_centered_justified(|ui| {
                ui.set_width_range(80.0..=80.0);
                ui.horizontal(|ui| {
                    if ui.button("Add").clicked() {
                        self.texture_window_state.is_open = true;
                    }
                    let remove_button = Button::new("Remove");
                    if ui.add_enabled(self.material.has_textures(), remove_button).clicked() {
                        self.material.unset_selected_texture();
                    }
                });
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

        let window = &mut self.texture_window_state;
        let material = &mut self.material;
        let is_valid = window.is_valid();
        let TextureWindowState {
            filename_input,
            last_directory_used,
            type_choice,
            is_open,
            is_ready,
            error,
        } = window;
        let mut should_close = false;
        let should_close_ref = &mut should_close;

        Window::new("Add/Replace Texture").open(is_open).show(ctx, |ui| {
            Grid::new("grid")
                .num_columns(3)
                .spacing([10.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    // Texture file selection
                    ui.label("File");
                    ui.text_edit_singleline(filename_input);
                    let location = match last_directory_used {
                        Some(ldr) => ldr.clone().into_os_string().into_string().unwrap(),
                        None => String::from("~")
                    };
                    if ui.button("Browse").clicked() {
                        let path = FileDialog::new()
                            .set_location(&location)
                            .show_open_single_file()
                            .unwrap();
                        if let Some(path) = path {
                            if let Some(ldu) = path.parent() {
                                *last_directory_used = Some(ldu.to_path_buf());
                            }
                            *filename_input = path.into_os_string().into_string().unwrap();
                        }
                    }
                    ui.end_row();

                    // Texture type selection
                    ui.label("Type");
                    Grid::new("choice_grid").num_columns(3).show(ui, |ui| {
                        ui.radio_value(type_choice, GUITextureType::DIFFUSE, "Diffuse");
                        ui.radio_value(type_choice, GUITextureType::NORMAL, "Normal");
                        ui.radio_value(type_choice, GUITextureType::AMBIENT, "Ambient");
                        ui.end_row();
                        ui.radio_value(type_choice, GUITextureType::SPECULAR, "Specular");
                        ui.radio_value(type_choice, GUITextureType::GLOSS, "Gloss");
                        ui.radio_value(type_choice, GUITextureType::EMISSIVE, "Emissive");
                    });
                    ui.end_row();
                });
            ui.separator();

            // Add/cancel
            ui.horizontal(|ui| {
                let mut add_or_replace = String::new();
                if material.get_texture(*type_choice).is_none() {
                    add_or_replace.push_str("Add");
                }
                else {
                    add_or_replace.push_str("Replace");
                }
                if ui.add_enabled(is_valid, Button::new(add_or_replace)).clicked() {
                    let filename = filename_input.trim();
                    if let Some(name) = PathBuf::from(filename).file_name() {
                        let name = name.to_os_string().into_string().unwrap();
                        match GUITexture::from_file(filename, &name, tex_alloc) {
                            Ok(gui_tex) => {
                                material.set_texture_and_select(*type_choice, gui_tex);
                                *should_close_ref = true;
                            }
                            Err(_) => {
                                *error = Some(String::from("Failed to open file"))
                            }
                        }
                    }
                }
                if ui.button("Cancel").clicked() {
                    *should_close_ref = true;
                }
                if let Some(ref mut error) = *error {
                    let label = Label::new(error).text_color(Color32::RED);
                    ui.add(label);
                }
            });
        });

        if should_close {
            window.close();
        }
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
    error: Option<String>,
    last_directory_used: Option<PathBuf>
}

impl TextureWindowState {
    fn is_valid(&self) -> bool {
        !self.filename_input.is_empty()
    }
    fn close(&mut self) {
        self.filename_input = String::new();
        self.error = None;
        self.is_open = false;
    }
}

impl Default for GUITextureType {
    fn default() -> Self { GUITextureType::DIFFUSE }
}