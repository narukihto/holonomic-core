use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct TensionMatrix {
    pub data: Vec<Vec<f64>>,
    pub size: usize,
}

impl TensionMatrix {
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        let size = matrix.len();
        Self { data: matrix, size }
    }

    pub fn enforce_terminal_boundary(&mut self, _t_max: f64) {
        let decay_factor = (-10.0f64).exp();
        self.data.par_iter_mut().for_each(|row| {
            for val in row.iter_mut() {
                if *val != 0.0 {
                    *val *= decay_factor;
                }
            }
        });
    }

    pub fn apply_asymmetric_bias(&mut self, i: usize, j: usize, bias: f64) {
        if i < self.size && j < self.data[i].len() {
            self.data[i][j] *= bias;
        }
    }
}
