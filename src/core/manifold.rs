use crate::core::tension::TensionMatrix;

pub struct SovereignManifold {
    pub nodes: Vec<[f64; 2]>,
    curvature_alpha: f64,
}

impl SovereignManifold {
    pub fn new<const N: usize>(nodes: &[[f64; 2]; N]) -> Self {
        Self {
            nodes: nodes.to_vec(),
            curvature_alpha: 1.5,
        }
    }

    pub fn compute_tension_matrix(&self) -> TensionMatrix {
        let n = self.nodes.len();
        let mut matrix = vec![vec![0.0; n]; n];

        for (i, node_i) in self.nodes.iter().enumerate() {
            for (j, node_j) in self.nodes.iter().enumerate() {
                if i != j {
                    let dist = self.euclidean_dist(*node_i, *node_j);
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
