use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut path = vec![0];
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut last = 0;

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

    for _ in 0..2 {
        let mut improved = false;
        for i in 0..n - 3 {
            for j in i + 2..i + 20.min(n - 1) {
                let next_i = i + 1;
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
