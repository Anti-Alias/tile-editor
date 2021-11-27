#ifdef M_DO_NOT_SET_ME
// Note: This is an 'ubershader' that must be preprocessed with 'gpp'.
// All macro variable names should be uppercase with words separated by '_'.
// All macro variable names should be prefixed with 'M_'. IE: 'M_MY_VARIABLE_NAME'.
// Macro flag variable names should be suffixed with '_ENABLED'. IE: 'M_UNICYCLES_ENABLED'.
#endif


// ------------- Vertex input types -------------
struct PointLightVertexIn {
    [[location(0)]] position: vec3<f32>;
};
struct PointLightInstanceIn {
    [[location(1)]] position: vec3<f32>;
    [[location(2)]] radius: f32;
    [[location(3)]] color: vec3<f32>;
};

// ------------- Vertex output type -------------
struct GBufferVertexOut {
    [[builtin(position)]] position: vec4<f32>;
};

// ------------- Uniform type(s) -------------
[[block]]
struct CameraUni {
    proj_view: mat4x4<f32>;
};

[[block]]
struct Size {
    width: f32;
    height: f32;
};

// ------------- GBuffer bind group -------------
[[group(M_GBUFFER_BIND_GROUP), binding(M_SIZE_BINDING)]]
var<uniform> gbuffer_size: Size;
[[group(M_GBUFFER_BIND_GROUP), binding(M_SAMPLER_BINDING)]]
var samp: sampler;
[[group(M_GBUFFER_BIND_GROUP), binding(M_POSITION_TEXTURE_BINDING)]]
var pos_tex: texture_2d<f32>;
[[group(M_GBUFFER_BIND_GROUP), binding(M_NORMAL_TEXTURE_BINDING)]]
var norm_tex: texture_2d<f32>;
#ifdef M_COLOR_BUFFER_ENABLED
[[group(M_GBUFFER_BIND_GROUP), binding(M_COLOR_TEXTURE_BINDING)]]
var color_tex: texture_2d<f32>;
#endif

// ------------- Camera bind group -------------
[[group(M_CAMERA_BIND_GROUP), binding(M_CAMERA_BINDING)]]
var<uniform> camera: CameraUni;

// ------------- Entrypoint -------------
[[stage(vertex)]]
fn main(
    vertex: PointLightVertexIn,
    instance: PointLightInstanceIn
) -> GBufferVertexOut {
    let model_pos = (vertex.position * instance.radius + instance.position);
    let clip_pos = camera.proj_view * vec4<f32>(model_pos, 1.0);
    return GBufferVertexOut(clip_pos);
}




//////////////////////////////// Fragment ////////////////////////////////

// ------------- Light/light set types -------------
struct PointLight {
    position: vec3<f32>;
    color: vec3<f32>;
};

[[block]]
struct PointLightSet {
    size: u32;
    lights: array<PointLight, 128>;
};

// ------------- Output type -------------
struct ColorTargetOut {
    [[location(0)]] color: vec4<f32>;
};

// ------------- Entrypoint -------------
[[stage(fragment)]]
fn main(in: GBufferVertexOut) -> ColorTargetOut {

    /// Initializes color components
    var output = vec4<f32>(0.0);
    let uv = vec2<f32>(
        in.position.x/gbuffer_size.width,
        1.0 - in.position.y/gbuffer_size.height
    );

#   ifdef M_COLOR_BUFFER_ENABLED
    /// Samples color texture and modifies color components
    let color = textureSample(color_tex, samp, uv);
    let ambient = unpack4x8unorm(bitcast<u32>(color.r));    // Unholy bit casting...
    let diffuse = unpack4x8unorm(bitcast<u32>(color.g));    // Unholy bit casting...
    let specular = unpack4x8unorm(bitcast<u32>(color.b));   // Unholy bit casting...
    let emissive = unpack4x8unorm(bitcast<u32>(color.a));   // Unholy bit casting...
    output = ambient + diffuse + specular + emissive;
#   endif

    // Done
    return ColorTargetOut(output);
}