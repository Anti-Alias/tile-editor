#ifdef M_DO_NOT_SET_ME
// Note: This is an 'ubershader' that must be preprocessed with 'gpp'.
// All macro variable names should be uppercase with words separated by '_'.
// All macro variable names should be prefixed with 'M_'. IE: 'M_MY_VARIABLE_NAME'.
// Macro flag variable names should be suffixed with '_ENABLED'. IE: 'M_UNICYCLES_ENABLED'.
#endif

// ------------- Vertex output type -------------
struct GBufferVertexOut {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
};

// ------------- GBuffer texture bind group -------------
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


// ------------- Entrypoint -------------
[[stage(vertex)]]
fn main(
    [[builtin(vertex_index)]] vertex_index: u32
) -> GBufferVertexOut {
    var xy: vec2<f32>;
    var uv: vec2<f32>;
    switch(vertex_index) {
        case 0: {
            xy = vec2<f32>(-1.0, -1.0);
            uv = vec2<f32>(0.0, 0.0);
            break;
        }
        case 1: {
            xy = vec2<f32>(1.0, -1.0);
            uv = vec2<f32>(1.0, 0.0);
            break;
        }
        case 2: {
            xy = vec2<f32>(1.0, 1.0);
            uv = vec2<f32>(1.0, 1.0);
            break;
        }
        case 3: {
            xy = vec2<f32>(1.0, 1.0);
            uv = vec2<f32>(1.0, 1.0);
            break;
        }
        case 4: {
            xy = vec2<f32>(-1.0, 1.0);
            uv = vec2<f32>(0.0, 1.0);
            break;
        }
        case 5: {
            xy = vec2<f32>(-1.0, -1.0);
            uv = vec2<f32>(0.0, 0.0);
            break;
        }
    }
    return GBufferVertexOut(
        vec4<f32>(xy, 0.0, 1.0),
        uv
    );
}




//////////////////////////////// Fragment ////////////////////////////////

// ------------- Light/light set types -------------
struct PointLight {
    position: vec3<f32>;
    color: vec3<f32>;
};

struct DirectionalLight {
    direction: vec3<f32>;
    color: vec3<f32>;
};

[[block]]
struct PointLightSet {
    size: u32;
    lights: array<PointLight, 128>;
};

[[block]]
struct DirectionalLightSet {
    size: u32;
    lights: array<DirectionalLight, 128>;
};

// ------------- Light/light set types -------------
[[group(M_LIGHT_BIND_GROUP), binding(M_POINT_LIGHT_BINDING)]]
var<uniform> point_lights: PointLightSet;
[[group(M_LIGHT_BIND_GROUP), binding(M_DIRECTIONAL_LIGHT_BINDING)]]
var<uniform> directional_lights: DirectionalLightSet;


// ------------- Output type -------------
struct ColorTargetOut {
    [[location(0)]] color: vec4<f32>;
};

// ------------- Uniforms type -------------

// ------------- Entrypoint -------------
[[stage(fragment)]]
fn main(in: GBufferVertexOut) -> ColorTargetOut {

    /// Initializes color components
    var output = vec4<f32>(0.0);    // RGBA

#   ifdef M_COLOR_BUFFER_ENABLED
    /// Samples color texture and modifies color components
    let color = textureSample(color_tex, samp, in.uv);
    let diffuse = unpack4x8unorm(bitcast<u32>(color.r));    // Unholy bit casting...
    let specular = unpack4x8unorm(bitcast<u32>(color.g));   // Unholy bit casting...
    let emissive = unpack4x8unorm(bitcast<u32>(color.b));   // Unholy bit casting...
    output = diffuse + specular + emissive;
#   endif

    // Done
    return ColorTargetOut(output);
}