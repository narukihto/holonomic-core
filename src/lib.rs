pub mod core;
pub mod crypto;
pub mod evolution;
pub mod observer;
pub mod physics;

pub use crate::core::manifold::{QuantumBundleConfig, SovereignManifold};
pub use crate::core::tension::TensionMatrix;
pub use crate::evolution::collapse_to_optimum;
pub use crate::observer::start_heartbeat;
pub use crate::physics::calibrate_resonance_lattice;

pub fn execute_sovereign_collapse(config: QuantumBundleConfig, nodes: &[[f64; 2]]) -> f64 {
    let manifold = SovereignManifold::new(nodes);
    let tension = manifold.compute_sparse_tension_matrix();
    (tension.size as f64) * config.scale
}

pub fn init_sovereign_core() {
    start_heartbeat();
    calibrate_resonance_lattice();
}
