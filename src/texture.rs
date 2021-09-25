use wgpu::{Sampler, TextureView, Device, Queue, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor, ImageCopyTexture, ImageDataLayout, SamplerDescriptor};
use std::rc::Rc;
use image::{DynamicImage, ImageResult, ImageError, GenericImageView};
use std::num::NonZeroU32;

/// Struct with a wgpu Texture handle, a TextureView handle and a SamplerS handle.
#[derive(Clone)]
pub struct Texture {
    pub texture: Rc<wgpu::Texture>,
    pub view: Rc<TextureView>,
    pub sampler: Rc<Sampler>,
}

impl Texture {

    /// Creates new texture from raw bytes.
    /// Guesses the image format.
    pub fn from_bytes<'a>(
        device: &Device,
        queue: &Queue,
        bytes: &[u8],
        label: Option<&'a str>
    ) -> Result<Self, ImageError> {
        let img = image::load_from_memory(bytes)?;
        Ok(Self::from_image(device, queue, &img, label))
    }

    /// Creates new texture from image.
    pub fn from_image<'a>(
        device: &Device,
        queue: &Queue,
        image: &DynamicImage,
        label: Option<&'a str>
    ) -> Texture {
        let tex = device.create_texture(&TextureDescriptor{
            label,
            size: todo!(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::COPY_DST
        });
        let dimensions = image.dimensions();
        queue.write_texture(
            ImageCopyTexture {
                texture: &tex,
                mip_level: 0,
                origin: Default::default(),
                aspect: Default::default()
            },
            image.as_bytes(),
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(NonZeroU32::new(4 * dimensions.0).unwrap()),
                rows_per_image: Some(NonZeroU32::new(dimensions.1).unwrap())
            },
            Default::default()
        );
        let view = tex.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor { ..Default::default() });
        Texture {
            texture: Rc::new(tex),
            view: Rc::new(view),
            sampler: Rc::new(sampler)
        }
    }

    fn create_with_texture(&self, texture: wgpu::Texture) -> Self {
        Self {
            texture: Rc::new(texture),
            view: self.view.clone(),
            sampler: self.sampler.clone()
        }
    }

    fn create_with_sampler(&self, sampler: Sampler) -> Self {
        Self {
            texture: self.texture.clone(),
            view: self.view.clone(),
            sampler: Rc::new(sampler)
        }
    }

    fn create_with_view(&self, view: TextureView) -> Self {
        Self {
            texture: self.texture.clone(),
            view: Rc::new(view),
            sampler: self.sampler.clone()
        }
    }
}