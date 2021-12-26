use epi::egui::style::{Widgets, WidgetVisuals};
use epi::egui::{TopBottomPanel, Color32, Stroke, Ui, CtxRef};

use crate::gui::{Input, VoxelEditor};
use crate::gui::{Editor, MapEditor};


/// Main GUI 'n chewy
pub struct GUI {
    /// All editors available
    editors: Vec<EditorWithMeta>,

    /// Index of selected editor
    editor_index: Option<usize>,

    /// Menu flags
    window_flags: MenuFlags,

    /// Selections from menus, checkboxes, etc
    inputs: GUIInputs
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

    pub fn show(&mut self, ctx: &CtxRef) {

        // Shows menus hovering over UI
        self.show_windows(ctx);

        // Top panel
        TopBottomPanel::top("top").show(ctx, |ui| {
            self.show_menu_bar(ui); // Menu bar
            self.show_tabs(ui);     // Tabs
        });

        // Editor panel
        if let Some(meta) = self.current_editor() {
            meta.editor.show(ctx);
        }
        else {
            epi::egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Create a new map or open an existing one!")
                });
            });
        }

        // Handles selections
        self.handle_inputs();
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
    fn current_editor(&self) -> Option<&EditorWithMeta> {
        self.editor_index.map(|idx| &self.editors[idx])
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

    fn show_new_map_menu(&mut self, ctx: &CtxRef) {
        let window_opened_ptr = &mut self.window_flags.new_map_opened;
        let input_ptr = &mut self.inputs.map_input.data;
        let input_ready_ptr = &mut self.inputs.map_input.is_ready;
        epi::egui::Window::new("New Map").show(ctx, move |ui| {
            ui.label("Name");
            ui.text_edit_singleline(input_ptr);
            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    *input_ready_ptr = true;
                    *window_opened_ptr = false;
                }
                if ui.button("Cancel").clicked() {
                    *window_opened_ptr = false;
                }
            });
        });
    }

    fn show_new_voxel_set_menu(&mut self, ctx: &CtxRef) {
        let window_opened_ptr = &mut self.window_flags.voxel_set_opened;
        let input_ptr = &mut self.inputs.voxel_set_input.data;
        let input_ready_ptr = &mut self.inputs.voxel_set_input.is_ready;
        epi::egui::Window::new("New Voxel Set").show(ctx, move |ui| {
            ui.label("Name");
            ui.text_edit_singleline(input_ptr);
            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    *input_ready_ptr = true;
                    *window_opened_ptr = false;
                }
                if ui.button("Cancel").clicked() {
                    *window_opened_ptr = false;
                }
            });
        });
    }

    fn show_windows(&mut self, ctx: &CtxRef) {
        if self.window_flags.new_map_opened {
            self.show_new_map_menu(ctx);
        }
        if self.window_flags.voxel_set_opened {
            self.show_new_voxel_set_menu(ctx);
        }
    }

    fn handle_inputs(&mut self) {

        // Handles new map
        if let Some(filename) = self.inputs.map_input.consume() {
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
        if let Some(filename) = self.inputs.voxel_set_input.consume() {
            let filename = filename.trim();
            if !filename.is_empty() {
                let editor = EditorWithMeta {
                    editor: Box::new(VoxelEditor::new()),
                    name: filename.to_owned()
                };
                self.editor_index = Some(self.editors.len());
                self.editors.push(editor);
            }
        }
    }

    fn show_menu_bar(&mut self, ui: &mut Ui) {
        epi::egui::menu::bar(ui, |ui|{
            epi::egui::menu::menu(ui, "File", |ui|{
                if ui.button("New Map").clicked() {
                    self.window_flags.new_map_opened = true;
                }
                if ui.button("Open Map").clicked() {
                    // todo
                }
                if ui.button("New Voxel Set").clicked() {
                    self.window_flags.voxel_set_opened = true;
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

impl Default for GUI {
    fn default() -> Self {
        Self {
            editors: vec![],
            editor_index: None,
            window_flags: MenuFlags::default(),
            inputs: GUIInputs::default()
        }
    }
}

/// Stores flags regarding the "opened" status of menus in `GUI`
#[derive(Default)]
struct MenuFlags {
    new_map_opened: bool,
    voxel_set_opened: bool
}

/// A set of inputs
#[derive(Default)]
struct GUIInputs {
    map_input: Input<String>,
    voxel_set_input: Input<String>
}


struct EditorWithMeta {
    editor: Box<dyn Editor>,
    name: String
}