use egui_wgpu_backend::wgpu::BindingType;
use wgpu::*;
use crate::graphics::Texture;

/// A set of textures that determines how light interacts with a `Mesh`
pub struct Material {
    normal: Option<Texture>,
    diffuse: Option<Texture>,
    specular: Option<Texture>,
    emissive: Option<Texture>,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
    flags: u64
}

impl Material {

    // Bits used for constructing `flags`
    pub const NORMAL_BIT: u64 = 1;
    pub const DIFFUSE_BIT: u64 = 1 << 1;
    pub const SPECULAR_BIT: u64 = 1 << 2;
    pub const EMISSIVE_BIT: u64 = 1 << 3;

    /// Diffuse texture
    pub fn diffuse(&self) -> Option<&Texture> {
        self.diffuse.as_ref()
    }

    /// Specular texture
    pub fn specular(&self) -> Option<&Texture> {
        self.specular.as_ref()
    }

    /// Normal texture
    pub fn normal(&self) -> Option<&Texture> {
        self.normal.as_ref()
    }

    /// Bit pattern where each bit determines the presence of a texture in the material.
    /// Bit order starting from LSB: NORMAL, DIFFUSE, SPECULAR, EMISSIVE.
    /// IE:
    ///     ...001 = NORMAL
    ///     ...010 = DIFFUSE
    ///     ...011 = NORMAL + DIFFUSE
    ///     ...100 = SPECULAR
    ///     ...etc
    pub fn flags(&self) -> u64 { self.flags }

    /// Bind group of this material
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    /// Layout of the bind group of this material
    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

/// Responsible for building a `Material`
#[derive(Default)]
pub struct MaterialBuilder {
    normal: Option<Texture>,
    diffuse: Option<Texture>,
    specular: Option<Texture>,
    emissive: Option<Texture>,
    flags: u64
}

impl MaterialBuilder {

    /// Makes new builder
    pub fn new() -> MaterialBuilder {
        MaterialBuilder { ..Default::default() }
    }

    /// Adds a normal texture
    pub fn normal(mut self, normal: Texture) -> Self {
        self.normal = Some(normal);
        self.flags |= Material::NORMAL_BIT;
        self
    }

    /// Adds a diffuse texture
    pub fn diffuse(mut self, diffuse: Texture) -> Self {
        self.diffuse = Some(diffuse);
        self.flags |= Material::DIFFUSE_BIT;
        self
    }

    /// Adds a specular texture
    pub fn specular(mut self, specular: Texture) -> Self {
        self.specular = Some(specular);
        self.flags |= Material::SPECULAR_BIT;
        self
    }

    /// Adds an emissive texture
    pub fn emissive(mut self, normal: Texture) -> Self {
        self.normal = Some(normal);
        self.flags |= Material::EMISSIVE_BIT;
        self
    }

    /// Creates `Material`
    pub fn build(self, device: &Device) -> Material {
        let bind_group_layout = self.create_bind_group_layout(device);
        let bind_group = self.create_bind_group(&bind_group_layout, device);
        Material {
            diffuse: self.diffuse,
            specular: self.specular,
            normal: self.normal,
            emissive: self.emissive,
            bind_group,
            bind_group_layout,
            flags: self.flags
        }
    }

    // Number of textures stores
    fn num_textures(&self) -> u32 {
        let mut count = 0;
        if self.diffuse.is_some() { count += 1; }
        if self.specular.is_some() { count += 1; }
        if self.normal.is_some() { count += 1; }
        count
    }

    // Creates layout of bind group
    fn create_bind_group_layout(&self, device: &Device) -> BindGroupLayout {
        let num_textures = self.num_textures();
        let mut entries = Vec::with_capacity((num_textures*2) as usize);
        for i in 0..num_textures {
            entries.push(BindGroupLayoutEntry {
                binding: i*2,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false
                },
                count: None
            });
            entries.push(BindGroupLayoutEntry {
                binding: i*2+1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler {
                    filtering: true,
                    comparison: false
                },
                count: None
            });
        }
        log::debug!("Created bind group layout entries: {:?}", entries);
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Material Bind Group Layout"),
            entries: entries.as_slice()
        })
    }

    fn create_bind_group(&self, layout: &BindGroupLayout, device: &Device) -> BindGroup {
        let mut bind_group_entries = Vec::new();
        if let Some(diffuse) = self.diffuse.as_ref() {
            Self::add_entries(&mut bind_group_entries, diffuse);
        }
        if let Some(specular) = self.specular.as_ref() {
            Self::add_entries(&mut bind_group_entries, specular);
        }
        if let Some(normal) = self.normal.as_ref() {
            Self::add_entries(&mut bind_group_entries, normal);
        }
        log::debug!("Created bind group entries: {:?}", bind_group_entries);
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout,
            entries: bind_group_entries.as_slice()
        })
    }

    fn add_entries<'a>(entries: &mut Vec<BindGroupEntry<'a>>, texture: &'a Texture) {
        let len = entries.len() as u32;
        entries.push(BindGroupEntry {
            binding: len,
            resource: BindingResource::TextureView(texture.view.as_ref())
        });
        entries.push(BindGroupEntry {
            binding: len+1,
            resource: BindingResource::Sampler(texture.sampler.as_ref())
        });
    }
}