use crate::core::tension::TensionMatrix;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    let mut rng = StdRng::seed_from_u64(42);
    let mut path: Vec<usize> = (0..n).collect();
    path.shuffle(&mut rng);

    for _ in 0..5 {
        let mut improved = false;
        for i in 0..n - 3 {
            let next_i = i + 1;
            let j = i + 2;
            let k = i + 3;

            let current_dist = tension.data[path[i]][path[next_i]] + tension.data[path[j]][path[k]];
            let new_dist = tension.data[path[i]][path[j]] + tension.data[path[next_i]][path[k]];

            if new_dist < current_dist {
                path.swap(next_i, j);
                improved = true;
            }
        }
        if !improved {
            break;
        }
    }
    path
}
