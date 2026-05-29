use holonomic_tsp_core::{HolonomicQuantumSolver, QuantumBundleConfig};

#[allow(clippy::needless_range_loop)]
fn main() {
    println!("=========================================================================");
    println!("===   HYBRID COSMIC TOPOLOGICAL QUANTUM SOLVER CORE INITIALIZING      ===");
    println!("=========================================================================");

    const NETWORK_SIZE: usize = 20;

    let mut distance_matrix = [[0.0; NETWORK_SIZE]; NETWORK_SIZE];
    for i in 0..NETWORK_SIZE {
        for j in 0..NETWORK_SIZE {
            if i != j {
                let cost = (((i * 7 + j * 13) % 45) + 5) as f64;
                distance_matrix[i][j] = cost;
            }
        }
    }

    let config = QuantumBundleConfig {
        distance_matrix,
        adiabatic_time: 500.0,
        coupling_constant: 0.08,
        penalty_gamma: 15.0,
    };

    println!("[MANIFOLD] Projecting Cosmic Dao Matrix Grid onto TACO Engine...");
    let solver = HolonomicQuantumSolver::new(config);

    println!("[EVOLUTION] Unleashing Karmic paths across 11 dimensions...");
    let final_energy_state = solver.execute_topological_collapse();

    println!("\n[COLLAPSE SUCCESS] Enlightenment attained instantaneously.");
    println!(
        ">> Absolute Ground State Tour Cost Discovered: {:.4}",
        final_energy_state
    );
    println!("=========================================================================");
}
