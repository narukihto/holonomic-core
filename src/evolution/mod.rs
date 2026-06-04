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
        path.push(current);
        visited[current] = true;
    }

    let mut improved = true;
    let mut iterations = 0;
    let max_iters = if n <= 10000 { 12 } else { 2 };

    while improved && iterations < max_iters {
        improved = false;
        iterations += 1;

        for i in 0..n - 3 {
            let next_i = i + 1;
            for j in i + 2..n {
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
