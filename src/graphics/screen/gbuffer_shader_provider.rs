use std::borrow::Cow;
use std::collections::HashMap;
use wgpu::*;
use crate::graphics::gbuffer::GBuffer;
use crate::graphics::util::string_with_lines;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct GBufferShaderFeatures {
    pub gbuffer_flags: u64
}

pub struct GBufferShaderProvider {
    source: String,                                         // Non-preprocessed source code
    modules: HashMap<GBufferShaderFeatures, ShaderModule>   // Preprocessed variants of `source` that are created as needed
}

impl GBufferShaderProvider {

    /// Creates a shader provider from shader source code
    pub fn new(source: String) -> Self {
        Self {
            source,
            modules: HashMap::new()
        }
    }

    /// Creates and returns a shader module with the specified features.
    /// On subsequent invocations with the same permutation of features, the cached version wil be returned.
    pub fn prime(&mut self, device: &Device, features: &GBufferShaderFeatures) -> &ShaderModule {
        let modules = &mut self.modules;
        let source = &self.source;
        modules.entry(*features).or_insert_with(move || {
            let shader = Self::create(source, device, features);
            log::info!("Created new shader");
            shader
        })
    }

    // Preprocesses shader source code with features and creates a shader module
    fn create(source: &str, device: &Device, features: &GBufferShaderFeatures) -> ShaderModule {
        let source = Self::preprocess_source(source, features);
        log::info!("Preprocessed gbuffer shader source as:\n{}", string_with_lines(&source));
        let source = ShaderSource::Wgsl(Cow::from(source.as_str()));
        device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            source
        })
    }

    // Preprocesses shader source code with specified features
    pub fn preprocess_source(source: &str, features: &GBufferShaderFeatures) -> String {

        // Prepares empty preprocessor context
        let mut context = gpp::Context::new();
        let macros = &mut context.macros;

        // ---------- GBuffer texture bind group -----------
        macros.insert(String::from("M_GBUFFER_BIND_GROUP"), String::from("0"));
        macros.insert(String::from("M_SAMPLER_BINDING"), String::from("0"));
        macros.insert(String::from("M_POSITION_TEXTURE_BINDING"), String::from("1"));
        macros.insert(String::from("M_NORMAL_TEXTURE_BINDING"), String::from("2"));
        if features.gbuffer_flags & GBuffer::COLOR_BUFFER_BIT != 0 {
            macros.insert(String::from("M_COLOR_BUFFER_ENABLED"), String::from("TRUE"));
            macros.insert(String::from("M_COLOR_TEXTURE_BINDING"), String::from("3"));
        }

        // Returns preprocessed string
        gpp::process_str(source, &mut context).unwrap()
    }
}