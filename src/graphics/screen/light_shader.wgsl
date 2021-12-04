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

[[block]]
struct CameraUni {
    eye: vec3<f32>;
    proj_view: mat4x4<f32>;
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

// ------------- Camera bind group -------------
[[group(M_CAMERA_BIND_GROUP), binding(M_CAMERA_BINDING)]]
var<uniform> camera: CameraUni;


[[stage(vertex)]]
fn main(
    [[builtin(vertex_index)]] index: u32
) -> VertexOutput {
    let pos = vec4<f32>(coords[index], 0.0, 1.0);
    return VertexOutput(pos);
}

[[stage(fragment)]]
fn main(frag: VertexOutput) -> [[location(0)]] vec4<f32> {

    // Samples geom
    let xy = vec2<i32>(i32(frag.position.x), i32(frag.position.y));
    let frag_world_pos = textureLoad(pos_tex, xy, 0).xyz;
    let norm_vec = textureLoad(norm_tex, xy, 0).xyz;

    // Samples color and extracts bits
    let color = textureLoad(color_tex, xy, 0);
    let diffuse_bits = bitcast<u32>(color.g);
    let specular_gloss_bits = bitcast<u32>(color.b);
    let emissive_bits = bitcast<u32>(color.a);

    // Unpacks bits
    let diffuse_col = unpack4x8unorm(diffuse_bits).rgb;
    let specular_bits = specular_gloss_bits & 0x00FFFFFFu;
    let gloss_bits = (specular_gloss_bits & 0xFF000000u) >> 24u;
    let specular_col = unpack4x8unorm(specular_bits).rgb;
    let gloss = f32(gloss_bits);
    let emissive = unpack4x8unorm(emissive_bits).rgb;

    // Adds directional lights with specular component
    var light_sum = vec3<f32>(0.0);
    var spec_sum = vec3<f32>(0.0);
    let frag_world_pos = textureLoad(pos_tex, xy, 0).xyz;       // Position of fragment
    let view_vec = normalize(camera.eye - frag_world_pos);
    for(var i: i32=0; i<directional_light_set.length; i=i+1) {

        // Computes lambertial part
        let light = directional_light_set.lights[i];
        let light_vec = normalize(-light.direction);                // Vec from frag to light origin (normalized)
        let costheta = max(0.0, dot(norm_vec, light_vec));          // Computes dot product of light vec with normal vec
        light_sum = light_sum + light.color*costheta;

        // Computes specular part
        let h = normalize(light_vec + view_vec);
        let dot = dot(h, norm_vec);
        let spec = pow(max(dot, 0.0), gloss*4.0);

        // Adds to sum
        spec_sum = spec_sum + light.color * specular_col * spec;
    }

    // Accumulates ambient lights
    for(var i: i32=0; i<ambient_light_set.length; i=i+1) {
        let light = ambient_light_set.lights[i];
        light_sum = light_sum + light.color;
    }

    // Returns combination of lights with emission
    return vec4<f32>(diffuse_col*light_sum + spec_sum + emissive, 1.0);
}