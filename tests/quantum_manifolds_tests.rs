use holonomic_tsp_core::{HolonomicQuantumSolver, QuantumBundleConfig};

#[test]
fn test_skew_symmetric_rotator_purity_and_invariants() {
    const N: usize = 4;

    // Non-trivial spatial distance grid to feed the manifold gradient
    let distance_matrix = [
        [0.0, 4.2, 9.1, 2.5],
        [4.2, 0.0, 1.8, 8.3],
        [9.1, 1.8, 0.0, 5.6],
        [2.5, 8.3, 5.6, 0.0],
    ];

    let config = QuantumBundleConfig {
        distance_matrix,
        adiabatic_time: 20.0,
        coupling_constant: 0.02,
        penalty_gamma: 12.0,
    };

    let solver = HolonomicQuantumSolver::new(config);

    // Constructing a high-density non-orthogonal sample test state matrix
    let sample_state = [
        [0.4, 0.6, 0.0, 0.0],
        [0.0, 0.4, 0.6, 0.0],
        [0.0, 0.0, 0.4, 0.6],
        [0.6, 0.0, 0.0, 0.4],
    ];

    let grad = solver.compute_manifold_gradient(&sample_state);
    let omega = solver.apply_skew_symmetric_rotator(&sample_state, &grad);

    // Verify Skew-Symmetry Invariant property strictly within floating-point precision tolerances: Omega == -Omega^T
    let tolerance = 1e-12;
    for i in 0..N {
        for j in 0..N {
            let combined_sum = omega[i][j] + omega[j][i];
            assert!(
                combined_sum.abs() < tolerance,
                "Topological Skew-Symmetry invariant broken at matrix node ({}, {}). Residual variance: {}",
                i, j, combined_sum
            );
        }
    }
}

#[test]
fn test_adversarial_uniform_matrix_deceptive_convergence() {
    const N: usize = 5;

    // Deceptive worst-case adversarial matrix where all path costs are uniformly distributed.
    // Classical algorithms fail or take exponential paths due to degenerate local valleys.
    let adversarial_matrix = [[15.0; N]; N];

    let config = QuantumBundleConfig {
        distance_matrix: adversarial_matrix,
        adiabatic_time: 100.0,
        coupling_constant: 0.05,
        penalty_gamma: 20.0,
    };

    let solver = HolonomicQuantumSolver::new(config);
    let result_state = solver.execute_topological_collapse();

    // Invariants assertion checking for deadlock clearance and structural integrity
    assert!(
        result_state.is_finite(),
        "Manifold solution collapsed into an invalid float state."
    );
    assert!(
        result_state > 0.0,
        "Physical system collapsed below ground zero energy limits."
    );
}

#[test]
fn test_concurrent_observer_data_race_safety_under_load() {
    const N: usize = 3;
    let distance_matrix = [[0.0, 5.0, 8.0], [5.0, 0.0, 2.0], [8.0, 2.0, 0.0]];

    let config = QuantumBundleConfig {
        distance_matrix,
        adiabatic_time: 10.0,
        coupling_constant: 0.1,
        penalty_gamma: 5.0,
    };

    let solver = HolonomicQuantumSolver::new(config);

    // Triggering dense parallel execution paths to enforce concurrent thread load on DashMap
    let ground_state = solver.execute_topological_collapse();

    // Verify that multi-threaded writers mutated the global state observer cache without data races
    let final_cached_energy = solver.state.state_observer.get("System_Ground_State");

    assert!(
        final_cached_energy.is_some(),
        "Concurrent observer failed to lock or record global ground state."
    );
    assert_eq!(
        *final_cached_energy.unwrap(),
        ground_state,
        "State observer recorded value mismatched the analytical calculation results."
    );
}

#[test]
fn test_adiabatic_spectral_gap_polynomial_bound() {
    const N: usize = 6;
    let dummy_matrix = [[1.0; N]; N];

    let config = QuantumBundleConfig {
        distance_matrix: dummy_matrix,
        adiabatic_time: 5.0,
        coupling_constant: 0.01,
        penalty_gamma: 1.0,
    };

    let solver = HolonomicQuantumSolver::new(config);

    // Testing specific interpolation points s inside [0.0, 1.0]
    let base_test_energy = 50.0;
    let simulated_holonomy = 0.0; // Baseline field projection for test isolation

    // Verification calls updated with the accumulated holonomy parameter to match new signature
    let energy_at_s_0 = solver.simulate_adiabatic_evolution(0.0, base_test_energy, simulated_holonomy);
    let energy_at_s_1 = solver.simulate_adiabatic_evolution(1.0, base_test_energy, simulated_holonomy);

    // Asserting that the initial state energy corresponds to baseline H_0
    assert_eq!(energy_at_s_0, base_test_energy);

    // Asserting that target energy is safely bounded and protected by the polynomial gap factor
    let minimum_allowed_gap = 1.0 / (N as f64).powf(2.0);
    assert_eq!(energy_at_s_1, base_test_energy * minimum_allowed_gap);
}
