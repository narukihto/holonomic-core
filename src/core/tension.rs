//! # Tension Matrix Engine
//! This module defines the stress relationships within the ARK manifold.

use rug::ops::Pow;
use rug::Float;

pub struct TensionMatrix {
    pub data: Vec<Vec<Float>>,
    pub size: usize,
}

impl TensionMatrix {
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        let size = matrix.len();
        let mut data = vec![vec![Float::with_val(128, 0.0); size]; size];
        for i in 0..size {
            for j in 0..size {
                data[i][j] = Float::with_val(128, matrix[i][j]);
            }
        }
        let mut t_matrix = Self { data, size };
        t_matrix.apply_topological_resistance();
        t_matrix.apply_convex_stabilization();
        t_matrix
    }

    fn apply_topological_resistance(&mut self) {
        let sigma = Float::with_val(128, 1.0);
        let two = Float::with_val(128, 2.0);
        for i in 0..self.size {
            for j in 0..self.size {
                if i != j {
                    let dist = &self.data[i][j];
                    let dist_sq = dist.clone().pow(2);
                    let sigma_sq = sigma.clone().pow(2);
                    let denom = &two * &sigma_sq;
                    let mut exponent = dist_sq.clone();
                    exponent /= &denom;
                    exponent = -exponent;
                    self.data[i][j] = exponent.exp();
                } else {
                    self.data[i][j] = Float::with_val(128, 0.0);
                }
            }
        }
    }

    pub fn apply_convex_stabilization(&mut self) {
        let lambda = Float::with_val(128, 1e-15);
        for i in 0..self.size {
            self.data[i][i] += &lambda;
        }
    }
}
