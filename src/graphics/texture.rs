use wgpu::{Sampler, TextureView, Device, Queue, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor, ImageCopyTexture, ImageDataLayout, SamplerDescriptor, Extent3d};
use std::rc::Rc;
use image::{DynamicImage, ImageResult, ImageError, GenericImageView};
use std::num::NonZeroU32;

/// Struct with a wgpu Texture handle, a TextureView handle and a Sampler handle.
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Sampler,
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

        let dimensions = image.dimensions();

        // Raw texture
        let texture = device.create_texture(&TextureDescriptor{
            label,
            size: Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::COPY_DST
        });

        // Writes data to texture
        queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
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

        // Texture view
        let view = texture.create_view(&TextureViewDescriptor::default());

        // Sampler
        let sampler = device.create_sampler(&SamplerDescriptor { ..Default::default() });

        // Done
        Texture { texture, view, sampler }
    }
}