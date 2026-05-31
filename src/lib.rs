pub mod core;
pub mod crypto;
pub mod evolution;
pub mod observer;
pub mod physics;

pub use crate::core::manifold::{HolonomicQuantumSolver, QuantumBundleConfig, SovereignManifold};
pub use crate::core::tension::TensionMatrix;
pub use crate::evolution::collapse_to_optimum;
pub use crate::observer::start_heartbeat;
pub use crate::physics::calibrate_resonance_lattice;

pub fn execute_sovereign_collapse(config: QuantumBundleConfig, nodes: &[[f64; 2]; 2]) -> f64 {
    let manifold = SovereignManifold::new(nodes);
    let mut tension = manifold.compute_tension_matrix();
    
    tension.enforce_terminal_boundary(config.adiabatic_time);
    
    config.execute_sovereign_collapse(&manifold)
}
pub fn init_sovereign_core() {
    start_heartbeat();
    calibrate_resonance_lattice();
}
