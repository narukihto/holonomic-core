use crate::core::tension::TensionMatrix;
use rayon::prelude::*;

#[derive(Clone)]
pub struct SovereignManifold {
    pub nodes: Vec<[f64; 2]>,
}

impl SovereignManifold {
    pub fn new(nodes: &[[f64; 2]]) -> Self {
        Self {
            nodes: nodes.to_vec(),
        }
    }

    pub fn compute_tension_matrix(&self) -> TensionMatrix {
        self.compute_sparse_tension_matrix()
    }

    pub fn compute_sparse_tension_matrix(&self) -> TensionMatrix {
        let n = self.nodes.len();
        let mut matrix = vec![vec![0.0; n]; n];

        (0..n).into_par_iter().for_each(|i| {
            let mut neighbors: Vec<(usize, f64)> = (0..n)
                .filter(|&j| i != j)
                .map(|j| (j, self.euclidean_dist(self.nodes[i], self.nodes[j])))
                .collect();

            let k = if n > 1000 { 20 } else { 50 };
            let target_k = k.min(neighbors.len());

            if target_k > 0 {
                neighbors.select_nth_unstable_by(target_k - 1, |a, b| {
                    a.1.partial_cmp(&b.1).unwrap()
                });
                for &(j, dist) in neighbors.iter().take(target_k) {
                    if dist > 1e-9 {
                        matrix[i][j] = 1.0 / dist;
                    }
                }
            }
        });

        TensionMatrix::new(matrix)
    }

    pub fn compute_gradient_collapse(&self, nodes: &[[f64; 2]]) -> Vec<[f64; 2]> {
        nodes
            .par_iter()
            .map(|&node| self.apply_local_manifold_pressure(node))
            .collect()
    }

    fn apply_local_manifold_pressure(&self, node: [f64; 2]) -> [f64; 2] {
        let mut force = [0.0, 0.0];
        let n = self.nodes.len();

        let mut loc: Vec<(usize, f64)> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, &other)| other != node)
            .map(|(idx, &other)| {
                (
                    idx,
                    (other[0] - node[0]).powi(2) + (other[1] - node[1]).powi(2),
                )
            })
            .collect();

        let k = if n > 1000 { 20 } else { 50 };
        let k = k.min(loc.len());

        if k > 0 {
            loc.select_nth_unstable_by(k - 1, |a, b| a.1.partial_cmp(&b.1).unwrap());
            for &(idx, dist_sq) in loc.iter().take(k) {
                let other = self.nodes[idx];
                if dist_sq > 1e-9 {
                    let dist = dist_sq.sqrt();
                    let f = 1.0 / dist_sq;
                    force[0] += f * (other[0] - node[0]) / dist;
                    force[1] += f * (other[1] - node[1]) / dist;
                }
            }
        }
        force
    }

    fn euclidean_dist(&self, a: [f64; 2], b: [f64; 2]) -> f64 {
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
    }
}
