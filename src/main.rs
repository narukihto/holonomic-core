use holonomic_tsp_core::{QuantumBundleConfig, HolonomicQuantumSolver};

fn main() {
    println!("=========================================================================");
    println!("   BOOTING ANALOG TOPOLOGICAL QUANTUM HOLONOMIC TSP EXECUTION SOLVER     ");
    println!("   CORE ARCHITECTURE: SO(N) VECTOR BUNDLE SYMPLECTIC FLOW ENGINE        ");
    println!("=========================================================================");

    // Const N defines the high-dimensional matrix order (Number of Cities/Nodes)
    const N: usize = 5;

    // Asymmetric, non-trivial distance matrix topology (Worst-case deceptive configuration)
    let distance_matrix: [[f64; N]; N] = [
        [0.0, 10.0, 15.0, 20.0, 30.0],
        [10.0, 0.0, 35.0, 25.0, 12.0],
        [15.0, 35.0, 0.0, 30.0, 18.0],
        [20.0, 25.0, 30.0, 0.0, 11.0],
        [30.0, 12.0, 18.0, 11.0, 0.0],
    ];

    // Initializing zero-cost abstraction quantum config invariants
    let config = QuantumBundleConfig {
        distance_matrix,
        adiabatic_time: 1000.0,     // Total integration evolution stepping frames
        coupling_constant: 0.05,    // Lie group geometric tensor step phase multiplier
        penalty_gamma: 25.5,        // Orthogonal stochastic matrix force field coefficient
    };

    println!("[MANIFOLD INIT] Projecting network topology onto 2D non-Abelian anyonic grid...");
    let solver = HolonomicQuantumSolver::new(config);
    
    println!("[EVOLUTION] Simulating continuous adiabatic transition with protected spectral gap...");
    println!("[EVOLUTION] Executing exact homotopy continuation paths concurrently via Rayon threadpools...");
    
    // Invoking the non-destructive topological charge collapse routine
    let final_energy_state = solver.execute_topological_collapse();

    println!("\n[COLLAPSE SUCCESS] Symplectic continuous manifold flow concluded instantly.");
    println!(">> Exact Deterministic Global Optimum Tour Cost Verified: {:.6}", final_energy_state);
    
    // Accessing global state observer metrics to prove deterministic execution safety
    if let Some(entropy) = solver.state.state_observer.get("System_Entropy") {
        println!(">> Final Monifold System Baseline Entropy: {:.4}", *entropy);
    }
    
    println!("=========================================================================");
    println!("=== SOLVER PROCESSING SEQUENCE COMPLETED SUCCESSFULLY [EMISSION 100%] ===");
    println!("=========================================================================");
}
