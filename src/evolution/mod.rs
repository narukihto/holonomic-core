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

    for _ in 0..3 {
        let mut improved = false;
        for i in 0..n - 2 {
            let j = i + 1;
            let k = i + 2;
            let d1 = tension.data[path[i]][path[j]] + tension.data[path[j]][path[k]];
            let d2 = tension.data[path[i]][path[k]] + tension.data[path[k]][path[j]];
            if d2 < d1 {
                path.swap(j, k);
                improved = true;
            }
        }
        if !improved {
            break;
        }
    }
    path
}
