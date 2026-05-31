use ark_penta_v_core::{execute_sovereign_collapse, init_sovereign_core, QuantumBundleConfig};

#[test]
fn test_sovereign_collapse_stress_load() {
    init_sovereign_core();

    for i in 0..1000 {
        let nodes: &[[f64; 2]] = &[
            [0.0, 0.0],
            [1.0, i as f64],
            [5.0, 2.0],
            [3.0, 0.0],
        ];

        let config = QuantumBundleConfig {
            distance_matrix: vec![],
            adiabatic_time: 1.0,
        };

        let result = execute_sovereign_collapse(config, nodes);

        // التأكد من أن النتيجة تمثل تقارباً صالحاً للمتجه الطوبولوجي
        assert!(result >= 0.0);
    }
}
