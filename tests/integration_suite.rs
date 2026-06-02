use ark_penta_v_core::{collapse_to_optimum, SovereignManifold};

#[cfg(test)]
mod integration_suite {
    use super::*;

    #[test]
    fn test_full_manifold_lifecycle() {
        let nodes: &[[f64; 2]] = &[[0.0, 0.0], [1.0, 2.0], [2.0, 1.0], [3.0, 3.0]];
        let manifold = SovereignManifold::new(nodes);
        let tension = manifold.compute_tension_matrix();
        let path = collapse_to_optimum(tension);
        assert!(path.len() <= 4);
    }

    #[test]
    fn test_jacobian_boundary_consistency() {
        let nodes: &[[f64; 2]] = &[[0.0, 0.0], [100.0, 100.0]];
        let manifold = SovereignManifold::new(nodes);
        let tension = manifold.compute_tension_matrix();
        let path = collapse_to_optimum(tension);
        assert!(path.len() <= 2);
    }
}
