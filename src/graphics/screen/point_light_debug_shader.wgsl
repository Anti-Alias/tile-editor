#ifdef M_DO_NOT_SET_ME
// Note: This is an shader that must be preprocessed with 'gpp'.
// All macro variable names should be uppercase with words separated by '_'.
// All macro variable names should be prefixed with 'M_'. IE: 'M_MY_VARIABLE_NAME'.
// Macro flag variable names should be suffixed with '_ENABLED'. IE: 'M_UNICYCLES_ENABLED'.
#endif

// ------------- Vertex input and output types -------------
struct PointLightVertex {
    [[location(0)]] position: vec3<f32>;
};
struct PointLightInstance {
    [[location(1)]] position: vec3<f32>;
    [[location(3)]] color: vec3<f32>;
};
struct PointLightFragment {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

// ------------- Uniform type(s) -------------
[[block]]
struct CameraUni {
    eye: vec3<f32>;
    proj_view: mat4x4<f32>;
};

// ------------- Camera bind group -------------
[[group(M_CAMERA_BIND_GROUP), binding(M_CAMERA_BINDING)]]
var<uniform> camera: CameraUni;


[[stage(vertex)]]
fn main(vertex: PointLightVertex, instance: PointLightInstance) -> PointLightFragment {
    let vpos = vertex.position * f32(M_LIGHT_RADIUS);
    let clip_pos = camera.proj_view * (vec4<f32>(vpos + instance.position, 1.0));
    let c = instance.color;
    let max_channel = max(max(c.r, max(c.g, c.b)), 0.0001);     // Get max of r,g,b with epsilon to prevent divide-by-zero.
    let color = c / max_channel;
    return PointLightFragment(clip_pos, color);
}

[[stage(fragment)]]
fn main(fragment: PointLightFragment) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(fragment.color, 1.0);
}