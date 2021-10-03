use egui::{CtxRef, Vec2, Rgba, Align, TopBottomPanel, CentralPanel, SidePanel, Visuals, Style, TextStyle, Color32, Stroke, Widget};
use egui_wgpu_backend::epi::{Frame, Storage};
use std::time::Duration;
use egui::style::{Widgets, Selection, WidgetVisuals};
use egui::epaint::Shadow;
use egui::WidgetType::Label;
use crate::gui::Editor;

enum EditorType {
    MapEditor,
    VoxelEditor
}

pub struct GUI {
    /// All editors available
    editors: Vec<Editor>,

    /// Index of selected editor
    editor_index: i32
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            editors: vec![
                Editor::new("Editor 1", "Editor 1 Content"),
                Editor::new("Editor 2", "Editor 2 Content")
            ],
            editor_index: 0
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

impl GUI {

    fn current_editor(&self) -> Option<&Editor> {
        if (0..=1).contains(&self.editor_index) {
            Some(&self.editors[self.editor_index as usize])
        }
        else {
            None
        }
    }

    fn light() -> Style {
        let mut vis = Visuals {
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

    pub fn update(&mut self, ctx: &CtxRef) {
        ctx.set_style(Self::light());

        // Top panel
        TopBottomPanel::top("top").show(ctx, |ui| {

            // Menu bar
            egui::menu::bar(ui, |ui|{
                egui::menu::menu(ui, "File", |ui|{
                    if ui.button("New").changed() {
                        println!("New clicked!");
                    }
                    if ui.button("Open").clicked() {
                        println!("Open clicked!");
                    }
                });
                egui::menu::menu(ui, "Options", |ui|{
                    ui.button("No Options");
                });
                egui::menu::menu(ui, "Help", |ui|{
                    ui.button("Nah, dude");
                });
            });

            // Tab selector
            ui.horizontal(|ui| {
                for (i, editor) in self.editors.iter().enumerate() {
                    if ui.button(&editor.name).clicked() {
                        self.editor_index = i as i32;
                    }
                }
            });
        });

        // Bottom panel
        TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.label("Bottom");
        });

        // Left panel
        SidePanel::left("left").resizable(false).show(ctx, |ui| {
            ui.label("Left");
        });

        // Right panel
        SidePanel::right("right").resizable(false).show(ctx, |ui| {
            ui.label("Right");
        });

        // Content panel
        CentralPanel::default().show(ctx, |ui|{
            if let Some(current_editor) = self.current_editor() {
                current_editor.ui(ui);
            }
        });
    }
}