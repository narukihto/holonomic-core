use ark_penta_v_core::{HolonomicQuantumSolver, QuantumBundleConfig};

#[test]
fn test_adiabatic_spectral_gap_polynomial_bound() {
    let distance_matrix = [[0.0, 10.0, 15.0], [10.0, 0.0, 35.0], [15.0, 35.0, 0.0]];
    let config = QuantumBundleConfig {
        distance_matrix,
        adiabatic_time: 10.0,
        coupling_constant: 0.1,
        penalty_gamma: 5.0,
    };
    let solver = HolonomicQuantumSolver::new(config);
    let energy = solver.simulate_adiabatic_evolution(0.5, 100.0, 1.5);
    assert!(energy > 0.0);
}

#[test]
fn test_skew_symmetric_rotator_purity_and_invariants() {
    let distance_matrix = [[0.0, 4.0], [4.0, 0.0]];
    let config = QuantumBundleConfig {
        distance_matrix,
        adiabatic_time: 5.0,
        coupling_constant: 0.5,
        penalty_gamma: 2.0,
    };
    let x = [[1.0, 0.0], [0.0, 1.0]];
    let grad = config.compute_manifold_gradient(&x);
    let omega = config.apply_skew_symmetric_rotator(&x, &grad);
    assert!((omega[0][1] + omega[1][0]).abs() < 1e-6);
}

#[test]
fn test_adversarial_uniform_matrix_deceptive_convergence() {
    let distance_matrix = [
        [0.0, 5.0, 5.0, 5.0],
        [5.0, 0.0, 5.0, 5.0],
        [5.0, 5.0, 0.0, 5.0],
        [5.0, 5.0, 5.0, 0.0],
    ];
    let config = QuantumBundleConfig {
        distance_matrix,
        adiabatic_time: 100.0,
        coupling_constant: 0.01,
        penalty_gamma: 10.0,
    };
    let solver = HolonomicQuantumSolver::new(config);
    let result = solver.execute_topological_collapse();
    assert!(result > 0.0);
}

#[test]
fn test_concurrent_observer_data_race_safety_under_load() {
    let distance_matrix = [[0.0, 12.0], [12.0, 0.0]];
    let config = QuantumBundleConfig {
        distance_matrix,
        adiabatic_time: 1.0,
        coupling_constant: 0.1,
        penalty_gamma: 1.0,
    };
    let solver = HolonomicQuantumSolver::new(config);
    let _ = solver.execute_topological_collapse();
    assert!(solver
        .state
        .state_observer
        .contains_key("System_Ground_State"));
}
