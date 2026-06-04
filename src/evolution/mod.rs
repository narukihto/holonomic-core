use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut path: Vec<usize> = (0..n).collect();

    let mut chunks = 1;
    while chunks * chunks < n {
        chunks += 1;
    }
    chunks = (chunks / 2).max(1);

    path.sort_by(|&a, &b| {
        let cost_a = tension.data[0][a];
        let cost_b = tension.data[0][b];
        let chunk_a = (cost_a * chunks as f64) as usize;
        let chunk_b = (cost_b * chunks as f64) as usize;

        if chunk_a != chunk_b {
            chunk_a.cmp(&chunk_b)
        } else if chunk_a % 2 == 0 {
            cost_a
                .partial_cmp(&cost_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        } else {
            cost_b
                .partial_cmp(&cost_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        }
    });

    for _ in 0..15 {
        let mut improved = false;
        for i in 0..n - 3 {
            let next_i = i + 1;
            let end = (i + 150).min(n);
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
