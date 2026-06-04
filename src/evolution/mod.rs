use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut path: Vec<usize> = (0..n).collect();
    let mut radial_distances = Vec::with_capacity(n);

    for i in 0..n {
        radial_distances.push(tension.data[0][i]);
    }

    path.sort_by(|&a, &b| {
        radial_distances[a]
            .partial_cmp(&radial_distances[b])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut improved = true;
    let mut iterations = 0;
    
    let max_iters = if n <= 10000 { 15 } else { 2 };
    let window_size = if n <= 10000 { 350 } else { 30 };

    while improved && iterations < max_iters {
        improved = false;
        iterations += 1;

        for i in 0..n - 3 {
            let next_i = i + 1;
            let end = (i + window_size).min(n);

            for j in i + 2..end {
                let next_j = (j + 1) % n;

                let d1 = tension.data[path[i]][path[next_i]] + tension.data[path[j]][path[next_j]];
                let d2 = tension.data[path[i]][path[j]] + tension.data[path[next_i]][path[next_j]];

                if d2 < d1 {
                    path[next_i..=j].reverse();
                    improved = true;
                }
            }
        }
    }

    path
}
