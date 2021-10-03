use egui::{CtxRef, Vec2, Rgba, Align, TopBottomPanel, CentralPanel, SidePanel, Visuals, Style, TextStyle, Color32, Stroke, Widget};
use egui_wgpu_backend::epi::{Frame, Storage};
use std::time::Duration;
use egui::style::{Widgets, Selection, WidgetVisuals};
use egui::epaint::Shadow;
use egui::WidgetType::Label;

pub struct GUI {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
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

impl epi::App for GUI {

    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
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

    fn name(&self) -> &str {
        "egui template"
    }
}

impl GUI {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update_old(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { label, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("egui template");
            ui.hyperlink("https://github.com/emilk/egui_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/egui_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if true {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn name(&self) -> &str {
        "egui template"
    }
}