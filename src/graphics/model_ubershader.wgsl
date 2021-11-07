// ----------------- Vertex -----------------
[[block]]
struct Camera {
    proj_view: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: Camera;


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


[[stage(vertex)]]
fn main(input: ModelVertex) -> ModelVertexOutput {
    let out_pos = camera.proj_view * vec4<f32>(input.position, 1.0);
    return ModelVertexOutput(
       out_pos,
       input.normal,
       input.color,
       input.uv
   );
}


// ----------------- Fragment -----------------
struct ColorTargetOutput {
    [[location(0)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn main(in: ModelVertexOutput) -> ColorTargetOutput {
    return ColorTargetOutput(in.color);
}