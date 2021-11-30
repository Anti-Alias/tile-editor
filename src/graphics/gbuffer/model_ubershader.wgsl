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
    [[location(0)]] model_position: vec4<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] color: vec4<f32>;
    [[location(3)]] uv: vec2<f32>;
};


// ------------- Uniform type(s) -------------
[[block]]
struct CameraUni {
    eye: vec3<f32>;
    proj_view: mat4x4<f32>;
};


// ------------- Camera bind group -------------
[[group(M_CAMERA_BIND_GROUP), binding(M_CAMERA_BINDING)]]
var<uniform> camera: CameraUni;


// ------------- Texture bind group -------------
#ifdef M_NORMAL_MATERIAL_ENABLED
[[group(M_MATERIAL_BIND_GROUP), binding(M_NORMAL_TEXTURE_BINDING)]]
var norm_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_NORMAL_SAMPLER_BINDING)]]
var norm_samp: sampler;
#endif

#ifdef M_AMBIENT_MATERIAL_ENABLED
[[group(M_MATERIAL_BIND_GROUP), binding(M_AMBIENT_TEXTURE_BINDING)]]
var amb_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_AMBIENT_SAMPLER_BINDING)]]
var amb_samp: sampler;
#endif

#ifdef M_DIFFUSE_MATERIAL_ENABLED
[[group(M_MATERIAL_BIND_GROUP), binding(M_DIFFUSE_TEXTURE_BINDING)]]
var diff_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_DIFFUSE_SAMPLER_BINDING)]]
var diff_samp: sampler;
#endif

#ifdef M_SPECULAR_MATERIAL_ENABLED
[[group(M_MATERIAL_BIND_GROUP), binding(M_SPECULAR_TEXTURE_BINDING)]]
var spec_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_SPECULAR_SAMPLER_BINDING)]]
var spec_samp: sampler;
#endif

#ifdef M_EMISSIVE_MATERIAL_ENABLED
[[group(M_MATERIAL_BIND_GROUP), binding(M_EMISSIVE_TEXTURE_BINDING)]]
var emi_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_EMISSIVE_SAMPLER_BINDING)]]
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
    let model_pos = model_mat * vec4<f32>(vertex.position, 1.0);
    let position = camera.proj_view * model_pos;
    return ModelVertexOut(
       position,
       model_pos,
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
    [[location(M_COLOR_BUFFER_LOCATION)]] color: vec4<f32>;
};


// ------------- Entrypoint -------------
[[stage(fragment)]]
fn main(in: ModelVertexOut) -> ColorTargetOut {

    // Variables to write out to color targets
    let position = in.model_position;       // X, Y, Z, <unused>
    let normal = vec4<f32>(in.normal, 1.0); // X, Y, Z, <unused>
    var color = vec4<f32>(0.0);             // ambient(rgba), diffuse(rgba), specular(rgba), emissive(rgba)

    // Alters those variables based on the material used
#   ifdef M_AMBIENT_MATERIAL_ENABLED
    let ambient = textureSample(amb_tex, amb_samp, in.uv);
    color.r = bitcast<f32>(pack4x8unorm(ambient));  // Unholy bit casting...
#   endif
#   ifdef M_DIFFUSE_MATERIAL_ENABLED
    let diffuse = in.color * textureSample(diff_tex, diff_samp, in.uv);
    color.g = bitcast<f32>(pack4x8unorm(diffuse));  // Unholy bit casting...
#   endif
#   ifdef M_SPECULAR_MATERIAL_ENABLED
    let specular = textureSample(spec_tex, spec_samp, in.uv);
    color.b = bitcast<f32>(pack4x8unorm(specular)); // Unholy bit casting...
#   endif
#   ifdef M_EMISSIVE_MATERIAL_ENABLED
    let emissive = textureSample(emi_tex, emi_samp, in.uv);
    color.a = bitcast<f32>(pack4x8unorm(emissive)); // Unholy bit casting...
#   endif

    // Outputs variables to color targets
    return ColorTargetOut(
        position,
        normal,
        color
    );
}