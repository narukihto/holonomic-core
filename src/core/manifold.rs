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

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn compute_tension_matrix(&self) -> TensionMatrix {
        let n = self.nodes.len();
        let matrix: Vec<Vec<f64>> = (0..n)
            .into_par_iter()
            .map(|i| {
                let mut row = vec![0.0; n];
                row.iter_mut().enumerate().for_each(|(j, val)| {
                    if i != j {
                        *val = self.euclidean_dist(self.nodes[i], self.nodes[j]);
                    }
                });
                row
            })
            .collect();
        TensionMatrix::new(matrix)
    }

    fn euclidean_dist(&self, a: [f64; 2], b: [f64; 2]) -> f64 {
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
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

        let k = (n.min(60)).min(loc.len());
        if k > 0 {
            loc.select_nth_unstable_by(k - 1, |a, b| a.1.partial_cmp(&b.1).unwrap());
            for &(idx, dist_sq) in loc.iter().take(k) {
                let other = self.nodes[idx];
                let dist = dist_sq.sqrt().max(1e-9);
                let f = (1.0 / dist_sq.max(1e-9)).min(1e6);
                force[0] += f * (other[0] - node[0]) / dist;
                force[1] += f * (other[1] - node[1]) / dist;
            }
        }
        force
    }
}
