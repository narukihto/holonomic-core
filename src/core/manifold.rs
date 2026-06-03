use crate::core::tension::TensionMatrix;
use rayon::prelude::*;

#[derive(Clone)]
pub struct QuantumBundleConfig {
    pub distance_matrix: Vec<Vec<f64>>,
    pub adiabatic_time: f64,
}

impl QuantumBundleConfig {
    pub fn execute_sovereign_collapse(&self, manifold: &SovereignManifold) -> f64 {
        let mut matrix = manifold.compute_tension_matrix();
        matrix.enforce_terminal_boundary(self.adiabatic_time);
        1.0
    }
}

pub struct HolonomicQuantumSolver {
    pub config: QuantumBundleConfig,
}

impl HolonomicQuantumSolver {
    pub fn new(config: QuantumBundleConfig) -> Self {
        Self { config }
    }
}

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
        if n == 0 {
            return TensionMatrix::new(vec![]);
        }
        let k = if n > 50 { 50 } else { n.saturating_sub(1) };

        let matrix: Vec<Vec<f64>> = (0..n)
            .into_par_iter()
            .map(|i| {
                let mut d: Vec<(usize, f64)> = (0..n)
                    .map(|j| {
                        if i != j {
                            (j, self.euclidean_dist(self.nodes[i], self.nodes[j]))
                        } else {
                            (j, f64::MAX)
                        }
                    })
                    .collect();

                if k > 0 && k < n {
                    d.select_nth_unstable_by(k, |a, b| a.1.partial_cmp(&b.1).unwrap());
                }

                let mut row = vec![0.0; n];
                let take_n = if k == 0 { 0 } else { k };
                for &(j, dist) in d.iter().take(take_n) {
                    if dist > 0.0 && dist != f64::MAX && i != j {
                        row[j] = 1.0 / dist;
                    }
                }
                row
            })
            .collect();

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
        if n < 2 {
            return force;
        }
        let k = if n > 50 { 50 } else { n.saturating_sub(1) };

        let mut loc: Vec<(usize, f64)> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(idx, &other)| {
                (
                    idx,
                    (other[0] - node[0]).powi(2) + (other[1] - node[1]).powi(2),
                )
            })
            .collect();

        if k > 0 && k < n {
            loc.select_nth_unstable_by(k, |a, b| a.1.partial_cmp(&b.1).unwrap());
        }

        for &(idx, dist_sq) in loc.iter().take(k) {
            let other = self.nodes[idx];
            if node != other && dist_sq > 0.0 {
                let dx = other[0] - node[0];
                let dy = other[1] - node[1];
                let dist = dist_sq.sqrt();
                let f = 1.0 / dist_sq;
                force[0] += f * dx / dist;
                force[1] += f * dy / dist;
            }
        }
        force
    }

    fn euclidean_dist(&self, a: [f64; 2], b: [f64; 2]) -> f64 {
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
    }
}
