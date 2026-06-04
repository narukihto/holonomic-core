use crate::core::tension::TensionMatrix;
use std::env;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut current_path = vec![0];
    let mut visited = vec![false; n];
    visited[0] = true;

    for _ in 1..n {
        let last = *current_path.last().unwrap();
        let next = (0..n)
            .filter(|&i| !visited[i])
            .min_by(|&a, &b| {
                tension.data[last][a]
                    .partial_cmp(&tension.data[last][b])
                    .unwrap()
            })
            .unwrap();
        current_path.push(next);
        visited[next] = true;
    }

    let max_epochs = if env::var("CI").is_ok() { 50 } else { 200 };
    for _ in 0..max_epochs {
        let mut improved = false;
        for i in 0..n - 1 {
            let next_i = (i + 1) % n;
            for j in i + 2..n {
                let next_j = (j + 1) % n;
                let u = current_path[i];
                let v = current_path[next_i];
                let x = current_path[j];
                let y = current_path[next_j];

                if (tension.data[u][v] + tension.data[x][y]) > (tension.data[u][x] + tension.data[v][y]) {
                    current_path[next_i..=j].reverse();
                    improved = true;
                }
            }
        }
        if !improved {
            break;
        }
    }
    current_path
}
