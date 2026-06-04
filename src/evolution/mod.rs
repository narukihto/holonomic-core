use crate::core::tension::TensionMatrix;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut current_path: Vec<usize> = (0..n).collect();
    let mut rng = StdRng::seed_from_u64(42);
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
                if (tension.data[current_path[i]][current_path[next_i]]
                    + tension.data[current_path[j]][current_path[next_j]])
                    > (tension.data[current_path[i]][current_path[j]]
                        + tension.data[current_path[next_i]][current_path[next_j]])
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
