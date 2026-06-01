use ark_penta_v_core::{
    collapse_to_optimum, execute_sovereign_collapse, init_sovereign_core, QuantumBundleConfig,
    SovereignManifold,
};

#[test]
fn test_sovereign_collapse_stress_load_100k() {
    init_sovereign_core();

    let large_nodes: Vec<[f64; 2]> = (0..100000).map(|i| [i as f64, (i % 250) as f64]).collect();

    let config = QuantumBundleConfig {
        distance_matrix: vec![],
        adiabatic_time: 1.0,
    };

    let start = std::time::Instant::now();
    let result = execute_sovereign_collapse(config, &large_nodes);
    let duration = start.elapsed();

    assert!(result >= 0.0);
    assert!(duration.as_secs() < 5);
}

#[test]
fn test_jacobian_projection_scaling_100k() {
    init_sovereign_core();

    let large_nodes: Vec<[f64; 2]> = (0..100000).map(|i| [i as f64, (i % 500) as f64]).collect();

    let manifold = SovereignManifold::new(&large_nodes);
    let tension = manifold.compute_tension_matrix();

    let start = std::time::Instant::now();
    let optimized_path = collapse_to_optimum(tension);
    let duration = start.elapsed();

    assert_eq!(optimized_path.len(), 100000);
    assert!(duration.as_secs() < 10);
}
