#ifdef M_DO_NOT_SET_ME
// Note: This is an 'ubershader' that must be preprocessed with 'gpp'.
// All macro variable names should be uppercase with words separated by '_'.
// All macro variable names should be prefixed with 'M_'. IE: 'M_MY_VARIABLE_NAME'.
// Macro flag variable names should be suffixed with '_ENABLED'. IE: 'M_UNICYCLES_ENABLED'.
#endif


// This shader renders point lights (light volumes) to the screen.
// Samples from textures in a GBuffer.


// ------------- Vertex input types -------------
struct PointLightVertexIn {
    [[location(0)]] position: vec3<f32>;
};
struct PointLightInstanceIn {
    [[location(1)]] position: vec3<f32>;
    [[location(2)]] radius: f32;
    [[location(3)]] color: vec3<f32>;
    [[location(4)]] coefficients: vec3<f32>;
};

// ------------- Vertex output type (fragment) -------------
struct GBufferVertexOut {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] light_position: vec3<f32>;
    [[location(1)]] light_color: vec3<f32>;
    [[location(2)]] light_coeff: vec3<f32>;
};

// ------------- Uniform type(s) -------------
[[block]]
struct CameraUni {
    proj_view: mat4x4<f32>;
};

// ------------- GBuffer bind group -------------
[[group(M_GBUFFER_BIND_GROUP), binding(M_POSITION_TEXTURE_BINDING)]]
var pos_tex: texture_2d<f32>;
[[group(M_GBUFFER_BIND_GROUP), binding(M_NORMAL_TEXTURE_BINDING)]]
var norm_tex: texture_2d<f32>;
[[group(M_GBUFFER_BIND_GROUP), binding(M_COLOR_TEXTURE_BINDING)]]
var color_tex: texture_2d<f32>;

// ------------- Camera bind group -------------
[[group(M_CAMERA_BIND_GROUP), binding(M_CAMERA_BINDING)]]
var<uniform> camera: CameraUni;

// ------------- Entrypoint -------------
[[stage(vertex)]]
fn main(
    vertex: PointLightVertexIn,
    light: PointLightInstanceIn
) -> GBufferVertexOut {
    let model_pos = (vertex.position * light.radius + light.position);
    let clip_pos = camera.proj_view * vec4<f32>(model_pos, 1.0);
    return GBufferVertexOut(
        clip_pos,
        light.position,
        light.color,
        light.coefficients
    );
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


fn compute_lighting(frag: GBufferVertexOut) -> vec4<f32> {

    // Converts framebuffer coordinates to UV coordiantes

    // Unpacks components from color
    let xy = vec2<i32>(i32(frag.position.x), i32(frag.position.y));
    let color = textureLoad(color_tex, xy, 0);
    let ambient = unpack4x8unorm(bitcast<u32>(color.r));    // Unholy bit casting...
    let diffuse = unpack4x8unorm(bitcast<u32>(color.g));    // Unholy bit casting...
    let specular = unpack4x8unorm(bitcast<u32>(color.b));   // Unholy bit casting...

    // Computes lambertian part
    let frag_world_pos = textureLoad(pos_tex, xy, 0).xyz;       // Position of fragment
    let norm_vec = normalize(textureLoad(norm_tex, xy, 0).xyz); // Normal of fragment (normalized)
    let frag_to_light = frag.light_position - frag_world_pos;   // Vec from frag to light (not normalized)
    let light_vec = normalize(frag_to_light);                   // Vec from frag to light origin (normalized)
    let costheta = max(0.0, dot(norm_vec, light_vec));          // Computes dot product of light vec with normal vec

    // Computes light attenuation part
    let d = length(frag_to_light);      // Distance of fragment's position to the light's origin
    let c = frag.light_coeff.x;         // Constant
    let l = frag.light_coeff.y;         // Linear
    let q = frag.light_coeff.z;         // Quadratic
    let att = 1.0 / (c + d*(l + q*d));  // Attenuation

    // Done
    let light_color = vec4<f32>(frag.light_color, 1.0);
    return diffuse * light_color * costheta * att;
}

// ------------- Entrypoint -------------
[[stage(fragment)]]
fn main(frag: GBufferVertexOut) -> ColorTargetOut {

    // Initializes color components
    var output = vec4<f32>(0.0);

    // Samples color texture and modifies color components
    output = compute_lighting(frag);

    // Done
    return ColorTargetOut(output);
    //return ColorTargetOut(vec4<f32>(1.0, 0.0, 0.0, 1.0));
}