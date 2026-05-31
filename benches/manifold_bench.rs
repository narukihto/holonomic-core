use ark_penta_v_core::{execute_sovereign_collapse, QuantumBundleConfig};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_sovereign_collapse(c: &mut Criterion) {
    let nodes: Vec<[f64; 2]> = vec![[0.0, 0.0], [1.0, 5.0], [2.0, 2.0], [5.0, 1.0], [10.0, 10.0]];
    let config = QuantumBundleConfig {
        distance_matrix: vec![],
        adiabatic_time: 100.0,
    };

    c.bench_function("sovereign_collapse_5_nodes", |b| {
        b.iter(|| execute_sovereign_collapse(black_box(&config), black_box(&nodes)))
    });
}

criterion_group!(benches, bench_sovereign_collapse);
criterion_main!(benches);
