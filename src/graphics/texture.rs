use wgpu::{Sampler, TextureView, Device, Queue, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor, ImageCopyTexture, ImageDataLayout, SamplerDescriptor, TextureAspect, Origin3d};
use std::rc::Rc;
use image::{DynamicImage, ImageError, GenericImageView};
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
        let rgba = image.as_rgba8().unwrap();
        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        // Raw texture
        let texture = device.create_texture(&TextureDescriptor{
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST
        });

        // Writes data to texture
        queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All
            },
            rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(NonZeroU32::new(4 * dimensions.0).unwrap()),
                rows_per_image: Some(NonZeroU32::new(dimensions.1).unwrap())
            },
            size
        );

        // Creates view and sampler, then finishes
        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        Texture {
            texture: Rc::new(texture),
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