use crate::core::tension::TensionMatrix;
use rug::Float;

pub struct CollapseState {
    pub path: Vec<usize>,
    pub gradient: Vec<Float>,
    pub iteration: u64,
}

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let mut state = CollapseState {
        path: (0..tension.size).collect(),
        gradient: vec![Float::with_val(128, 0.0); tension.size],
        iteration: 0,
    };

    while state.iteration < 1000 {
        state.gradient = calculate_gradient(&state.path, &tension);

        if is_zero_or_negligible(&state.gradient) {
            break;
        }

        state.path = refine_geodesic(state.path, &state.gradient);
        state.iteration += 1;
    }

    state.path
}

fn calculate_gradient(path: &[usize], tension: &TensionMatrix) -> Vec<Float> {
    path.iter()
        .map(|&node| {
            let mut force = Float::with_val(128, 0.0);
            for j in 0..tension.size {
                force += &tension.data[node][j];
            }
            force
        })
        .collect()
}

fn refine_geodesic(path: Vec<usize>, gradient: &[Float]) -> Vec<usize> {
    let mut new_path = path;
    new_path.sort_by(|&a, &b| {
        gradient[a]
            .partial_cmp(&gradient[b])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    new_path
}

fn is_zero_or_negligible(gradient: &[Float]) -> bool {
    let epsilon = Float::with_val(128, 1e-20);
    gradient.iter().all(|g| g.clone().abs() < epsilon)
}
