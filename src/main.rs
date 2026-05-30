use ark_penta_v_core::{execute_sovereign_collapse, init_sovereign_core};

fn main() {
    init_sovereign_core();

    println!("🏛️ ARK-Penta-V Core Initialized. Sovereign Mode: ACTIVE.");

    let nodes: [[f64; 2]; 4] = [[0.0, 0.0], [1.0, 5.0], [5.0, 2.0], [3.0, 0.0]];

    println!("📐 Folding manifold for {} nodes...", nodes.len());

    let path = execute_sovereign_collapse(&nodes);

    println!("✅ Sovereign Collapse Complete. Optimal Path: {:?}", path);
}
