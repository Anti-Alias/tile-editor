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
fn main(
    [[builtin(vertex_index)]] vertex_index: u32,
    input: ModelVertex
) -> ModelVertexOutput {
    let color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    let x = f32(1 - i32(vertex_index)) * 0.5;
    let y = f32(i32(vertex_index & 1u) * 2 - 1) * 0.5;
    let out = ModelVertexOutput(
        vec4<f32>(x, y, 0.0, 1.0),
        vec3<f32>(1.0, 1.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec2<f32>(0.0, 0.0)
    );
    return out;
}

[[stage(vertex)]]
fn main_old(input: ModelVertex) -> ModelVertexOutput {
    let position = input.position + vec3<f32>(0.0, 0.0, -10.0);
    let out = ModelVertexOutput(
        vec4<f32>(position, 1.0),
        input.normal,
        input.color,
        input.uv
    );
    return out;
}



// ----------------- Fragment -----------------
struct ColorTargetOutput {
    [[location(0)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn main_old(in: ModelVertexOutput) -> ColorTargetOutput {
    //return ColorTargetOutput(in.color);
    return ColorTargetOutput(vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

[[stage(fragment)]]
fn main(in: ModelVertexOutput) -> ColorTargetOutput {
    //return ColorTargetOutput(in.color);
    return ColorTargetOutput(vec4<f32>(1.0, 1.0, 1.0, 1.0));
}