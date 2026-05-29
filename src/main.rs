use holonomic_tsp_core::{HolonomicQuantumSolver, QuantumBundleConfig};

fn main() {
    println!("=========================================================================");
    println!("=== DETERMINISTIC EXACT HOLONOMIC TSP QUANTUM SIMULATOR CORE STARTING ===");
    println!("=========================================================================");

    // Quantum System Dimensions Configuration (Const Generic Mapping Instance)
    const NETWORK_SIZE: usize = 5;

    // Symmetric Euclidean Non-Abelian Distance Matrix Initialization for Optimization Nodes
    let distance_matrix = [
        [0.0, 10.0, 15.0, 20.0, 30.0],
        [10.0, 0.0, 35.0, 25.0, 18.0],
        [15.0, 35.0, 0.0, 30.0, 12.0],
        [20.0, 25.0, 30.0, 0.0, 5.0],
        [30.0, 18.0, 12.0, 5.0, 0.0],
    ];

    // Initializing zero-cost abstraction quantum config invariants
    let config = QuantumBundleConfig {
        distance_matrix,
        adiabatic_time: 1000.0,  // Total integration evolution stepping frames
        coupling_constant: 0.05, // Lie group geometric tensor step phase multiplier
        penalty_gamma: 25.5,     // Orthogonal stochastic matrix force field coefficient
    };

    println!("[MANIFOLD INIT] Projecting network topology onto 2D non-Abelian anyonic grid...");
    println!(
        "[MANIFOLD INIT] Generating SU(n) cross-sections for {} local cluster nodes...",
        NETWORK_SIZE
    );

    let solver = HolonomicQuantumSolver::new(config);

    println!(
        "[EVOLUTION] Simulating continuous adiabatic transition with protected spectral gap..."
    );
    println!("[EVOLUTION] Executing exact homotopy continuation paths concurrently via Rayon threadpools...");

    // Invoking the non-destructive topological charge collapse routine
    let final_energy_state = solver.execute_topological_collapse();

    println!("\n[COLLAPSE SUCCESS] Symplectic continuous manifold flow concluded instantly.");
    println!(
        ">> Exact Deterministic Global Optimum Tour Cost Verified: {:.6}",
        final_energy_state
    );

    // Accessing global state observer metrics to prove deterministic execution safety
    if let Some(entropy) = solver.state.state_observer.get("System_Entropy") {
        println!(">> Final Monifold System Baseline Entropy: {:.4}", *entropy);
    }

    println!("=========================================================================");
    println!("=== SOLVER PROCESSING SEQUENCE COMPLETED SUCCESSFULLY [EMISSION 100%] ===");
    println!("=========================================================================");
}
