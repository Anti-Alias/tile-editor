use std::borrow::Cow;
use std::collections::HashMap;
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};
use crate::graphics::{Material, ShaderFeatures};

/// Provides shader variants based on
pub struct ShaderModuleProvider {
    source: String,                                 // Non-preprocessed source code
    modules: HashMap<ShaderFeatures, ShaderModule>  // Preprocessed variants of `source` that are created as needed
}

impl ShaderModuleProvider {

    pub fn new(source: String) -> Self {
        Self {
            source,
            modules: HashMap::new()
        }
    }

    /// Gets cached shader module, or creates it based on features provided
    pub fn get(&mut self, device: &Device, features: &ShaderFeatures) -> &mut ShaderModule {
        let modules = &mut self.modules;
        let source = &self.source;
        modules.entry(*features).or_insert_with(|| {
            let source = Self::preprocess_source(source, features);
            let source = ShaderSource::Wgsl(Cow::from(source.as_str()));
            device.create_shader_module(&ShaderModuleDescriptor {
                label: None,
                source
            })
        })
    }

    pub fn preprocess_source(source: &str, features: &ShaderFeatures) -> String {
        let mut context = gpp::Context::new();
        let macros = &mut context.macros;
        let mat_flags = features.material_flags;
        if mat_flags & Material::DIFFUSE_BIT != 0 {
            macros.insert(String::from("DIFFUSE"), String::from("EXISTS"));
        }
        if mat_flags & Material::NORMAL_BIT != 0 {
            macros.insert(String::from("NORMAL"), String::from("EXISTS"));
        }
        gpp::process_str(source, &mut context).unwrap()
    }
}
