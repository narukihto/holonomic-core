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

    pub fn compute_tension_matrix(&self) -> TensionMatrix {
        let n = self.nodes.len();
        let matrix: Vec<Vec<f64>> = (0..n)
            .into_par_iter()
            .map(|i| {
                (0..n)
                    .map(|j| {
                        if i != j {
                            let dist = self.euclidean_dist(self.nodes[i], self.nodes[j]);
                            1.0 / dist
                        } else {
                            0.0
                        }
                    })
                    .collect()
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
        for &other in &self.nodes {
            if node != other {
                let dx = other[0] - node[0];
                let dy = other[1] - node[1];
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt();
                if dist > 0.0 {
                    let f = 1.0 / dist_sq;
                    force[0] += f * dx / dist;
                    force[1] += f * dy / dist;
                }
            }
        }
        force
    }

    fn euclidean_dist(&self, a: [f64; 2], b: [f64; 2]) -> f64 {
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
    }
}
