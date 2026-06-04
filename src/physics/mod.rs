use crate::core::tension::TensionMatrix;

pub const RESONANCE_STIFFNESS: f64 = 1.0;
pub const DAMPING_COUNT: f64 = 0.5;

pub fn calibrate_resonance_lattice() {
    apply_field_constraints();
}

fn apply_field_constraints() {}

pub fn calculate_jacobian_manifold_operator(
    path: &[usize],
    tension: &TensionMatrix,
) -> Vec<Vec<f64>> {
    let n = tension.size;
    let mut jacobian = vec![vec![0.0; n]; n];

    if path.len() < 2 {
        return jacobian;
    }

    for i in 0..n {
        for j in 0..n {
            if i != j && i < path.len() && j < path.len() {
                let u = path[i];
                let v = path[j];

                if u < tension.data.len() && v < tension.data[u].len() {
                    let metric_val = tension.data[u][v];
                    if metric_val != 0.0 {
                        jacobian[i][j] = 1.0 / metric_val;
                    }
                }
            }
        }
    }
    jacobian
}
