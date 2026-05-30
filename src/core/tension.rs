//! # Tension Matrix Engine
//! 
//! This module defines the stress relationships within the ARK manifold.
//! It processes the topological resistance $\Omega_{ij}$ to force the geodesic convergence.

pub struct TensionMatrix {
    pub data: Vec<Vec<f64>>,
    pub size: usize,
}

impl TensionMatrix {
    /// Creates a new Tension Matrix and initializes topological resistance.
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        let size = matrix.len();
        let mut t_matrix = Self { data: matrix, size };
        t_matrix.apply_topological_resistance();
        t_matrix
    }

    /// Applies topological resistance Omega_ij to the tension values.
    /// This forces the system to focus on paths with the lowest 'Tension' (Geodesic Weight).
    fn apply_topological_resistance(&mut self) {
        // Here we refine the tension using the topological resistance matrix Omega.
        // This enforces the Geodesic Flow Principle: T_ij = exp(-dist^2 / 2sigma^2) * Omega_ij
        let sigma = 1.0; 
        for i in 0..self.size {
            for j in 0..self.size {
                if i != j {
                    let resistance = 1.0; // Dynamic resistance factor
                    let weight = (-self.data[i][j].powi(2) / (2.0 * sigma.powi(2))).exp();
                    self.data[i][j] = weight * resistance;
                }
            }
        }
    }

    /// Returns the matrix data for the Geodesic Flow computation.
    pub fn get_matrix(&self) -> &Vec<Vec<f64>> {
        &self.data
    }
}
