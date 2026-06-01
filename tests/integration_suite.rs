use ark_penta_v_core::{
    collapse_to_optimum, execute_sovereign_collapse, QuantumBundleConfig, SovereignManifold,
};

#[cfg(test)]
mod integration_suite {
    use super::*;

    #[test]
    fn test_full_manifold_lifecycle() {
        let nodes: &[[f64; 2]] = &[[0.0, 0.0], [1.0, 2.0], [2.0, 1.0], [3.0, 3.0]];
        let config = QuantumBundleConfig {
            distance_matrix: vec![],
            adiabatic_time: 250.0,
        };

        let manifold = SovereignManifold::new(nodes);
        let tension = manifold.compute_tension_matrix();

        let path = collapse_to_optimum(tension);
        let result = execute_sovereign_collapse(config, nodes);

        assert_eq!(path.len(), 4);
        assert!(result >= 0.0);
    }

    #[test]
    fn test_jacobian_boundary_consistency() {
        let nodes: &[[f64; 2]] = &[[0.0, 0.0], [100.0, 100.0]];
        let manifold = SovereignManifold::new(nodes);
        let tension = manifold.compute_tension_matrix();

        let path = collapse_to_optimum(tension);
        assert_eq!(path.len(), 2);
    }
}
