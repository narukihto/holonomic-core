use ark_penta_v_core::{collapse_to_optimum, QuantumBundleConfig, SovereignManifold};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env;
use std::time::Duration;

fn bench_sovereign_collapse(c: &mut Criterion) {
    let nodes: Vec<[f64; 2]> = vec![[0.0, 0.0], [1.0, 5.0], [2.0, 2.0], [5.0, 1.0], [10.0, 10.0]];
    let manifold = SovereignManifold::new(&nodes);
    let config = QuantumBundleConfig {
        distance_matrix: vec![],
        adiabatic_time: 100.0,
    };

    c.bench_function("sovereign_collapse_5_nodes", |b| {
        b.iter(|| config.execute_sovereign_collapse(black_box(&manifold)))
    });

    let large_nodes: Vec<[f64; 2]> = (0..10000).map(|i| [i as f64, (i % 100) as f64]).collect();
    let large_manifold = SovereignManifold::new(&large_nodes);
    let tension = large_manifold.compute_tension_matrix();

    let mut group = c.benchmark_group("jacobian");
    if env::var("CI").is_ok() {
        group.sample_size(10);
        group.measurement_time(Duration::from_secs(60));
    }

    group.bench_function("jacobian_collapse_10000_nodes", |b| {
        b.iter(|| collapse_to_optimum(black_box(tension.clone())))
    });
    group.finish();
}

criterion_group!(benches, bench_sovereign_collapse);
criterion_main!(benches);
