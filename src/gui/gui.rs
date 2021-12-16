use epi::egui::style::{Widgets, WidgetVisuals};
use epi::egui::{TopBottomPanel, Color32, Stroke, Ui, CtxRef};

use crate::gui::{Editor, SimpleEditor};
use crate::gui::{Input};


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

    fn current_editor(&self) -> Option<&EditorWithMeta> {
        self.editor_index.map(|idx| &self.editors[idx])
    }

    /*
    fn light_style() -> Style {
        let vis = Visuals {
            dark_mode: false,
            widgets: light_widget_style(),
            selection: Selection {
                bg_fill: Color32::from_rgb(144, 209, 255),
                stroke: Stroke::new(1.0, Color32::from_rgb(0, 83, 125)),
            },
            hyperlink_color: Color32::from_rgb(0, 155, 255),
            faint_bg_color: Color32::from_gray(240),
            extreme_bg_color: Color32::from_gray(250),
            code_bg_color: Color32::from_gray(200),
            window_shadow: Shadow::big_light(),
            popup_shadow: Shadow::small_light(),
            ..Visuals::dark()
        };
        Style {
            body_text_style: TextStyle::Monospace,
            override_text_style: Some(TextStyle::Monospace),
            wrap: None,
            spacing: Default::default(),
            interaction: Default::default(),
            visuals: vis,
            animation_time: 0.0,
            debug: Default::default()
        }
    }
     */

    fn show_new_map_menu(&mut self, ctx: &CtxRef) {
        let opened = &mut self.window_flags.new_map_opened;
        let input = &mut self.inputs.new_map_input.data;
        let ready = &mut self.inputs.new_map_input.is_ready;
        epi::egui::Window::new("New Map")
            .show(ctx, move |ui| {
                ui.label("Name");
                ui.text_edit_singleline(input);
                ui.horizontal(|ui| {
                    if ui.button("Create").clicked() {
                        *ready = true;
                        *opened = false;
                    }
                    if ui.button("Cancel").clicked() {
                        *opened = false;
                    }
                });
            });
    }

    fn show_windows(&mut self, ctx: &CtxRef) {

        // "New Map" Window
        if self.window_flags.new_map_opened {
            self.show_new_map_menu(ctx);
        }
    }

    fn handle_inputs(&mut self) {

        // Handles new map
        if let Some(filename) = self.inputs.new_map_input.consume() {
            let filename = filename.trim();
            if !filename.is_empty() {
                self.editor_index = Some(self.editors.len());
                let editor = EditorWithMeta {
                    editor: Box::new(SimpleEditor {
                        name: filename.to_owned(),
                        content: filename.to_owned()
                    }),
                    name: filename.to_owned()
                };
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
                    // todo
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

    pub fn update(&mut self, ctx: &CtxRef) {

        // Use light style in subsequent draw calls
        //ctx.set_style(Self::light_style());

        // Shows menus hovering over UI
        self.show_windows(ctx);

        // Top panel
        TopBottomPanel::top("top").show(ctx, |ui| {
            self.show_menu_bar(ui); // Menu bar
            self.show_tabs(ui);     // Tabs
        });

        // Editor panels
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
struct MenuFlags {
    new_map_opened: bool
}

impl Default for MenuFlags {
    fn default() -> Self {
        Self {
            new_map_opened: false
        }
    }
}


/// A set of inputs
struct GUIInputs {

    /// Selection for a new map.
    new_map_input: Input<String>
}

impl Default for GUIInputs {
    fn default() -> Self {
        Self {
            new_map_input: Default::default()
        }
    }
}

pub fn light_widget_style() -> Widgets {
    Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: Color32::from_gray(235), // window background
            bg_stroke: Stroke::new(1.0, Color32::from_gray(190)), // separators, indentation lines, windows outlines
            fg_stroke: Stroke::new(1.0, Color32::from_gray(100)), // normal text color
            corner_radius: 2.0,
            expansion: 0.0,
        },
        inactive: WidgetVisuals {
            bg_fill: Color32::from_gray(215), // button background
            bg_stroke: Default::default(),
            fg_stroke: Stroke::new(1.0, Color32::from_gray(80)), // button text
            corner_radius: 2.0,
            expansion: 0.0,
        },
        hovered: WidgetVisuals {
            bg_fill: Color32::from_gray(210),
            bg_stroke: Stroke::new(1.0, Color32::from_gray(105)), // e.g. hover over window edge or button
            fg_stroke: Stroke::new(1.5, Color32::BLACK),
            corner_radius: 3.0,
            expansion: 1.0,
        },
        active: WidgetVisuals {
            bg_fill: Color32::from_gray(165),
            bg_stroke: Stroke::new(1.0, Color32::BLACK),
            fg_stroke: Stroke::new(2.0, Color32::BLACK),
            corner_radius: 2.0,
            expansion: 1.0,
        },
        open: WidgetVisuals {
            bg_fill: Color32::from_gray(220),
            bg_stroke: Stroke::new(1.0, Color32::from_gray(160)),
            fg_stroke: Stroke::new(1.0, Color32::BLACK),
            corner_radius: 2.0,
            expansion: 0.0,
        },
    }
}

struct EditorWithMeta {
    editor: Box<dyn Editor>,
    name: String
}