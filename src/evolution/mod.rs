use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut path = Vec::with_capacity(n);
    let mut visited = vec![false; n];

    let mut current = 0;
    path.push(current);
    visited[current] = true;

    let stride = if n > 10000 { 7 } else { 1 };

    while path.len() < n {
        let mut best_next = None;
        let mut min_cost = f64::MAX;

        let mut i = 0;
        while i < n {
            if !visited[i] {
                let cost = tension.data[current][i];
                if cost < min_cost {
                    min_cost = cost;
                    best_next = Some(i);
                }
            }
            i += stride;
        }

        let next_node = match best_next {
            Some(node) => node,
            None => {
                let mut fallback = 0;
                for (idx, &v) in visited.iter().enumerate() {
                    if !v {
                        fallback = idx;
                        break;
                    }
                }
                fallback
            }
        };

        current = next_node;
        path.push(current);
        visited[current] = true;
    }

    for _ in 0..12 {
        let mut improved = false;
        for i in 0..n - 3 {
            let next_i = i + 1;
            let end = (i + 120).min(n);
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
        if !improved {
            break;
        }
    }

    visited.fill(false);
    drop(visited);

    path
}
