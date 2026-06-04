use crate::core::tension::TensionMatrix;
use rand::seq::SliceRandom;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut current_path: Vec<usize> = (0..n).collect();
    let mut rng = rand::thread_rng();
    current_path.shuffle(&mut rng);

    for _ in 0..50 {
        let mut improved = false;

        for i in 0..n - 1 {
            let next_i = i + 1;
            let range = n - i - 2;

            if range == 0 {
                continue;
            }

            for _ in 0..100 {
                let j = (i + 2 + (rand::random::<usize>() % range)).min(n - 1);
                let next_j = (j + 1) % n;

                let u = current_path[i];
                let v = current_path[next_i];
                let x = current_path[j];
                let y = current_path[next_j];

                if (tension.data[u][v] + tension.data[x][y])
                    > (tension.data[u][x] + tension.data[v][y])
                {
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
