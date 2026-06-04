use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut current_path: Vec<usize> = (0..n).collect();

    let mut improved = true;
    let mut iterations = 0;

    while improved && iterations < 20 {
        improved = false;
        iterations += 1;

        for i in 0..n - 3 {
            for j in i + 2..n - 1 {
                for k in j + 2..n {
                    let a = current_path[i];
                    let b = current_path[i + 1];
                    let c = current_path[j];
                    let d = current_path[j + 1];
                    let e = current_path[k];
                    let f = current_path[(k + 1) % n];

                    let old_dist = tension.data[a][b] + tension.data[c][d] + tension.data[e][f];
                    let new_dist = tension.data[a][c] + tension.data[b][e] + tension.data[d][f];

                    if new_dist < old_dist {
                        let mut new_segment = Vec::new();
                        new_segment.extend_from_slice(&current_path[i + 1..=j]);
                        new_segment.reverse();
                        current_path[i + 1..=j].copy_from_slice(&new_segment);
                        improved = true;
                    }
                }
                if improved {
                    break;
                }
            }
            if improved {
                break;
            }
        }
    }
    current_path
}
