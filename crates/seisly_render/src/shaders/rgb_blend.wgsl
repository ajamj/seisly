// RGB Blending Shader for Multi-Volume Analysis

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@group(0) @binding(0) var t_red: texture_2d<f32>;
@group(0) @binding(1) var s_red: sampler;
@group(0) @binding(2) var t_green: texture_2d<f32>;
@group(0) @binding(3) var s_green: sampler;
@group(0) @binding(4) var t_blue: texture_2d<f32>;
@group(0) @binding(5) var s_blue: sampler;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let r = textureSample(t_red, s_red, in.tex_coords).r;
    let g = textureSample(t_green, s_green, in.tex_coords).r;
    let b = textureSample(t_blue, s_blue, in.tex_coords).r;
    
    return vec4<f32>(r, g, b, 1.0);
}
