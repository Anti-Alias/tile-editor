use std::fs::File;
use std::io::Read;
use egui::{ScrollArea, Ui, Vec2};
use epi::{Frame, TextureAllocator};
use epi::egui::TextureId;
use image::{DynamicImage, GenericImageView};

/// Represents a collection of textures making up a single material
#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct GUIMaterial {
    pub(crate) normal: Option<GUITexture>,
    pub(crate) ambient: Option<GUITexture>,
    pub(crate) diffuse: Option<GUITexture>,
    pub(crate) specular: Option<GUITexture>,
    pub(crate) gloss: Option<GUITexture>,
    pub(crate) emissive: Option<GUITexture>,
    pub(crate) selected: GUITextureType
}

impl GUIMaterial {

    pub fn show(&self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            if self.diffuse.is_some() { ui.button("Diff"); }
            if self.normal.is_some() { ui.button("Nor"); }
            if self.ambient.is_some() { ui.button("Amb"); }
            if self.specular.is_some() { ui.button("Spec"); }
            if self.gloss.is_some() { ui.button("Gloss"); }
            if self.emissive.is_some() { ui.button("Emi"); }
        });
        ScrollArea::vertical().max_height(128.0).show(ui, |ui| {
            if let Some(gui_tex) = self.selected_texture() {
                let size = (gui_tex.width as f32, gui_tex.height as f32);
                ui.image(gui_tex.texture_id, size);
            }
        });
        ui.allocate_space(Vec2::new(0.0, 10.0));
    }

    pub(crate) fn set_texture(&mut self, typ: GUITextureType, texture: Option<GUITexture>) {
        match typ {
            GUITextureType::NORMAL => self.normal = texture,
            GUITextureType::AMBIENT => self.ambient = texture,
            GUITextureType::DIFFUSE => self.diffuse = texture,
            GUITextureType::SPECULAR => self.specular = texture,
            GUITextureType::GLOSS => self.gloss = texture,
            GUITextureType::EMISSIVE => self.emissive = texture,
        }
    }

    pub(crate) fn selected_texture(&self) -> Option<&GUITexture> {
        match self.selected {
            GUITextureType::NORMAL => self.normal.as_ref(),
            GUITextureType::AMBIENT => self.ambient.as_ref(),
            GUITextureType::DIFFUSE => self.diffuse.as_ref(),
            GUITextureType::SPECULAR => self.specular.as_ref(),
            GUITextureType::GLOSS => self.gloss.as_ref(),
            GUITextureType::EMISSIVE => self.emissive.as_ref()
        }
    }
}

/// A texture belonging to a material
#[derive(Debug, Eq, PartialEq)]
pub struct GUITexture {
    pub(crate) filename: String,
    pub(crate) texture_id: TextureId,
    pub(crate) width: u32,
    pub(crate) height: u32
}

impl GUITexture {

    /// Loads a material texture from a file.
    /// Texture is stored in the frame specified.
    pub fn from_file(filename: &str, tex_alloc: &mut dyn TextureAllocator) -> std::io::Result<GUITexture> {
        let mut f = File::open(filename)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);
        let image = image::load_from_memory(buffer.as_slice()).expect("Failed to load image from memory");
        std::io::Result::Ok(Self::from_image(filename, &image, tex_alloc))
    }

    /// Loads a material texture from loaded client-side image.
    /// Texture is stored in the frame specified
    pub fn from_image(
        filename: &str,
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
            texture_id,
            width: image.width(),
            height: image.height()
        }
    }
}

// Different texture types available
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum GUITextureType {
    NORMAL,
    AMBIENT,
    DIFFUSE,
    SPECULAR,
    GLOSS,
    EMISSIVE
}