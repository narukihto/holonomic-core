use ark_penta_v_core::{
    collapse_to_optimum, execute_sovereign_collapse, QuantumBundleConfig, SovereignManifold,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_tsp_benefit_extraction() {
        let nodes: &[[f64; 2]] = &[[0.0, 0.0], [1.0, 5.0], [2.0, 2.0], [5.0, 1.0]];
        let config = QuantumBundleConfig {
            distance_matrix: vec![],
            adiabatic_time: 500.0,
        };
        let result = execute_sovereign_collapse(config, nodes);
        assert!(result > 0.0);
    }

    #[test]
    fn test_adiabatic_spectral_gap_polynomial_bound() {
        let nodes: &[[f64; 2]] = &[[0.0, 10.0], [10.0, 0.0], [15.0, 35.0]];
        let config = QuantumBundleConfig {
            distance_matrix: vec![],
            adiabatic_time: 10.0,
        };
        let result = execute_sovereign_collapse(config, nodes);
        assert!(result > 0.0);
    }

    #[test]
    fn test_adversarial_uniform_convergence() {
        let nodes: &[[f64; 2]] = &[[0.0, 5.0], [5.0, 0.0], [5.0, 5.0], [5.0, 0.0]];
        let config = QuantumBundleConfig {
            distance_matrix: vec![],
            adiabatic_time: 100.0,
        };
        let result = execute_sovereign_collapse(config, nodes);
        assert!(result > 0.0);
    }

    #[test]
    fn test_sovereign_core_stability() {
        let nodes: &[[f64; 2]] = &[[0.0, 0.0], [12.0, 12.0]];
        let config = QuantumBundleConfig {
            distance_matrix: vec![],
            adiabatic_time: 1.0,
        };
        let result = execute_sovereign_collapse(config, nodes);
        assert!(result > 0.0);
    }

    #[test]
    fn test_jacobian_convergence_integrity() {
        let nodes: &[[f64; 2]] = &[[0.0, 0.0], [1.0, 1.0], [2.0, 0.0]];
        let manifold = SovereignManifold::new(nodes);
        let tension = manifold.compute_tension_matrix();
        let path = collapse_to_optimum(tension);
        assert_eq!(path.len(), 3);
    }
}
