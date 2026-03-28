struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec3<f32>,
    @location(1) world_position: vec3<f32>,
};

struct Uniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec3<f32>,
    gain: f32,
    clip: f32,
    opacity: f32,
};

@group(0) @binding(0)
var t_volume: texture_3d<f32>;
@group(0) @binding(1)
var s_volume: sampler;
@group(0) @binding(2)
var t_colormap: texture_1d<f32>;
@group(0) @binding(3)
var s_colormap: sampler;

@group(1) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.world_position = model.position;
    out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let ray_dir = normalize(in.world_position - uniforms.camera_pos);
    var ray_pos = in.tex_coords;
    
    // Transform ray_dir to texture space if world space != texture space
    // For now assume proxy cube matches texture space [0, 1]
    
    let step_size = 0.005;
    let max_steps = 400;
    
    var accumulated_color = vec4<f32>(0.0);
    
    // Simple ray marching
    for (var i = 0; i < max_steps; i++) {
        // Bounds check in texture space
        if (ray_pos.x < 0.0 || ray_pos.x > 1.0 || 
            ray_pos.y < 0.0 || ray_pos.y > 1.0 || 
            ray_pos.z < 0.0 || ray_pos.z > 1.0) {
            break;
        }
        
        let val = textureSampleLevel(t_volume, s_volume, ray_pos, 0.0).r;
        
        // Transfer function
        if (abs(val) > uniforms.clip) {
            let normalized_val = (val * uniforms.gain + 1.0) * 0.5;
            let color = textureSampleLevel(t_colormap, s_colormap, clamp(normalized_val, 0.0, 1.0), 0.0);
            
            let alpha = color.a * uniforms.opacity * step_size * 50.0;

            // WGSL does not support swizzle assignments, assign each component individually
            let contribution = (1.0 - accumulated_color.a) * color.rgb * alpha;
            accumulated_color = vec4<f32>(
                accumulated_color.r + contribution.r,
                accumulated_color.g + contribution.g,
                accumulated_color.b + contribution.b,
                accumulated_color.a + (1.0 - accumulated_color.a) * alpha
            );
        }
        
        if (accumulated_color.a >= 0.95) {
            break;
        }
        
        ray_pos += ray_dir * step_size;
    }
    
    return accumulated_color;
}
