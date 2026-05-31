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
        for (i, row) in matrix.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                data[i][j] = Float::with_val(128, val);
            }
        }
        Self { data, size }
    }

    pub fn enforce_terminal_boundary(&mut self, t_max: f64) {
        let decay_factor = Float::with_val(128, (-10.0f64).exp());
        for i in 0..self.size {
            for j in 0..self.size {
                self.data[i][j] *= &decay_factor;
            }
        }
    }
}
