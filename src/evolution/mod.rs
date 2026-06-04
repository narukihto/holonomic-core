use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut best_path = Vec::with_capacity(n);
    let mut min_total_cost = f64::MAX;
    let sample_seeds = if n > 10000 { vec![0, n / 5, (2 * n) / 5] } else { vec![0] };

    for start_seed in sample_seeds {
        let mut visited = vec![false; n];
        let mut current_path = Vec::with_capacity(n);
        let mut current = start_seed;

        current_path.push(current);
        visited[current] = true;

        for _ in 1..n {
            let mut best_next = 0;
            let mut min_cost = f64::MAX;
            let row = &tension.data[current];

            for (i, &is_visited) in visited.iter().enumerate() {
                if !is_visited {
                    let cost = row[i];
                    if cost < min_cost {
                        min_cost = cost;
                        best_next = i;
                    }
                }
            }
            current = best_next;
            current_path.push(current);
            visited[current] = true;
        }

        let mut total_cost = 0.0;
        for i in 0..n {
            total_cost += tension.data[current_path[i]][current_path[(i + 1) % n]];
        }

        if total_cost < min_total_cost {
            min_total_cost = total_cost;
            best_path = current_path;
        }
    }

    let mut path = best_path;
    let mut improved = true;
    let mut limit = 0;

    while improved && limit < 5 {
        improved = false;
        limit += 1;

        for i in 0..n - 3 {
            let next_i = i + 1;
            let end = (i + 130).min(n);

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
