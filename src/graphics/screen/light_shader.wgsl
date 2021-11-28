var<private> coords: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(1.0, -1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(-1.0, 1.0),
    vec2<f32>(-1.0, -1.0)
);

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
};

[[stage(vertex)]]
fn main(
    [[builtin(vertex_index)]] index: u32
) -> VertexOutput {
    let pos = vec4<f32>(coords[index], 0.0, 1.0);
    return VertexOutput(pos);
}

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}