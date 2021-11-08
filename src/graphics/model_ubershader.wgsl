// ------------------------- Vertex -------------------------
struct ModelVertexIn {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] color: vec4<f32>;
    [[location(3)]] uv: vec2<f32>;
};

struct ModelVertexOut {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] normal: vec3<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct ModelInstanceIn {
    [[location(4)]] col0: vec4<f32>;
    [[location(5)]] col1: vec4<f32>;
    [[location(6)]] col2: vec4<f32>;
    [[location(7)]] col3: vec4<f32>;
};

[[block]]
struct CameraUni {
    proj_view: mat4x4<f32>;
};

// Camera uniform var
[[group(0), binding(0)]]
var<uniform> camera: CameraUni;

// Diffuse uniforms
#ifdef DIFFUSE
[[group(1), binding(T_DIFFUSE_BINDING)]]
var t_diffuse: texture_2d<f32>;
[[group(1), binding(S_DIFFUSE_BINDING)]]
var s_diffuse: sampler;

[[stage(vertex)]]
fn main(vertex: ModelVertexIn, instance: ModelInstanceIn) -> ModelVertexOut {
    let vpos = vec4<f32>(vertex.position, 1.0);
    let model_mat = mat4x4<f32>(
        instance.col0,
        instance.col1,
        instance.col2,
        instance.col3
    );
    let out_pos = camera.proj_view * model_mat * vpos;
    return ModelVertexOut(
       out_pos,
       vertex.normal,
       vertex.color,
       vertex.uv
   );
}




// ------------------------- Fragment -------------------------
struct ColorTargetOut {
    [[location(0)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn main(in: ModelVertexOut) -> ColorTargetOut {
    return ColorTargetOut(in.color);
}