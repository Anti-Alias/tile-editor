use std::fs::File;
use std::io::Read;
use epi::Frame;
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
    pub(crate) emissive: Option<GUITexture>
}

impl GUIMaterial {
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
}

/// A texture belonging to a material
#[derive(Debug, Eq, PartialEq)]
pub struct GUITexture {
    filename: String,
    texture_id: TextureId,
    width: u32,
    height: u32
}

impl GUITexture {

    /// Loads a material texture from a file.
    /// Texture is stored in the frame specified.
    pub fn from_file(filename: &str, frame: &mut Frame) -> GUITexture {
        let mut f = File::open(filename).expect("File not found");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);
        let image = image::load_from_memory(buffer.as_slice()).expect("Failed to load image from memory");
        Self::from_image(filename, &image, frame)
    }

    /// Loads a material texture from loaded client-side image.
    /// Texture is stored in the frame specified
    pub fn from_image(
        filename: &str,
        image: &DynamicImage,
        frame: &mut Frame
    ) -> GUITexture {
        let rgbai = image.to_rgba8();
        let pixels: Vec<_> = rgbai.chunks_exact(4).map(|p| {
            egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3])
        }).collect();
        let size = (rgbai.width() as usize, rgbai.height() as usize);
        let texture_id = frame.tex_allocator().alloc_srgba_premultiplied(size, pixels.as_slice());
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