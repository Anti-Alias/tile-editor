#ifdef M_DO_NOT_SET_ME
// Note: This is an 'ubershader' that must be preprocessed with 'gpp'.
// All macro variable names should be uppercase with words separated by '_'.
// All macro variable names should be prefixed with 'M_'. IE: 'M_MY_VARIABLE_NAME'.
// Macro flag variable names should be suffixed with '_ENABLED'. IE: 'M_UNICYCLES_ENABLED'.
#endif



//////////////////////////////// Vertex ////////////////////////////////
// ------------- Vertex input type -------------
struct ModelVertexIn {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] tangent: vec3<f32>;
    [[location(3)]] bitangent: vec3<f32>;
    [[location(4)]] color: vec4<f32>;
    [[location(5)]] uv: vec2<f32>;
};


// ------------- Instance input type -------------
struct ModelInstanceIn {
    // Model matrix columns
    [[location(6)]] m_col0: vec4<f32>;
    [[location(7)]] m_col1: vec4<f32>;
    [[location(8)]] m_col2: vec4<f32>;
    [[location(9)]] m_col3: vec4<f32>;

    // Normal matrix columns
    [[location(10)]] n_col0: vec3<f32>;
    [[location(11)]] n_col1: vec3<f32>;
    [[location(12)]] n_col2: vec3<f32>;
};


// ------------- Vertex output type -------------
struct ModelVertexOut {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] model_position: vec4<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] color: vec4<f32>;
    [[location(3)]] uv: vec2<f32>;
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


// ------------- Texture bind group -------------
#ifdef M_NORMAL_MATERIAL_ENABLED
[[group(M_MATERIAL_BIND_GROUP), binding(M_NORMAL_TEXTURE_BINDING)]]
var norm_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_NORMAL_SAMPLER_BINDING)]]
var norm_samp: sampler;
#endif

#ifdef M_AMBIENT_MATERIAL_ENABLED
[[group(M_MATERIAL_BIND_GROUP), binding(M_AMBIENT_TEXTURE_BINDING)]]
var amb_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_AMBIENT_SAMPLER_BINDING)]]
var amb_samp: sampler;
#endif

#ifdef M_DIFFUSE_MATERIAL_ENABLED
[[group(M_MATERIAL_BIND_GROUP), binding(M_DIFFUSE_TEXTURE_BINDING)]]
var diff_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_DIFFUSE_SAMPLER_BINDING)]]
var diff_samp: sampler;
#endif

#ifdef M_SPECULAR_GLOSS_ENABLED
[[group(M_MATERIAL_BIND_GROUP), binding(M_SPECULAR_TEXTURE_BINDING)]]
var spec_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_SPECULAR_SAMPLER_BINDING)]]
var spec_samp: sampler;
[[group(M_MATERIAL_BIND_GROUP), binding(M_GLOSS_TEXTURE_BINDING)]]
var gloss_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_GLOSS_SAMPLER_BINDING)]]
var gloss_samp: sampler;
#endif

#ifdef M_EMISSIVE_MATERIAL_ENABLED
[[group(M_MATERIAL_BIND_GROUP), binding(M_EMISSIVE_TEXTURE_BINDING)]]
var emi_tex: texture_2d<f32>;
[[group(M_MATERIAL_BIND_GROUP), binding(M_EMISSIVE_SAMPLER_BINDING)]]
var emi_samp: sampler;
#endif


// ------------- Entrypoint -------------
[[stage(vertex)]]
fn main(vertex: ModelVertexIn, instance: ModelInstanceIn) -> ModelVertexOut {
    let model_mat = mat4x4<f32>(
        instance.m_col0,
        instance.m_col1,
        instance.m_col2,
        instance.m_col3
    );
    let norm_mat = mat3x3<f32>(
        instance.n_col0,
        instance.n_col1,
        instance.n_col2,
    );
    let model_pos = model_mat * vec4<f32>(vertex.position, 1.0);
    let position = camera.proj_view * model_pos;
    let norm = norm_mat * vertex.normal;
    return ModelVertexOut(
       position,
       model_pos,
       norm,
       vertex.color,
       vertex.uv
   );
}




//////////////////////////////// Fragment ////////////////////////////////
// ------------- Output type -------------
struct ColorTargetOut {
    [[location(M_POSITION_BUFFER_LOCATION)]] position: vec4<f32>;
    [[location(M_NORMAL_BUFFER_LOCATION)]] normal: vec4<f32>;
    [[location(M_COLOR_BUFFER_LOCATION)]] color: vec4<f32>;
};

fn sample_ambient(in: ModelVertexOut) -> f32 {
#   ifdef M_AMBIENT_MATERIAL_ENABLED
    let ambient = textureSample(amb_tex, amb_samp, in.uv);
    return bitcast<f32>(pack4x8unorm(ambient));  // Unholy bit casting...
#   else
    return 0.0;
#endif
}

fn sample_diffuse(in: ModelVertexOut) -> f32 {
#   ifdef M_DIFFUSE_MATERIAL_ENABLED
    let diffuse = in.color * textureSample(diff_tex, diff_samp, in.uv);
    return bitcast<f32>(pack4x8unorm(diffuse));  // Unholy bit casting...
#   else
    return 0.0;
#   endif
}

fn sample_specular_gloss(in: ModelVertexOut) -> f32 {
#   ifdef M_SPECULAR_GLOSS_ENABLED

    // Samples specular and gloss, then trims them
    var specColor = textureSample(spec_tex, spec_samp, in.uv);
    let glossColor = textureSample(gloss_tex, gloss_samp, in.uv);
    let glossGray = (glossColor.r + glossColor.g + glossColor.b)/3.0;

    // Packs specular and gloss into ints. Mind the endianness :)
    let specPacked = pack4x8unorm(specColor) & 0x00FFFFFFu;             // uint (spec_r, spec_g, spec_b, 0    )
    let glossPacked = (u32(glossGray * 255.0) << 24u) & 0xFF000000u;    // uint (0,      0,      0,      gloss)
    return bitcast<f32>(specPacked + glossPacked);                      // uint (spec_r, spec_g, spec_b, gloss)
#   else
    return bitcast<f32>(0x01000000);
#   endif
}

fn sample_emissive(in: ModelVertexOut) -> f32 {
#   ifdef M_EMISSIVE_MATERIAL_ENABLED
    let emissive = textureSample(emi_tex, emi_samp, in.uv);
    return bitcast<f32>(pack4x8unorm(emissive));
#   else
    return 0.0;
#   endif
}

fn sample_normal(in: ModelVertexOut) -> f32 {
#   ifdef M_NORMAL_MATERIAL_ENABLED
    let norm = textureSample(norm_tex, norm_samp, in.uv);
    return bitcast<f32>(pack4x8unorm(norm));
#   else
    return 0.0;
#   endif
}



// ------------- Entrypoint -------------
[[stage(fragment)]]
fn main(in: ModelVertexOut) -> ColorTargetOut {

    // Variables to write out to color targets
    let position = in.model_position;       // X, Y, Z, <unused>
    let normal = vec4<f32>(in.normal, 1.0); // X, Y, Z, <unused>
    var color = vec4<f32>(0.0);             // ambient(rgba), diffuse(rgba), specular(red, green, blue, gloss), emissive(rgba)

    // Sample from textures and write encoded value to color
    color.r = sample_ambient(in);
    color.g = sample_diffuse(in);
    color.b = sample_specular_gloss(in);
    color.a = sample_emissive(in);

    // Outputs variables to color targets
    return ColorTargetOut(
        position,
        normal,
        color
    );
}