//! # ARK-Penta-V Sovereign Core
//! 
//! Sovereign kernel for managing geometric stability and topological collapse of combinatorial problems.
//! This crate integrates the Ricci-ARK engine for Geodesic Flow with the Penta-V security substrate.

pub mod core;
pub mod crypto;
pub mod evolution;
pub mod observer;
pub mod physics;

use crate::core::manifold::SovereignManifold;
use crate::evolution::collapse_to_optimum;
use crate::observer::start_heartbeat;
use crate::physics::calibrate_resonance_lattice;

/// Entry point for the "Sovereign Collapse" process.
/// This function maps input nodes into a Geodesic Manifold and anchors the result against tampering.
pub fn execute_sovereign_collapse<const N: usize>(nodes: &[[f64; 2]; N]) -> Vec<usize> {
    // 1. Initialize the Riemannian Manifold (Space Folding)
    let manifold = SovereignManifold::new(nodes);
    
    // 2. Compute the tension matrix and generate an LWE signature
    // Any structural divergence in input constraints will invalidate this signature.
    let tension = manifold.compute_tension_matrix();
    let signature = crypto::lwe::sign_manifold(&tension);
    
    if !signature.is_valid() {
        panic!("🚨 Geometric Lockdown: Tampering detected in the Sovereign Core!");
    }
    
    // 3. Trigger the Adiabatic Collapse (Geodesic Flow)
    // Removed 'mod::' to respect Rust's module resolution rules.
    collapse_to_optimum(tension)
}

/// Initialize the Kernel Heartbeat and calibrate the resonance lattice.
pub fn init_sovereign_core() {
    // Removed 'mod::' for direct module invocation.
    start_heartbeat();
    calibrate_resonance_lattice();
}
