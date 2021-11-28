#ifdef M_DO_NOT_SET_ME
// Note: This is an 'ubershader' that must be preprocessed with 'gpp'.
// All macro variable names should be uppercase with words separated by '_'.
// All macro variable names should be prefixed with 'M_'. IE: 'M_MY_VARIABLE_NAME'.
// Macro flag variable names should be suffixed with '_ENABLED'. IE: 'M_UNICYCLES_ENABLED'.
#endif


// This shader renders point lights specifically.
// Ambient/directional lights are handled by a separate shader entirely.



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
    [[location(0)]] light_position: vec3<f32>;
    [[location(1)]] light_color: vec3<f32>;
};

// ------------- Uniform type(s) -------------
[[block]]
struct CameraUni {
    proj_view: mat4x4<f32>;
};

[[block]]
struct LightAttenuation {
    constant: f32;
    linear: f32;
    quadratic: f32;
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

// ------------- Light attenuation bind group -------------
[[group(M_LIGHT_ATT_BIND_GROUP), binding(M_LIGHT_ATT_BINDING)]]
var<uniform> light_attenuation: LightAttenuation;

// ------------- Entrypoint -------------
[[stage(vertex)]]
fn main(
    vertex: PointLightVertexIn,
    light: PointLightInstanceIn
) -> GBufferVertexOut {
    let model_pos = (vertex.position * light.radius + light.position);
    let clip_pos = camera.proj_view * vec4<f32>(model_pos, 1.0);
    return GBufferVertexOut(clip_pos, light.position, light.color);
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


fn compute_lighting(vert: GBufferVertexOut) -> vec4<f32> {

    // Converts framebuffer coordinates to UV coordiantes

    // Unpacks components from color
    let x: u32 = u32(vert.position.x);
    let y: u32 = u32(vert.position.y);
    let xy = vec2<i32>(i32(vert.position.x), i32(vert.position.y));
    let color = textureLoad(color_tex, xy, 0);
    let ambient = unpack4x8unorm(bitcast<u32>(color.r));    // Unholy bit casting...
    let diffuse = unpack4x8unorm(bitcast<u32>(color.g));    // Unholy bit casting...
    let specular = unpack4x8unorm(bitcast<u32>(color.b));   // Unholy bit casting...
    let emissive = unpack4x8unorm(bitcast<u32>(color.a));   // Unholy bit casting...

    // Samples geom data
    let fwp = textureLoad(pos_tex, xy, 0);
    let nv = textureLoad(norm_tex, xy, 0);

    // Computes lambertian part
    let frag_world_pos = vec3<f32>(fwp.x, fwp.y, fwp.z);        // Position of fragment
    let frag_to_light = vert.light_position - frag_world_pos;   // Vec from frag to light (not normalized)
    let light_vec = normalize(frag_to_light);                   // Vec from frag to light origin (normalized)
    let norm_vec = normalize(vec3<f32>(nv.x, nv.y, nv.z));      // Normal of fragment (normalized)
    let costheta = max(0.0, dot(norm_vec, light_vec));

    // Computes light attenuation part
    let kc = light_attenuation.constant;
    let kl = light_attenuation.linear;
    let kq = light_attenuation.quadratic;
    let d = length(frag_to_light);
    let att = 1.0 / (kc + kl*d + kq*d*d);
    let light_color = vec4<f32>(vert.light_color, 1.0);

    // Done
    return diffuse * light_color * costheta * att;
}

// ------------- Entrypoint -------------
[[stage(fragment)]]
fn main(vert: GBufferVertexOut) -> ColorTargetOut {

    // Initializes color components
    var output = vec4<f32>(0.0);

    // Samples color texture and modifies color components
    output = compute_lighting(vert);

    // Done
    return ColorTargetOut(output);
    //return ColorTargetOut(vec4<f32>(1.0, 0.0, 0.0, 1.0));
}