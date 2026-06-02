use crate::core::tension::TensionMatrix;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct SovereignObserver {
    pub start_time: Instant,
    pub epoch_threshold: u64,
}

impl Default for SovereignObserver {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignObserver {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            epoch_threshold: 0,
        }
    }

    #[allow(clippy::manual_is_multiple_of)]
    pub fn observe_collapse_integrity(&self, _tension: &TensionMatrix, current_epoch: u64) {
        if current_epoch % 50 == 0 {
            let elapsed = self.start_time.elapsed().as_secs_f64();
            if elapsed > 15.0 {
                eprintln!(
                    "⚠️ Warning: Manifold collapse under heavy stress at epoch {}",
                    current_epoch
                );
            }
        }
    }

    pub fn log_completion(&self, final_distance: u64) {
        println!(
            "✅ Sovereign collapse completed. Final optimal distance: {}",
            final_distance
        );
    }
}

pub fn start_heartbeat() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    println!("[OBSERVER] Sovereign Heartbeat Active at: {}", timestamp);
}
