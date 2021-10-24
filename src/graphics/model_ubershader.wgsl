// ----------------- Vertex -----------------
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
    return ModelVertexOutput(
       vec4<f32>(input.position, 1.0),
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