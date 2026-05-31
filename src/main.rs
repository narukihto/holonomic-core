use ark_penta_v_core::{execute_sovereign_collapse, init_sovereign_core, QuantumBundleConfig};

fn main() {
    init_sovereign_core();

    println!("🏛️ ARK-Penta-V Core Initialized. Sovereign Mode: ACTIVE.");

    let nodes: [[f64; 2]; 4] = [[0.0, 0.0], [1.0, 5.0], [5.0, 2.0], [3.0, 0.0]];
    
    let config = QuantumBundleConfig {
        distance_matrix: vec![],
        adiabatic_time: 500.0,
    };

    println!("📐 Folding manifold for {} nodes...", nodes.len());

    let result = execute_sovereign_collapse(config, &nodes);

    println!("✅ Sovereign Collapse Complete. Metric Convergence: {:?}", result);
}
