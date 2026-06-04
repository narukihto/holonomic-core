use crate::core::tension::TensionMatrix;
use crate::physics::calculate_jacobian_manifold_operator;
use std::collections::HashSet;
use std::env;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut current_path: Vec<usize> = (0..n).collect();
    let max_epochs = if env::var("CI").is_ok() { 100 } else { 500 };
    let mut quantum_temperature = 500.0f64;
    let cooling_rate = 0.95;

    for epoch in 0..max_epochs {
        let jacobian_matrix = calculate_jacobian_manifold_operator(&current_path, &tension);
        let mut converged = true;

        for i in 0..n {
            let next = (i + 1) % n;
            let mut internal_pressure = 0.0f64;
            let mut external_pressure = 0.0f64;

            for j in 0..n {
                internal_pressure += jacobian_matrix[current_path[i]][current_path[j]];
                external_pressure += jacobian_matrix[current_path[next]][current_path[j]];
            }

            let delta = internal_pressure - external_pressure;
            let prob = (delta / quantum_temperature.max(1e-6)).exp();

            if delta > 0.0 || rand::random::<f64>() < prob {
                current_path.swap(i, next);
                converged = false;
            }
        }

        quantum_temperature *= cooling_rate;
        if converged && epoch > 50 {
            break;
        }
    }

    let unique_nodes: HashSet<usize> = current_path.iter().cloned().collect();
    assert_eq!(unique_nodes.len(), n);
    current_path
}
