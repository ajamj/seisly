// Fault surface shader with transparency support and 3D transformation

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct FaultUniforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> fault_uniforms: FaultUniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.world_position = model.position;
    output.normal = model.normal;
    
    // Apply MVP transformation
    let mvp = fault_uniforms.projection * fault_uniforms.view * fault_uniforms.model;
    output.clip_position = mvp * vec4<f32>(model.position, 1.0);
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple lighting calculation
    let light_dir = normalize(vec3<f32>(0.5, 0.8, 1.0));
    let normal = normalize(in.normal);
    let diff = max(dot(normal, light_dir), 0.3); // Ambient + diffuse
    
    // Apply lighting to RGB, keep alpha from uniform
    let lit_color = fault_uniforms.color.rgb * diff;
    
    return vec4<f32>(lit_color, fault_uniforms.color.a);
}
