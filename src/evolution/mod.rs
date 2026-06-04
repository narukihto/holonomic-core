use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut visited = vec![false; n];
    let mut path = Vec::with_capacity(n);
    let mut nearest_neighbors = vec![Vec::with_capacity(3); n];

    let mut current = 0;
    path.push(current);
    visited[current] = true;

    for _ in 1..n {
        let mut best_next = 0;
        let mut min_cost = f64::MAX;
        let row = &tension.data[current];

        let mut c1 = 0;
        let mut c2 = 0;
        let mut m1 = f64::MAX;
        let mut m2 = f64::MAX;

        for (i, &is_visited) in visited.iter().enumerate() {
            if !is_visited {
                let cost = row[i];
                if cost < min_cost {
                    min_cost = cost;
                    best_next = i;
                }
                if cost < m1 {
                    m2 = m1;
                    c2 = c1;
                    m1 = cost;
                    c1 = i;
                } else if cost < m2 {
                    m2 = cost;
                    c2 = i;
                }
            }
        }

        if m1 < f64::MAX {
            nearest_neighbors[current].push(c1);
        }
        if m2 < f64::MAX {
            nearest_neighbors[current].push(c2);
        }

        current = best_next;
        path.push(current);
        visited[current] = true;
    }

    let mut pos = vec![0; n];
    for (index, &node) in path.iter().enumerate() {
        pos[node] = index;
    }

    let mut improved = true;
    let mut iterations = 0;

    while improved && iterations < 25 {
        improved = false;
        iterations += 1;

        for u in 0..n {
            let idx_u = pos[u];
            if idx_u >= n - 1 {
                continue;
            }

            for &v in &nearest_neighbors[u] {
                let idx_v = pos[v];
                if idx_u == idx_v {
                    continue;
                }

                let i = idx_u.min(idx_v);
                let j = idx_u.max(idx_v);

                if i + 1 >= j || j >= n - 1 {
                    continue;
                }

                let next_i = i + 1;
                let next_j = j + 1;

                let d1 = tension.data[path[i]][path[next_i]] + tension.data[path[j]][path[next_j]];
                let d2 = tension.data[path[i]][path[j]] + tension.data[path[next_i]][path[next_j]];

                if d2 < d1 {
                    path[next_i..=j].reverse();
                    for k in next_i..=j {
                        pos[path[k]] = k;
                    }
                    improved = true;
                }
            }
        }
    }

    path
}
