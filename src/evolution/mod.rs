use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut visited = vec![false; n];
    let mut path = Vec::with_capacity(n);

    let mut current = 0;
    path.push(current);
    visited[current] = true;

    for _ in 1..n {
        let mut best_next = 0;
        let mut min_score = f64::MAX;
        let row = &tension.data[current];
        let center_row = &tension.data[0];
        let far_row = &tension.data[n - 1];

        for (i, &is_visited) in visited.iter().enumerate() {
            if !is_visited {
                let cost = row[i];
                let score = cost + 0.15 * center_row[i] - 0.05 * far_row[i];
                if score < min_score {
                    min_score = score;
                    best_next = i;
                }
            }
        }

        current = best_next;
        path.push(current);
        visited[current] = true;
    }

    let mut improved = true;
    let mut iterations = 0;
    let max_iters = if n <= 10000 { 8 } else { 1 };
    let window_size = if n <= 10000 { 120 } else { 20 };

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
