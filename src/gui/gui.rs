use egui::{CtxRef, TopBottomPanel, Color32, Stroke, Ui};
use egui::style::{Widgets, WidgetVisuals};

use crate::gui::Editor;


/// Main GUI 'n chewy
pub struct GUI {
    /// All editors available
    editors: Vec<Editor>,

    /// Index of selected editor
    editor_index: usize,

    /// Menu flags
    window_flags: MenuFlags,

    /// Selections from menus, checkboxes, etc
    inputs: GUIInputs
}

impl GUI {

    pub fn new(starting_editor: Editor) -> GUI {
        GUI {
            editors: vec![starting_editor],
            ..Default::default()
        }
    }

    fn current_editor(&self) -> Option<&Editor> {
        if (0..self.editors.len()).contains(&self.editor_index) {
            Some(&self.editors[self.editor_index])
        }
        else {
            None
        }
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
        egui::Window::new("New Map")
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
                self.editor_index = self.editors.len();
                self.editors.push(Editor {
                    name: filename.to_owned(),
                    content: filename.to_owned()
                });
            }
        }
    }

    fn show_menu_bar(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui|{
            egui::menu::menu(ui, "File", |ui|{
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
            egui::menu::menu(ui, "Options", |ui|{
                ui.button("No Options");
            });
            egui::menu::menu(ui, "Help", |ui|{
                ui.button("Nah, dude");
            });
        });
    }

    fn show_tabs(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for (i, editor) in self.editors.iter().enumerate() {
                if ui.button(&editor.name).clicked() {
                    self.editor_index = i;
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
        if let Some(editor) = self.current_editor() {
            editor.left_panel(ctx);     // Left panel
            editor.right_panel(ctx);    // Right panel
            editor.bottom_panel(ctx);   // Bottom panel
            editor.content_panel(ctx);  // Content panel
        }
        else {
            egui::CentralPanel::default().show(ctx, |ui| {
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
            editor_index: 0,
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

/// Represents input to be filled out.
/// Should only be consumed when `is_ready` is set to true.
struct Input<T: Default> {
    pub is_ready: bool,
    pub data: T
}

impl<T: Default> Input<T> {

    /// Consumes the input, setting the `is_ready` to false if
    pub fn consume(&mut self) -> Option<&T> {
        if self.is_ready {
            self.is_ready = false;
            Some(&self.data)
        }
        else {
            None
        }
    }
}

impl<T: Default> Default for Input<T> {
    fn default() -> Self {
        Input { is_ready: false, data: Default::default() }
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