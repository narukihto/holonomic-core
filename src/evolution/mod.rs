use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut best_path = Vec::with_capacity(n);
    let mut best_cost = f64::MAX;

    // فحص 3 نقاط بداية مختلفة بشكل سريع لكسر جمود الجار الأقرب الثابت
    let starts = [0, n / 3, (2 * n) / 3];

    for &start in &starts {
        let mut path = vec![start];
        let mut visited = vec![false; n];
        visited[start] = true;
        let mut last = start;

        for _ in 1..n {
            let mut best = None;
            let mut min = f64::MAX;
            for (i, &is_visited) in visited.iter().enumerate() {
                if !is_visited && tension.data[last][i] < min {
                    min = tension.data[last][i];
                    best = Some(i);
                }
            }
            if let Some(next) = best {
                path.push(next);
                visited[next] = true;
                last = next;
            }
        }

        let mut current_cost = 0.0;
        for i in 0..n {
            current_cost += tension.data[path[i]][path[(i + 1) % n]];
        }

        if current_cost < best_cost {
            best_cost = current_cost;
            best_path = path;
        }
    }

    let mut path = best_path;

    for _ in 0..3 {
        let mut improved = false;
        for i in 0..n - 3 {
            let next_i = i + 1;
            let end = (i + 31).min(n);
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
    path
}
