use crate::core::tension::TensionMatrix;
use crate::physics::calculate_jacobian_manifold_operator;
use rug::Float;
use std::collections::HashSet;
use std::env;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut current_path: Vec<usize> = (0..n).collect();
    let max_epochs = if env::var("CI").is_ok() { 50 } else { 500 };
    let mut quantum_temperature = 500.0f64;
    let cooling_rate = 0.92;
    let k_neighbors = if n > 5 { 5 } else { n.saturating_sub(1) };

    for epoch in 0..max_epochs {
        let jacobian_matrix = calculate_jacobian_manifold_operator(&current_path, &tension);
        let mut converged = true;

        for i in 0..n {
            let next = (i + 1) % n;
            let mut internal_pressure = Float::with_val(64, 0.0);
            let start_j = i.saturating_sub(k_neighbors / 2);
            let end_j = std::cmp::min(start_j + k_neighbors + 1, n);

            for j in start_j..end_j {
                if j < n && i < n {
                    internal_pressure += &jacobian_matrix[current_path[i]][current_path[j]];
                }
            }

            let mut external_pressure = Float::with_val(64, 0.0);
            for j in start_j..end_j {
                if j < n && next < n {
                    external_pressure += &jacobian_matrix[current_path[next]][current_path[j]];
                }
            }

            let delta = internal_pressure - external_pressure;
            let should_swap = if delta > 0.0 {
                true
            } else {
                quantum_temperature > 0.01 && (rand::random::<f64>() < (-(quantum_temperature) / 20.0).exp())
            };

            if should_swap && i != next {
                current_path.swap(i, next);
                converged = false;
            }

            if epoch > 40 && i % 12 == 0 {
                let target_jump = rand::random::<usize>() % n;
                if target_jump != i && target_jump != next {
                    current_path.swap(i, target_jump);
                    converged = false;
                }
            }
        }
        quantum_temperature *= cooling_rate;
        if converged && epoch > 20 {
            break;
        }
    }

    let unique_nodes: HashSet<usize> = current_path.iter().cloned().collect();
    assert_eq!(unique_nodes.len(), n, "Logic error: duplicate nodes");
    current_path
}
