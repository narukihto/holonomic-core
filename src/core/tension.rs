use rayon::prelude::*;
use rug::Float;

#[derive(Clone, Debug)]
pub struct TensionMatrix {
    pub data: Vec<Vec<Float>>,
    pub size: usize,
}

impl TensionMatrix {
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        let size = matrix.len();
        let data: Vec<Vec<Float>> = matrix
            .into_par_iter()
            .map(|row| {
                row.into_iter()
                    .map(|val| Float::with_val(64, val))
                    .collect()
            })
            .collect();

        Self { data, size }
    }

    pub fn enforce_terminal_boundary(&mut self, _t_max: f64) {
        let decay_factor = Float::with_val(64, (-10.0f64).exp());
        self.data.par_iter_mut().for_each(|row| {
            for val in row.iter_mut() {
                if *val != 0.0 {
                    *val *= &decay_factor;
                }
            }
        });
    }

    pub fn apply_asymmetric_bias(&mut self, i: usize, j: usize, bias: f64) {
        if i < self.size && j < self.data[i].len() {
            self.data[i][j] *= Float::with_val(64, bias);
        }
    }
}
