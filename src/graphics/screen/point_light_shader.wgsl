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
    eye: vec3<f32>;
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


fn compute_lighting(frag: GBufferVertexOut) -> vec3<f32> {

    // Converts framebuffer coordinates to UV coordiantes

    // Unpacks components from color
    let xy = vec2<i32>(i32(frag.position.x), i32(frag.position.y));
    let color = textureLoad(color_tex, xy, 0);
    let diffuse_bits = bitcast<u32>(color.g);
    let specular_gloss_bits = bitcast<u32>(color.b);
    let specular_bits = specular_gloss_bits & 0x00FFFFFFu;
    let gloss_bits = (specular_gloss_bits & 0xFF000000u) >> 24u;
    let diffuse_col = unpack4x8unorm(diffuse_bits).rgb;
    let specular_col = unpack4x8unorm(specular_bits).rgb;
    let gloss = f32(gloss_bits);

    // Computes lambertian part
    let frag_world_pos = textureLoad(pos_tex, xy, 0).xyz;       // Position of fragment
    let norm_vec = textureLoad(norm_tex, xy, 0).xyz;            // Normal of fragment (normalized)
    let frag_to_light = frag.light_position - frag_world_pos;   // Vec from frag to light (not normalized)
    let d = length(frag_to_light);                              // Distance of fragment's position to the light's origin
    let light_vec = frag_to_light / d;                          // Normalized frag-to-light vector
    let costheta = max(0.0, dot(norm_vec, light_vec));          // Computes dot product of light vec with normal vec
    let c = frag.light_coeff.x;         // Constant
    let l = frag.light_coeff.y;         // Linear
    let q = frag.light_coeff.z;         // Quadratic
    let att = 1.0 / (c + d*(l + q*d));  // Attenuation
    let diffuse = diffuse_col * frag.light_color * costheta * att;

    // Computes specular part
    let frag_to_camera = camera.eye - frag_world_pos;
    let h = normalize(frag_to_camera + frag_to_light);
    let spec = pow(max(dot(h, norm_vec), 0.0), gloss);
    let specular = frag.light_color * specular_col * spec * att;

    // Done
    return  diffuse + specular;
}

// ------------- Entrypoint -------------
[[stage(fragment)]]
fn main(frag: GBufferVertexOut) -> ColorTargetOut {

    // Samples color texture and modifies color components
    let output = compute_lighting(frag);

    // Done
    return ColorTargetOut(vec4<f32>(output, 1.0));
}