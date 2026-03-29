use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use sf_attributes::amplitude::RmsAmplitude;
use sf_attributes::trait_def::SeismicAttribute;
use sf_attributes_gpu::GpuAttributeComputer;
use tokio::runtime::Runtime;

fn generate_test_data(size: usize) -> Vec<f32> {
    (0..size).map(|i| (i as f32 * 0.1).sin() * 100.0).collect()
}

fn benchmark_cpu_rms(c: &mut Criterion) {
    let attr = RmsAmplitude;
    let mut group = c.benchmark_group("CPU RMS");
    
    for size in [1000, 10000, 100000].iter() {
        let trace = generate_test_data(*size);
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &trace,
            |b, trace| {
                b.iter(|| attr.compute(black_box(trace), black_box(11)));
            },
        );
    }
    group.finish();
}

fn benchmark_gpu_rms(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let computer = match rt.block_on(GpuAttributeComputer::new()) {
        Ok(c) => c,
        Err(_) => return, // Skip if no GPU available
    };
    
    let mut group = c.benchmark_group("GPU RMS");
    
    for size in [1000, 10000, 100000].iter() {
        let trace = generate_test_data(*size);
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &trace,
            |b, trace| {
                b.iter(|| {
                    rt.block_on(computer.compute_rms_gpu(black_box(trace), black_box(11)))
                        .unwrap()
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, benchmark_cpu_rms, benchmark_gpu_rms);
criterion_main!(benches);
