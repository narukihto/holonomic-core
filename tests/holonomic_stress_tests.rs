use ark_penta_v_core::{execute_sovereign_collapse, init_sovereign_core};

#[test]
fn test_sovereign_collapse_stress_load() {
    init_sovereign_core();

    for i in 0..1000 {
        let nodes: [[f64; 2]; 4] = [
            [0.0, 0.0],
            [1.0, i as f64],
            [5.0, 2.0],
            [3.0, 0.0],
        ];

        let path = execute_sovereign_collapse(&nodes);

        assert!(!path.is_empty());
    }
}
