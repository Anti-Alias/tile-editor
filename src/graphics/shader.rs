use std::borrow::Cow;
use std::collections::HashMap;
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};
use crate::graphics::{Material};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ShaderFeatures {
    pub material_flags: u64 // See material.rs for flag bits
}


/// Provides shader variants derived from an 'ubershader'
/// Which variant is provided depends on the features given
/// Users should preprocess variants ahead of time via `provide_or_create`
pub struct ShaderProvider {
    source: String,                                 // Non-preprocessed source code
    modules: HashMap<ShaderFeatures, ShaderModule>  // Preprocessed variants of `source` that are created as needed
}

impl ShaderProvider {

    /// Creates a shader provider from shader source code
    pub fn new(source: String) -> Self {
        Self {
            source,
            modules: HashMap::new()
        }
    }

    /// Gets cached shader module, or creates it based on features provided.
    /// Often used to preprocess a shader variant.
    pub fn provide_or_create(&mut self, device: &Device, features: &ShaderFeatures) -> &ShaderModule {
        let modules = &mut self.modules;
        let source = &self.source;
        modules.entry(*features).or_insert_with(move || {
            let shader = Self::create(source, device, features);
            log::info!("Created new shader");
            shader
        })
    }

    /// Gets cached shader module with given features if one is present
    pub fn provide(&self, features: &ShaderFeatures) -> Option<&ShaderModule> {
        self.modules.get(features)
    }

    // Preprocesses shader source code with features and creates a shader module
    fn create(source: &str, device: &Device, features: &ShaderFeatures) -> ShaderModule {
        let source = Self::preprocess_source(source, features);
        log::debug!("Preprocessed source as:\n{}", Self::source_with_lines(&source));
        let source = ShaderSource::Wgsl(Cow::from(source.as_str()));
        device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            source
        })
    }

    // Preprocesses shader source code with features
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

    fn source_with_lines(source: &str) -> String {
        let mut result = String::new();
        for (i, line) in source.lines().enumerate() {
            let header = format!("{:>4}|  ", i+1);
            result.push_str(&header);
            result.push_str(line);
            result.push('\n');
        }
        result
    }
}
