use crate::core::tension::TensionMatrix;
use crate::physics::calculate_jacobian_manifold_operator;
use rug::Float;

pub struct CollapseState {
    pub path: Vec<usize>,
    pub iteration: u64,
}

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 2 {
        return (0..n).collect();
    }

    let mut current_path: Vec<usize> = (0..n).collect();
    let max_epochs = 200;

    for _epoch in 0..max_epochs {
        let jacobian_matrix = calculate_jacobian_manifold_operator(&current_path, &tension);
        let mut converged = true;

        for i in 0..n {
            let next = (i + 1) % n;

            let mut internal_pressure = Float::with_val(128, 0.0);
            for j in 0..n {
                internal_pressure += &jacobian_matrix[current_path[i]][current_path[j]];
            }

            let mut external_pressure = Float::with_val(128, 0.0);
            for j in 0..n {
                external_pressure += &jacobian_matrix[current_path[next]][current_path[j]];
            }

            if internal_pressure > external_pressure {
                current_path.swap(i, next);
                converged = false;
            }
        }

        if converged {
            break;
        }
    }

    current_path
}
