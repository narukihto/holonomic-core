use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut current_path = vec![0];
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut last = 0;

    for _ in 1..n {
        let mut nearest = None;
        let mut min_dist = f64::MAX;
        
        for (next, &is_visited) in visited.iter().enumerate() {
            if !is_visited && tension.data[last][next] < min_dist {
                min_dist = tension.data[last][next];
                nearest = Some(next);
            }
        }
        
        if let Some(next) = nearest {
            current_path.push(next);
            visited[next] = true;
            last = next;
        }
    }

    for _ in 0..5 {
        let mut improved = false;
        for i in 0..n - 3 {
            let next_i = i + 1;
            for j in i + 2..i + 50 {
                let k = j % n;
                let next_j = (k + 1) % n;

                if (tension.data[current_path[i]][current_path[next_i]]
                    + tension.data[current_path[k]][current_path[next_j]])
                    > (tension.data[current_path[i]][current_path[k]]
                        + tension.data[current_path[next_i]][current_path[next_j]])
                {
                    current_path[next_i..=k].reverse();
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
