use std::borrow::Cow;
use std::collections::HashMap;
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};
use crate::graphics::{Material};
use crate::graphics::util::with_lines;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ModelShaderFeatures {
    /// See material.rs for flag bits
    pub material_flags: u64
}


/// Provides shader variants derived from an 'ubershader'
/// Which variant is provided depends on the features given
/// Users should preprocess variants ahead of time via `provide_or_create`
pub struct ModelShaderProvider {
    source: String,                                     // Non-preprocessed source code
    modules: HashMap<ModelShaderFeatures, ShaderModule> // Preprocessed variants of `source` that are created as needed
}

impl ModelShaderProvider {

    /// Creates a shader provider from shader source code
    pub fn new(source: String) -> Self {
        Self {
            source,
            modules: HashMap::new()
        }
    }

    /// Creates and returns a shader module with the specified features.
    /// On subsequent invocations with the same permutation of features, the cached version wil be returned.
    pub fn prime(&mut self, device: &Device, features: &ModelShaderFeatures) -> &ShaderModule {
        let modules = &mut self.modules;
        let source = &self.source;
        modules.entry(*features).or_insert_with(move || {
            let shader = Self::create(source, device, features);
            log::info!("Created new shader");
            shader
        })
    }

    /// Gets cached shader module with given features if one is present
    pub fn provide(&self, features: &ModelShaderFeatures) -> Option<&ShaderModule> {
        self.modules.get(features)
    }

    // Preprocesses shader source code with features and creates a shader module
    fn create(source: &str, device: &Device, features: &ModelShaderFeatures) -> ShaderModule {
        let source = Self::preprocess_source(source, features);
        log::info!("Preprocessed source as:\n{}", with_lines(&source));
        let source = ShaderSource::Wgsl(Cow::from(source.as_str()));
        device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            source
        })
    }

    /// Preprocesses shader source code with specified features
    pub fn preprocess_source(source: &str, features: &ModelShaderFeatures) -> String {

        // Prepares empty preprocessor context
        let mut context = gpp::Context::new();
        let macros = &mut context.macros;
        let mat_flags = features.material_flags;
        let mut current_binding = 0;

        // Sets normal macros
        if mat_flags & Material::NORMAL_BIT != 0 {
            macros.insert(String::from("M_NORMAL_ENABLED"), String::from("TRUE"));
            macros.insert(String::from("M_NORMAL_TEXTURE_BINDING"), String::from(current_binding.to_string()));
            current_binding += 1;
            macros.insert(String::from("M_NORMAL_SAMPLER_BINDING"), String::from(current_binding.to_string()));
            current_binding += 1;
        }

        // Sets diffuse macros
        if mat_flags & Material::DIFFUSE_BIT != 0 {
            macros.insert(String::from("M_DIFFUSE_ENABLED"), String::from("TRUE"));
            macros.insert(String::from("M_DIFFUSE_TEXTURE_BINDING"), String::from(current_binding.to_string()));
            current_binding += 1;
            macros.insert(String::from("M_DIFFUSE_SAMPLER_BINDING"), String::from(current_binding.to_string()));
            current_binding += 1;
        }

        // Sets specular macros
        if mat_flags & Material::SPECULAR_BIT != 0 {
            macros.insert(String::from("M_SPECULAR_ENABLED"), String::from("TRUE"));
            macros.insert(String::from("M_SPECULAR_TEXTURE_BINDING"), String::from(current_binding.to_string()));
            current_binding += 1;
            macros.insert(String::from("M_SPECULAR_SAMPLER_BINDING"), String::from(current_binding.to_string()));
            current_binding += 1;
        }

        // Sets emissive macros
        if mat_flags & Material::EMISSIVE_BIT != 0 {
            macros.insert(String::from("M_EMISSIVE_ENABLED"), String::from("TRUE"));
            macros.insert(String::from("M_EMISSIVE_TEXTURE_BINDING"), String::from(current_binding.to_string()));
            current_binding += 1;
            macros.insert(String::from("M_EMISSIVE_SAMPLER_BINDING"), String::from(current_binding.to_string()));
            current_binding += 1;
        }

        // Returns preprocessed string
        gpp::process_str(source, &mut context).unwrap()
    }
}