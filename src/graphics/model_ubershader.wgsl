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
    let position = input.position + vec3<f32>(0.0, 0.0, -10.0);
    let color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    let out = ModelVertexOutput(
        vec4<f32>(position, 1.0),
        input.normal,
        color,
        input.uv
    );
    return out;
}

// ----------------- Fragment -----------------
struct ColorTargetOutput {
    [[location(0)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn main(in: ModelVertexOutput) -> ColorTargetOutput {
    return ColorTargetOutput(in.color);
}