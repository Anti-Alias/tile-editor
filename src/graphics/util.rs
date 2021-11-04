use wgpu::{Device, Extent3d, Surface, SurfaceConfiguration, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor};

/// Creates a wgpu depth texture from a surface config
pub fn create_surface_depth_texture(device: &Device, _format: &TextureFormat, config: &SurfaceConfiguration) -> Texture {
    device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Depth32Float,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING
    })
}

pub fn get_texture_view_of_surface(surface: &Surface) -> TextureView {
    surface.get_current_frame().unwrap().output.texture.create_view(&TextureViewDescriptor::default())
}