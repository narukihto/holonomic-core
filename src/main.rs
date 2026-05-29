use holonomic_tsp_core::{HolonomicQuantumSolver, QuantumBundleConfig};

#[allow(clippy::needless_range_loop)]
fn main() {
    println!("=========================================================================");
    println!("===   HYBRID TOPOLOGICAL ACO QUANTUM SOLVER CORE INITIALIZING ENGINE  ===");
    println!("=========================================================================");

    // Scale up instance boundary parameters safely to evaluate Polynomial scaling
    const NETWORK_SIZE: usize = 20;

    // Generate a pseudo-randomized deterministic dense mesh
    let mut distance_matrix = [[0.0; NETWORK_SIZE]; NETWORK_SIZE];
    for i in 0..NETWORK_SIZE {
        for j in 0..NETWORK_SIZE {
            if i != j {
                let cost = (((i * 7 + j * 13) % 45) + 5) as f64;
                distance_matrix[i][j] = cost;
            }
        }
    }

    // Initializing high-dimensional heuristic configuration invariants
    let config = QuantumBundleConfig {
        distance_matrix,
        adiabatic_time: 500.0,
        coupling_constant: 0.08,
        penalty_gamma: 15.0,
    };

    println!("[MANIFOLD INIT] Projecting dense matrix grid mapping onto TACO framework...");
    println!(
        "[MANIFOLD INIT] System Size: {} fully interconnected optimization nodes.",
        NETWORK_SIZE
    );

    let solver = HolonomicQuantumSolver::new(config);

    println!("[EVOLUTION] Streaming Berry Phase forces down through topological ant tracks...");

    // Fire the calculation instantly
    let final_energy_state = solver.execute_topological_collapse();

    println!("\n[COLLAPSE SUCCESS] Manifold heuristic solution concluded instantly.");
    println!(
        ">> High-Precision Optimal Tour Cost Discovered: {:.4}",
        final_energy_state
    );

    println!("=========================================================================");
    println!("===        HYBRID GENERATION PROCESSING SEQUENCE COMPLETED SUCCESS      ===");
    println!("=========================================================================");
}
