use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut path: Vec<usize> = (0..n).collect();
    let mut coords = Vec::with_capacity(n);

    for i in 0..n {
        let x = tension.data[0][i];
        let y = tension.data[i % n][(i + 1) % n];
        coords.push((x, y));
    }

    path.sort_by(|&a, &b| {
        let ca = coords[a];
        let cb = coords[b];

        let cell_x_a = (ca.0 * 50.0) as i64;
        let cell_y_a = (ca.1 * 50.0) as i64;
        let cell_x_b = (cb.0 * 50.0) as i64;
        let cell_y_b = (cb.1 * 50.0) as i64;

        if cell_x_a != cell_x_b {
            cell_x_a.cmp(&cell_x_b)
        } else if cell_x_a.is_multiple_of(2) {
            cell_y_a.cmp(&cell_y_b)
        } else {
            cell_y_b.cmp(&cell_y_a)
        }
    });

    for block_size in [62, 124] {
        if block_size >= n {
            break;
        }
        for chunk in path.chunks_mut(block_size) {
            let chunk_len = chunk.len();
            if chunk_len < 3 {
                continue;
            }
            for _ in 0..4 {
                let mut improved = false;
                for i in 0..chunk_len - 2 {
                    let next_i = i + 2;
                    for j in next_i..chunk_len {
                        let next_j = (j + 1) % chunk_len;

                        let d1 = tension.data[chunk[i]][chunk[i + 1]]
                            + tension.data[chunk[j]][chunk[next_j]];
                        let d2 = tension.data[chunk[i]][chunk[j]]
                            + tension.data[chunk[i + 1]][chunk[next_j]];

                        if d2 < d1 {
                            chunk[(i + 1)..=j].reverse();
                            improved = true;
                        }
                    }
                }
                if !improved {
                    break;
                }
            }
        }
    }

    for _ in 0..8 {
        let mut improved = false;
        for i in 0..n - 3 {
            let next_i = i + 1;
            let end = (i + 300).min(n);

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
