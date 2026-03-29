// Seismic Attribute Compute Shaders

struct Uniforms {
    window_size: u32,
};

@group(0) @binding(0)
var<storage, read> input_trace: array<f32>;

@group(0) @binding(1)
var<storage, read_write> output_rms: array<f32>;

@group(0) @binding(2)
var<uniform> uniforms: Uniforms;

@compute @workgroup_size(64)
fn compute_rms(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    let trace_len = arrayLength(&input_trace);
    let window_size = uniforms.window_size;
    
    // Check bounds
    if idx >= trace_len - window_size + 1 {
        return;
    }
    
    // Compute RMS in window
    var sum_squares: f32 = 0.0;
    for (var i: u32 = 0; i < window_size; i = i + 1) {
        let val = input_trace[idx + i];
        sum_squares = sum_squares + val * val;
    }
    
    output_rms[idx] = sqrt(sum_squares / f32(window_size));
}

@compute @workgroup_size(64)
fn compute_mean(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    let trace_len = arrayLength(&input_trace);
    let window_size = uniforms.window_size;
    
    if idx >= trace_len - window_size + 1 {
        return;
    }
    
    var sum: f32 = 0.0;
    for (var i: u32 = 0; i < window_size; i = i + 1) {
        sum = sum + input_trace[idx + i];
    }
    
    output_rms[idx] = sum / f32(window_size);
}

@compute @workgroup_size(64)
fn compute_energy(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    let trace_len = arrayLength(&input_trace);
    let window_size = uniforms.window_size;
    
    if idx >= trace_len - window_size + 1 {
        return;
    }
    
    var sum_squares: f32 = 0.0;
    for (var i: u32 = 0; i < window_size; i = i + 1) {
        let val = input_trace[idx + i];
        sum_squares = sum_squares + val * val;
    }
    
    output_rms[idx] = sum_squares;
}
