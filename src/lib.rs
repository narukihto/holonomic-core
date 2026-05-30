//! # ARK-Penta-V Sovereign Core
//! 
//! نواة سيادية لإدارة الاستقرار الهندسي والانهيار الطوبولوجي للمعضلات التوافقية.
//! هذا الـ Crate يدمج محرك Ricci-ARK للتدفق الجيوديسي مع طبقة الحماية Penta-V.

pub mod core;
pub mod crypto;
pub mod evolution;
pub mod observer;
pub mod physics;

pub use core::manifold::SovereignManifold;
pub use core::tension::TensionMatrix;
pub use crypto::lwe::SovereignSignature;

/// النقطة الدخول الرئيسية لعملية "الانهيار السيادي" (Sovereign Collapse).
/// تقوم هذه الدالة بأخذ العقد وتحويلها إلى مسار جيوديسي محصن.
pub fn execute_sovereign_collapse<const N: usize>(nodes: &[[f64; 2]; N]) -> Vec<usize> {
    // 1. إنشاء الفضاء المانيفولد (Space Folding)
    let manifold = SovereignManifold::new(nodes);
    
    // 2. حساب مصفوفة التوتر وتوقيعها (LWE Security)
    let tension = manifold.compute_tension_matrix();
    let signature = crypto::lwe::sign_manifold(&tension);
    
    if !signature.is_valid() {
        panic!("🚨 Geometric Lockdown: Tampering detected in the Sovereign Core!");
    }
    
    // 3. محرك الانهيار الأدياباتي (Geodesic Flow)
    evolution::mod::collapse_to_optimum(tension)
}

/// تهيئة النظام (Heartbeat Initialization)
pub fn init_sovereign_core() {
    observer::mod::start_heartbeat();
    physics::mod::calibrate_resonance_lattice();
}
