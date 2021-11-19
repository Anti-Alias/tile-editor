#ifdef M_DO_NOT_SET_ME
// Note: This is an 'ubershader' that must be preprocessed with 'gpp'.
// All macro variable names should be uppercase with words separated by '_'.
// All macro variable names should be prefixed with 'M_'. IE: 'M_MY_VARIABLE_NAME'.
// Macro flag variable names should be suffixed with '_ENABLED'. IE: 'M_UNICYCLES_ENABLED'.
#endif


//////////////////////////////// Vertex ////////////////////////////////
// ------------- Vertex input type -------------
struct ModelVertexIn {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] color: vec4<f32>;
    [[location(3)]] uv: vec2<f32>;
};


// ------------- Instance input type -------------
struct ModelInstanceIn {
    [[location(4)]] col0: vec4<f32>;
    [[location(5)]] col1: vec4<f32>;
    [[location(6)]] col2: vec4<f32>;
    [[location(7)]] col3: vec4<f32>;
};


// ------------- Vertex output type -------------
struct ModelVertexOut {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0]] model_position: vec4<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] color: vec4<f32>;
    [[location(3)]] uv: vec2<f32>;
};


// ------------- Uniform type(s) -------------
[[block]]
struct CameraUni {
    proj_view: mat4x4<f32>;
};


// ------------- Camera bind group -------------
[[group(0), binding(0)]]
var<uniform> camera: CameraUni;


// ------------- Texture bind group -------------
#ifdef M_NORMAL_MATERIAL_ENABLED
[[group(1), binding(M_NORMAL_TEXTURE_BINDING)]]
var norm_tex: texture_2d<f32>;
[[group(1), binding(M_NORMAL_SAMPLER_BINDING)]]
var norm_samp: sampler;
#endif

#ifdef M_DIFFUSE_MATERIAL_ENABLED
[[group(1), binding(M_DIFFUSE_TEXTURE_BINDING)]]
var diff_tex: texture_2d<f32>;
[[group(1), binding(M_DIFFUSE_SAMPLER_BINDING)]]
var diff_samp: sampler;
#endif

#ifdef M_SPECULAR_MATERIAL_ENABLED
[[group(1), binding(M_SPECULAR_TEXTURE_BINDING)]]
var spec_tex: texture_2d<f32>;
[[group(1), binding(M_SPECULAR_SAMPLER_BINDING)]]
var spec_samp: sampler;
#endif

#ifdef M_EMISSIVE_MATERIAL_ENABLED
[[group(1), binding(M_EMISSIVE_TEXTURE_BINDING)]]
var emi_tex: texture_2d<f32>;
[[group(1), binding(M_EMISSIVE_SAMPLER_BINDING)]]
var emi_samp: sampler;
#endif


// ------------- Entrypoint -------------
[[stage(vertex)]]
fn main(vertex: ModelVertexIn, instance: ModelInstanceIn) -> ModelVertexOut {
    let model_mat = mat4x4<f32>(
        instance.col0,
        instance.col1,
        instance.col2,
        instance.col3
    );
    let model_position = model_mat * vec4<f32>(vertex.position, 1.0);
    let position = camera.proj_view * model_pos;
    return ModelVertexOut(
       position,
       model_position,
       vertex.normal,
       vertex.color,
       vertex.uv
   );
}




//////////////////////////////// Fragment ////////////////////////////////
// ------------- Output type -------------
struct ColorTargetOut {
    [[location(M_POSITION_BUFFER_LOCATION)]] position: vec4<f32>;
    [[location(M_NORMAL_BUFFER_LOCATION)]] normal: vec4<f32>;
#   ifdef M_DIFFUSE_BUFFER_ENABLED
    [[location(M_DIFFUSE_BUFFER_LOCATION)]] diffuse: vec4<f32>;
#   endif
#   ifdef M_SPECULAR_BUFFER_ENABLED
    [[location(M_SPECULAR_BUFFER_LOCATION)]] specular: vec4<f32>;
#   endif
#   ifdef M_EMISSIVE_BUFFER_ENABLED
    [[location(M_EMISSIVE_BUFFER_LOCATION)]] emissive: vec4<f32>;
#   endif
};


// ------------- Entrypoint -------------
[[stage(fragment)]]
fn main(in: ModelVertexOut) -> ColorTargetOut {

    // Variables to write out to color targets
    let position = in.model_position;
    let normal = in.normal;
    var diffuse = vec4<f32>(in.color, 1.0);
    var specular = vec4<f32>(0.0);
    var emissive = vec4<f32>(0.0);

    // Alters those variables based on the material used
#   ifdef M_DIFFUSE_MATERIAL_ENABLED
    diffuse = diffuse * textureSample(diff_tex, diff_samp, in.uv);
#   endif
#   ifdef M_SPECULAR_MATERIAL_ENABLED
    specular = textureSample(spec_tex, spec_samp, in.uv);
#   endif
#   ifdef M_EMISSIVE_MATERIAL_ENABLED
    emissive = textureSample(emi_tex, emi_samp, in.uv);
#   endif

    // Outputs variables to color targets
    return ColorTargetOut(
        position,
        normal,
#       ifdef M_DIFFUSE_BUFFER_ENABLED
        diffuse,
#       endif
#       ifdef M_SPECULAR_BUFFER_ENABLED
        specular,
#       endif
#       ifdef M_EMISSIVE_BUFFER_ENABLED
        emissive,
#       endif
    );
}