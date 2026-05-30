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

pub fn execute_sovereign_collapse<const N: usize>(nodes: &[[f64; 2]; N]) -> Vec<usize> {
    let manifold = SovereignManifold::new(nodes);
    let tension = manifold.compute_tension_matrix();
    let signature = crypto::lwe::sign_manifold(&tension);

    if !signature.is_valid() {
        panic!("🚨 Geometric Lockdown: Tampering detected in the Sovereign Core!");
    }

    collapse_to_optimum(tension)
}

pub fn init_sovereign_core() {
    start_heartbeat();
    calibrate_resonance_lattice();
}
