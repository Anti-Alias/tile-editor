// Note: This is an 'ubershader' that must be preprocessed with 'gpp'.
// All macro variable names should be all uppercase with words separated by '_'.
// All macro variable names should be prefixed with 'M_'. IE: 'M_MY_VARIABLE_NAME'.
// Macro flag variable names should be suffixed with '_ENABLED'. IE: 'M_UNICYCLES_ENABLED'.


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
    [[location(0)]] normal: vec3<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
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
#ifdef M_DIFFUSE_ENABLED
[[group(1), binding(M_DIFFUSE_TEXTURE_BINDING)]]
var diff_tex: texture_2d<f32>;
[[group(1), binding(M_DIFFUSE_SAMPLER_BINDING)]]
var diff_samp: sampler;
#endif

#ifdef M_SPECULAR_ENABLED
[[group(1), binding(M_SPECULAR_TEXTURE_BINDING)]]
var spec_tex: texture_2d<f32>;
[[group(1), binding(M_SPECULAR_SAMPLER_BINDING)]]
var spec_samp: sampler;
#endif

#ifdef M_NORMAL_ENABLED
[[group(1), binding(M_NORMAL_TEXTURE_BINDING)]]
var norm_tex: texture_2d<f32>;
[[group(1), binding(M_NORMAL_SAMPLER_BINDING)]]
var norm_samp: sampler;
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
    let out_pos = camera.proj_view * model_mat * vec4<f32>(vertex.position, 1.0);
    return ModelVertexOut(
       out_pos,
       vertex.normal,
       vertex.color,
       vertex.uv
   );
}




//////////////////////////////// Fragment ////////////////////////////////
// ------------- Output type -------------
struct ColorTargetOut {
    [[location(0)]] color: vec4<f32>;
};


// ------------- Entrypoint -------------
[[stage(fragment)]]
fn main(in: ModelVertexOut) -> ColorTargetOut {

    // Defines output color
    var output = vec4<f32>(0.0);

    // Applies diffuse texture
#   ifdef M_DIFFUSE_ENABLED
    output = output + textureSample(diff_tex, diff_samp, in.uv);
#   endif

    // Done
    return ColorTargetOut(output);
}