//! # Space Folding Engine
//! This module defines the Riemannian manifold $\mathcal{M}_{ARK}$.
//! It transforms discrete spatial nodes into a continuous potential field.

use crate::core::tension::TensionMatrix;

pub struct SovereignManifold {
    pub nodes: Vec<[f64; 2]>,
    curvature_alpha: f64,
}

impl SovereignManifold {
    /// Creates a new Manifold with a default curvature alpha.
    pub fn new<const N: usize>(nodes: &[[f64; 2]; N]) -> Self {
        Self {
            nodes: nodes.to_vec(),
            curvature_alpha: 1.5, // Default manifold curvature coefficient
        }
    }

    /// Calculates the Tension Matrix T_ij based on potential energy mapping.
    /// This method performs the 'Space Folding' by reducing the distance between nodes
    /// into a stress-potential relationship.
    pub fn compute_tension_matrix(&self) -> TensionMatrix {
        let n = self.nodes.len();
        let mut matrix = vec![vec![0.0; n]; n];

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let dist = self.euclidean_dist(self.nodes[i], self.nodes[j]);
                    // Space Folding Formula: Psi(x) = sum(1 / ||x - pi||^alpha)
                    // The tension is the inverse of the potential field density.
                    matrix[i][j] = (1.0 / dist).powf(self.curvature_alpha);
                }
            }
        }
        TensionMatrix::new(matrix)
    }

    fn euclidean_dist(&self, a: [f64; 2], b: [f64; 2]) -> f64 {
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
    }
}
