use egui::Frame;
use epi::egui::style::{Widgets, WidgetVisuals};
use epi::egui::{TopBottomPanel, Color32, Stroke, Ui, CtxRef};
use epi::TextureAllocator;

use crate::gui::{Input, VoxelSetEditor};
use crate::gui::{Editor, MapEditor};


/// Main GUI 'n chewy
#[derive(Default)]
pub struct GUI {
    /// All editors available
    editors: Vec<EditorWithMeta>,

    /// Index of selected editor
    editor_index: Option<usize>,

    /// Window states
    map_window_state: MapWindowState,
    voxel_set_window_state: VoxelSetWindowState
}

impl GUI {

    pub fn new() -> GUI {
        GUI {
            editors: Vec::new(),
            ..Default::default()
        }
    }

    /// Creates a new GUI with an initial editor
    pub fn new_with_editor(editor: impl Editor, name: &str) -> GUI {
        let gui = GUI {
            editors: vec![EditorWithMeta {
                editor: Box::new(editor),
                name: name.to_owned()}
            ],
            editor_index: Some(0),
            ..Default::default()
        };
        gui
    }

    pub fn show(&mut self, ctx: &CtxRef, tex_alloc: &mut dyn TextureAllocator) {

        // Top panel
        TopBottomPanel::top("top").show(ctx, |ui| {
            self.show_menu_bar(ui); // Menu bar
            self.show_tabs(ui);     // Tabs
        });

        // Editor content (or default message)
        if let Some(meta) = self.current_editor() {
            meta.editor.show(ctx, tex_alloc);
        }
        else {
            epi::egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Create a new map or open an existing one!")
                });
            });
        }

        // Shows windows, and handles their selections
        self.show_windows(ctx);
        self.handle_windows();
    }

    /// Gets editor with specified name if present
    fn editor(&mut self, name: &str) -> Option<&mut EditorWithMeta> {
        for editor in &mut self.editors {
            if editor.name == name {
                return Some(editor)
            }
        }
        None
    }

    /// Gets currently selected editor, if any
    fn current_editor(&mut self) -> Option<&mut EditorWithMeta> {
        self.editor_index.map(move |idx| &mut self.editors[idx])
    }

    /// Uses editor with specified name.
    /// Returns true if change occurred, and false if not
    fn use_editor(&mut self, name: &str) -> bool {
        for (i, editor) in self.editors.iter_mut().enumerate() {
            if editor.name == name {
                self.editor_index = Some(i);
                return true
            }
        }
        false
    }

    fn show_new_map_window(&mut self, ctx: &CtxRef) {
        let input_ptr = &mut self.map_window_state.filename;
        let ready_ptr = &mut self.map_window_state.is_ready;
        let show_window_ptr = &mut self.map_window_state.is_open;
        epi::egui::Window::new("New Map").show(ctx, move |ui| {
            ui.label("Name");
            ui.text_edit_singleline(input_ptr);
            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    *ready_ptr = true;
                    *show_window_ptr = false;
                }
                if ui.button("Cancel").clicked() {
                    *show_window_ptr = false;
                }
            });
        });
    }

    fn show_new_voxel_set_window(&mut self, ctx: &CtxRef) {
        let input_ptr = &mut self.voxel_set_window_state.filename;
        let ready_ptr = &mut self.voxel_set_window_state.is_ready;
        let show_window_ptr = &mut self.voxel_set_window_state.is_open;
        epi::egui::Window::new("New Voxel Set").show(ctx, move |ui| {
            ui.label("Name");
            ui.text_edit_singleline(input_ptr);
            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    *ready_ptr = true;
                    *show_window_ptr = false;
                }
                if ui.button("Cancel").clicked() {
                    *show_window_ptr = false;
                }
            });
        });
    }

    fn show_windows(&mut self, ctx: &CtxRef) {
        if self.map_window_state.is_open {
            self.show_new_map_window(ctx);
        }
        if self.voxel_set_window_state.is_open {
            self.show_new_voxel_set_window(ctx);
        }
    }

    fn handle_windows(&mut self) {

        // Handles new map
        if self.map_window_state.is_ready {
            self.map_window_state.is_ready = false;
            let filename = std::mem::take(&mut self.map_window_state.filename);
            let filename = filename.trim();
            if !filename.is_empty() {
                let editor = EditorWithMeta {
                    editor: Box::new(MapEditor {
                        name: filename.to_owned(),
                        content: filename.to_owned()
                    }),
                    name: filename.to_owned()
                };
                self.editor_index = Some(self.editors.len());
                self.editors.push(editor);
            }
        }

        // Handles new voxel set
        if self.voxel_set_window_state.is_ready {
            self.map_window_state.is_ready = false;
            let filename = std::mem::take(&mut self.voxel_set_window_state.filename);
            let filename = filename.trim();
            if !filename.is_empty() {
                let editor = EditorWithMeta {
                    editor: Box::new(VoxelSetEditor::new(filename)),
                    name: filename.to_owned()
                };
                self.editor_index = Some(self.editors.len());
                self.editors.push(editor);
            }
        }
    }

    fn show_menu_bar(&mut self, ui: &mut Ui) {
        epi::egui::menu::bar(ui, move |ui|{
            epi::egui::menu::menu(ui, "File", move |ui|{
                if ui.button("New Map").clicked() {
                    self.map_window_state.is_open = true;
                }
                if ui.button("Open Map").clicked() {
                    // todo
                }
                if ui.button("New Voxel Set").clicked() {
                    self.voxel_set_window_state.is_open = true;
                }
                if ui.button("Open Voxel Set").clicked() {
                    // todo
                }
            });
            epi::egui::menu::menu(ui, "Options", |ui|{
                ui.button("No Options");
            });
            epi::egui::menu::menu(ui, "Help", |ui|{
                ui.button("Nah, dude");
            });
        });
    }

    fn show_tabs(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for (i, editor) in self.editors.iter().enumerate() {
                if ui.button(&editor.name).clicked() {
                    self.editor_index = Some(i);
                }
            }
        });
    }
}

// Represents the state of a window that allows for selecting a new map
#[derive(Default)]
struct MapWindowState {
    filename: String,
    is_open: bool,
    is_ready: bool
}

// Represents the state of a window that allows for selecting a new map
#[derive(Default)]
struct VoxelSetWindowState {
    filename: String,
    is_open: bool,
    is_ready: bool
}

struct EditorWithMeta {
    editor: Box<dyn Editor>,
    name: String
}