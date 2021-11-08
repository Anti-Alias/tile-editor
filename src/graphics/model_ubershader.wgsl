// ------------------------- Vertex -------------------------

[[block]]
struct Camera {
    proj_view: mat4x4<f32>;
};

struct ModelVertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] color: vec4<f32>;
    [[location(3)]] uv: vec2<f32>;
};

struct ModelVertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] normal: vec3<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct ModelInstance {
    [[location(4)]] col0: vec4<f32>;
    [[location(5)]] col1: vec4<f32>;
    [[location(6)]] col2: vec4<f32>;
    [[location(7)]] col3: vec4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: Camera;

[[stage(vertex)]]
fn main(vertex: ModelVertex, instance: ModelInstance) -> ModelVertexOutput {
    let vpos = vec4<f32>(vertex.position, 1.0);
    let model_mat = mat4x4<f32>(
        instance.col0,
        instance.col1,
        instance.col2,
        instance.col3
    );
    let out_pos = camera.proj_view * model_mat * vpos;
    return ModelVertexOutput(
       out_pos,
       vertex.normal,
       vertex.color,
       vertex.uv
   );
}




// ------------------------- Fragment -------------------------
struct ColorTargetOutput {
    [[location(0)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn main(in: ModelVertexOutput) -> ColorTargetOutput {
    return ColorTargetOutput(in.color);
}