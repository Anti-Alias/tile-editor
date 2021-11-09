use egui_wgpu_backend::wgpu::BindingType;
use wgpu::*;
use crate::graphics::Texture;

/// A set of textures that determines how light interacts with a `Mesh`
pub struct Material {
    diffuse: Option<Texture>,
    specular: Option<Texture>,
    normal: Option<Texture>,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout
}

/// Responsible for building a `Material`
pub struct MaterialBuilder {
    diffuse: Option<Texture>,
    specular: Option<Texture>,
    normal: Option<Texture>
}

impl MaterialBuilder {

    /// Makes new builder
    pub fn new() -> MaterialBuilder {
        MaterialBuilder {
            diffuse: None,
            specular: None,
            normal: None
        }
    }

    /// Adds a diffuse texture
    pub fn diffuse(mut self, diffuse: Texture) -> Self {
        self.diffuse = Some(diffuse);
        self
    }

    /// Adds a specular texture
    pub fn specular(mut self, specular: Texture) -> Self {
        self.specular = Some(specular);
        self
    }

    /// Adds a normal texture
    pub fn normal(mut self, normal: Texture) -> Self {
        self.normal = Some(normal);
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
            bind_group,
            bind_group_layout
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
        let entries = (0..self.num_textures())
            .map(|binding| {
                BindGroupLayoutEntry {
                    binding,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: Default::default(),
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false
                    },
                    count: None
                }
            })
            .collect::<Vec<_>>();
        log::debug!("Created bind group layout with {} entries", entries.len());
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Material Bind Group Layout"),
            entries: entries.as_slice()
        })
    }

    fn create_bind_group(&self, layout: &BindGroupLayout, device: &Device) -> BindGroup {
        let mut entries = Vec::new();
        let mut binding: u32 = 0;
        if let Some(diffuse) = self.diffuse.as_ref() {
            entries.push(Self::create_bind_group_entry(binding, diffuse.view.as_ref()));
            binding += 1;
        }
        if let Some(specular) = self.specular.as_ref() {
            entries.push(Self::create_bind_group_entry(binding, specular.view.as_ref()));
            binding += 1;
        }
        if let Some(normal) = self.normal.as_ref() {
            entries.push(Self::create_bind_group_entry(binding, normal.view.as_ref()));
            binding += 1;
        }
        log::debug!("Created bind group with {} entries", entries.len());
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout,
            entries: entries.as_slice()
        })
    }

    fn create_bind_group_entry(binding: u32, view: &TextureView) -> BindGroupEntry {
        BindGroupEntry {
            binding,
            resource: BindingResource::TextureView(view)
        }
    }
}

impl Material {
    /// Determines if diffuse texture will be used
    pub const DIFFUSE_BIT: u64 = 1;

    /// Determines if specular texture will be used
    pub const SPECULAR_BIT: u64 = 1 << 1;

    /// Determines if normal texture will be used
    pub const NORMAL_BIT: u64 = 1 << 2;

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
    /// Bit order starting from LSB: DIFFUSE, SPECULAR, NORMAL.
    /// IE:
    ///     ...001 = DIFFUSE
    ///     ...010 = SPECULAR
    ///     ...011 = DIFFUSE + SPECULAR
    ///     ...100 = NORMAL
    ///     ...etc
    pub fn flags(&self) -> u64 {
        let mut result = 0;
        result |= self.diffuse.is_some() as u64;
        result |= (self.specular.is_some() as u64) << 1;
        result |= (self.normal.is_some() as u64) << 2;
        result
    }

    /// Bind group of this material
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    /// Layout of the bind group of this material
    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}