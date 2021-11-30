#ifdef M_DO_NOT_SET_ME
// Note: This is an 'ubershader' that must be preprocessed with 'gpp'.
// All macro variable names should be uppercase with words separated by '_'.
// All macro variable names should be prefixed with 'M_'. IE: 'M_MY_VARIABLE_NAME'.
// Macro flag variable names should be suffixed with '_ENABLED'. IE: 'M_UNICYCLES_ENABLED'.
#endif


// This shader renders ambient and directional lights.
// Samples from textures in a GBuffer.

struct PointLight {
    position: vec3<f32>;
    radius: f32;
    color: vec3<f32>;
    coefficients: vec3<f32>;
};

struct DirectionalLight {
    direction: vec3<f32>;
    color: vec3<f32>;
};

struct AmbientLight {
    color: vec3<f32>;
};

[[block]]
struct PointLightSet {
    length: i32;
    lights: array<PointLight, 64>;
};

[[block]]
struct DirectionalLightSet {
    length: i32;
    lights: array<DirectionalLight, 64>;
};

[[block]]
struct AmbientLightSet {
    length: i32;
    lights: array<AmbientLight, 64>;
};


// ------------- Vertices of screen-sized quad -------------
var<private> coords: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(1.0, -1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(-1.0, 1.0),
    vec2<f32>(-1.0, -1.0)
);


// ------------- Vertex output type (fragment) -------------
struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
};


// ------------- GBuffer bind group -------------
[[group(M_GBUFFER_BIND_GROUP), binding(M_POSITION_TEXTURE_BINDING)]]
var pos_tex: texture_2d<f32>;
[[group(M_GBUFFER_BIND_GROUP), binding(M_NORMAL_TEXTURE_BINDING)]]
var norm_tex: texture_2d<f32>;
[[group(M_GBUFFER_BIND_GROUP), binding(M_COLOR_TEXTURE_BINDING)]]
var color_tex: texture_2d<f32>;


// ------------- Light bind group -------------
[[group(M_LIGHT_BUNDLE_BIND_GROUP), binding(M_POINT_LIGHT_BINDING)]]
var<uniform> point_light_set: PointLightSet;
[[group(M_LIGHT_BUNDLE_BIND_GROUP), binding(M_DIRECTIONAL_LIGHT_BINDING)]]
var<uniform> directional_light_set: DirectionalLightSet;
[[group(M_LIGHT_BUNDLE_BIND_GROUP), binding(M_AMBIENT_LIGHT_BINDING)]]
var<uniform> ambient_light_set: AmbientLightSet;

[[stage(vertex)]]
fn main(
    [[builtin(vertex_index)]] index: u32
) -> VertexOutput {
    let pos = vec4<f32>(coords[index], 0.0, 1.0);
    return VertexOutput(pos);
}

[[stage(fragment)]]
fn main(frag: VertexOutput) -> [[location(0)]] vec4<f32> {

    // Samples from GBuffer
    let xy = vec2<i32>(i32(frag.position.x), i32(frag.position.y));
    let color = textureLoad(color_tex, xy, 0);
    let ambient = unpack4x8unorm(bitcast<u32>(color.r));        // Unholy bit casting...
    let diffuse = unpack4x8unorm(bitcast<u32>(color.g)).rgb;    // Unholy bit casting...
    let specular = unpack4x8unorm(bitcast<u32>(color.b));       // Unholy bit casting...
    let emissive = unpack4x8unorm(bitcast<u32>(color.a)).rgb;   // Unholy bit casting...

    // Accumulates ambient lights
    var light_sum = vec3<f32>(0.0);
    for(var i: i32=0; i<ambient_light_set.length; i=i+1) {
        let light = ambient_light_set.lights[i];
        light_sum = light_sum + light.color;
    }

    // Accumulates directional lights (lambertial)
    let frag_world_pos = textureLoad(pos_tex, xy, 0).xyz;       // Position of fragment
    let norm_vec = normalize(textureLoad(norm_tex, xy, 0).xyz); // Normal of fragment (normalized)
    for(var i: i32=0; i<directional_light_set.length; i=i+1) {
        let light = directional_light_set.lights[i];
        let light_vec = normalize(-light.direction);                // Vec from frag to light origin (normalized)
        let costheta = max(0.0, dot(norm_vec, light_vec));          // Computes dot product of light vec with normal vec
        light_sum = light_sum + light.color * costheta;
    }

    // Adds emissive light
    let output = diffuse * light_sum + emissive;
    return vec4<f32>(output, 1.0);
}