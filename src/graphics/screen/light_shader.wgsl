#ifdef M_DO_NOT_SET_ME
// Note: This is an 'ubershader' that must be preprocessed with 'gpp'.
// All macro variable names should be uppercase with words separated by '_'.
// All macro variable names should be prefixed with 'M_'. IE: 'M_MY_VARIABLE_NAME'.
// Macro flag variable names should be suffixed with '_ENABLED'. IE: 'M_UNICYCLES_ENABLED'.
#endif


// This shader renders ambient and directional lights.
// Samples from textures in a GBuffer.

struct AmbientLight {
    color: vec3<f32>;
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
var<private> ambient_lights: array<AmbientLight, 1> = array<AmbientLight, 1>(
    AmbientLight(vec3<f32>(0.05, 0.05, 0.05))
);

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
    let emissive = unpack4x8unorm(bitcast<u32>(color.a));       // Unholy bit casting...

    // Accumulates ambient lights
    var output = vec3<f32>(0.0);
    for(var i: i32=0; i<1; i=i+1) {
        let light = ambient_lights[i];
        output = output + diffuse * light.color;
    }

    // Done
    return vec4<f32>(output, 1.0);
}