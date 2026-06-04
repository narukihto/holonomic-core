use crate::core::tension::TensionMatrix;
use std::env;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut current_path: Vec<usize> = (0..n).collect();
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

                let d1 = tension.data[u][v];
                let d2 = tension.data[x][y];
                let d3 = tension.data[u][x];
                let d4 = tension.data[v][y];

                if (d1 + d2) > (d3 + d4) {
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
