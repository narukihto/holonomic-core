use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut path: Vec<usize> = (0..n).collect();
    let mut metrics = Vec::with_capacity(n);

    for i in 0..n {
        let d0 = tension.data[0][i];
        let d1 = tension.data[n / 4][i];
        let d2 = tension.data[n / 2][i];
        let d3 = tension.data[3 * n / 4][i];
        metrics.push((d0, d1, d2, d3));
    }

    path.sort_by(|&a, &b| {
        let ma = metrics[a];
        let mb = metrics[b];

        let cell_x_a = ((ma.0 - ma.1) * 0.05) as i64;
        let cell_y_a = ((ma.2 - ma.3) * 0.05) as i64;
        let cell_x_b = ((mb.0 - mb.1) * 0.05) as i64;
        let cell_y_b = ((mb.2 - mb.3) * 0.05) as i64;

        if cell_x_a != cell_x_b {
            cell_x_a.cmp(&cell_x_b)
        } else if cell_x_a % 2 == 0 {
            cell_y_a.cmp(&cell_y_b)
        } else {
            cell_y_b.cmp(&cell_y_a)
        }
    });

    let mut improved = true;
    let mut iterations = 0;

    let max_iters = if n <= 10000 { 45 } else { 2 };
    let window_size = if n <= 10000 { 180 } else { 30 };

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
