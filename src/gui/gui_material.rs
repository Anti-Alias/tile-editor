use std::fs::File;
use std::io::Read;
use egui::{Button, Color32, Image, Label, Pos2, Rect, Response, ScrollArea, Sense, Ui, Vec2, Widget};
use epi::{Frame, TextureAllocator};
use epi::egui::TextureId;
use image::{DynamicImage, GenericImageView};

/// Represents a collection of textures making up a single material
#[derive(Debug, Default, PartialEq)]
pub struct GUIMaterial {
    pub max_width: f32,
    pub max_height: f32,
    pub normal: Option<GUITexture>,
    pub ambient: Option<GUITexture>,
    pub diffuse: Option<GUITexture>,
    pub specular: Option<GUITexture>,
    pub gloss: Option<GUITexture>,
    pub emissive: Option<GUITexture>,
    pub selected: GUITextureType
}

impl GUIMaterial {

    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.set_height_range(self.max_height..=self.max_height);
            ui.horizontal_wrapped(|ui| {
                if self.diffuse.is_some() { self.show_button(ui, "Diffuse", GUITextureType::DIFFUSE); }
                if self.normal.is_some() { self.show_button(ui, "Normal", GUITextureType::NORMAL); }
                if self.ambient.is_some() { self.show_button(ui, "Ambient", GUITextureType::AMBIENT); }
                if self.specular.is_some() { self.show_button(ui, "Specular", GUITextureType::SPECULAR); }
                if self.gloss.is_some() { self.show_button(ui, "Gloss", GUITextureType::GLOSS); }
                if self.emissive.is_some() { self.show_button(ui, "Emissive", GUITextureType::EMISSIVE); }
            });
            if let Some(gui_tex) = self.selected_texture() {
                ScrollArea::both().show(ui, |ui| {
                    let size = (gui_tex.width as f32, gui_tex.height as f32);
                    let image = Image::new(gui_tex.texture_id, size);
                    add_rounded(ui, image);
                });
            }
            if let Some(gui_tex) = self.selected_texture() {
                ui.allocate_space(Vec2::new(0.0, 1.0));
                let label = Label::new(&gui_tex.name).text_color(Color32::DARK_GRAY);
                ui.add(label);
                ui.allocate_space(Vec2::new(0.0, 1.0));
            }
            ui.allocate_space(Vec2::new(0.0, 10.0));
        });
    }

    fn show_button(&mut self, ui: &mut Ui, name: &str, typ: GUITextureType) {
        let mut button = Button::new(name.to_owned());
        if self.selected == typ {
            button = button.text_color(Color32::WHITE);
        }
        if ui.add(button).clicked() {
            self.selected = typ;
        }
    }

    pub fn set_texture(&mut self, typ: GUITextureType, texture: Option<GUITexture>) {
        match typ {
            GUITextureType::NORMAL => self.normal = texture,
            GUITextureType::AMBIENT => self.ambient = texture,
            GUITextureType::DIFFUSE => self.diffuse = texture,
            GUITextureType::SPECULAR => self.specular = texture,
            GUITextureType::GLOSS => self.gloss = texture,
            GUITextureType::EMISSIVE => self.emissive = texture,
        }
    }

    pub fn unset_selected_texture(&mut self) {
        self.set_texture(self.selected, None);
        self.auto_select();
    }

    pub fn get_texture(&self, typ: GUITextureType) -> Option<&GUITexture> {
        match typ {
            GUITextureType::NORMAL => self.normal.as_ref(),
            GUITextureType::AMBIENT => self.ambient.as_ref(),
            GUITextureType::DIFFUSE => self.diffuse.as_ref(),
            GUITextureType::SPECULAR => self.specular.as_ref(),
            GUITextureType::GLOSS => self.gloss.as_ref(),
            GUITextureType::EMISSIVE => self.emissive.as_ref()
        }
    }

    pub fn selected_texture(&self) -> Option<&GUITexture> {
        self.get_texture(self.selected)
    }

    pub fn has_textures(&self) -> bool {
        if self.diffuse.is_some() { return true; }
        if self.normal.is_some() { return true; }
        if self.ambient.is_some() { return true; }
        if self.specular.is_some() { return true; }
        if self.gloss.is_some() { return true; }
        if self.emissive.is_some() { return true; }
        false
    }

    pub fn set_texture_and_select(&mut self, typ: GUITextureType, texture: GUITexture) {
        self.set_texture(typ, Some(texture));
        self.selected = typ;
    }

    fn auto_select(&mut self) {
        if self.diffuse.is_some() { self.selected = GUITextureType::DIFFUSE; return; }
        if self.normal.is_some() { self.selected = GUITextureType::NORMAL; return; }
        if self.ambient.is_some() { self.selected = GUITextureType::AMBIENT; return; }
        if self.specular.is_some() { self.selected = GUITextureType::SPECULAR; return; }
        if self.gloss.is_some() { self.selected = GUITextureType::GLOSS; return; }
        if self.emissive.is_some() { self.selected = GUITextureType::EMISSIVE; return; }
        self.selected = GUITextureType::DIFFUSE;
    }
}

/// A texture belonging to a material
#[derive(Debug, Eq, PartialEq)]
pub struct GUITexture {
    pub filename: String,
    pub name: String,
    pub texture_id: TextureId,
    pub width: u32,
    pub height: u32
}

impl GUITexture {

    /// Loads a material texture from a file.
    /// Texture is stored in the frame specified.
    pub fn from_file(filename: &str, name: &str, tex_alloc: &mut dyn TextureAllocator) -> std::io::Result<GUITexture> {
        let mut f = File::open(filename)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);
        let image = image::load_from_memory(buffer.as_slice()).expect("Failed to load image from memory");
        std::io::Result::Ok(Self::from_image(filename, name, &image, tex_alloc))
    }

    /// Loads a material texture from loaded client-side image.
    /// Texture is stored in the frame specified
    pub fn from_image(
        filename: &str,
        name: &str,
        image: &DynamicImage,
        tex_alloc: &mut dyn TextureAllocator
    ) -> GUITexture {
        let rgbai = image.to_rgba8();
        let pixels: Vec<_> = rgbai.chunks_exact(4).map(|p| {
            egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3])
        }).collect();
        let size = (rgbai.width() as usize, rgbai.height() as usize);
        let texture_id = tex_alloc.alloc_srgba_premultiplied(size, pixels.as_slice());
        GUITexture {
            filename: filename.to_owned(),
            name: name.to_owned(),
            texture_id,
            width: image.width(),
            height: image.height()
        }
    }
}

// Different texture types available
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GUITextureType {
    NORMAL,
    AMBIENT,
    DIFFUSE,
    SPECULAR,
    GLOSS,
    EMISSIVE
}

fn add_rounded(ui: &mut Ui, image: Image) -> Response {
    let (mut rect, response) = ui.allocate_exact_size(image.size(), Sense::hover());
    rect.min = rect.min.round();
    rect.max = rect.max.round();
    image.paint_at(ui, rect);
    response
}