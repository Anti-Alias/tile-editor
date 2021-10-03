use egui::{CtxRef, Vec2, Rgba, Align, TopBottomPanel, CentralPanel, SidePanel, Visuals, Style, TextStyle, Color32, Stroke, Widget};
use egui_wgpu_backend::epi::{Frame, Storage};
use std::time::Duration;
use egui::style::{Widgets, Selection, WidgetVisuals};
use egui::epaint::Shadow;
use egui::WidgetType::Label;

pub struct GUI {
}

impl Default for GUI {
    fn default() -> Self {
        Self {}
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

    pub fn update(&mut self, ctx: &CtxRef) {
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
        ctx.set_style(Style {
            body_text_style: TextStyle::Monospace,
            override_text_style: Some(TextStyle::Monospace),
            wrap: None,
            spacing: Default::default(),
            interaction: Default::default(),
            visuals: vis,
            animation_time: 0.0,
            debug: Default::default()
        });
        TopBottomPanel::top("top").show(ctx, |ui| {
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
        });
        TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.label("Bottom");
        });
        SidePanel::left("left").resizable(false).show(ctx, |ui| {
            ui.label("Left");
        });
        SidePanel::right("right").resizable(false).show(ctx, |ui| {
            ui.label("Right");
        });
        CentralPanel::default().show(ctx, |ui|{
            ui.label("Center");
        });
    }
}