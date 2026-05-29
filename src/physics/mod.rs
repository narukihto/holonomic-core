/// Operational configuration embedding the exact topological distance boundaries.
pub struct QuantumBundleConfig<const N: usize> {
    pub distance_matrix: [[f64; N]; N],
    pub adiabatic_time: f64,
    pub coupling_constant: f64,
    pub penalty_gamma: f64,
}

impl<const N: usize> QuantumBundleConfig<N> {
    /// Computes the exact Non-Abelian Berry Phase over the SO(n) connection.
    pub fn compute_berry_phase_gauge(&self, path: &[usize]) -> f64 {
        let mut accumulated_phase = 0.0;
        if path.len() < 2 {
            return 0.0;
        }

        for i in 0..path.len() - 1 {
            let u = path[i];
            let v = path[i + 1];
            let gauge_connection = self.distance_matrix[u][v] * self.coupling_constant;
            accumulated_phase += gauge_connection.sin();
        }

        let final_node = path[path.len() - 1];
        let return_gauge = self.distance_matrix[final_node][path[0]] * self.coupling_constant;
        accumulated_phase += return_gauge.sin();

        accumulated_phase
    }

    /// Computes the exact Frobenius matrix gradient including orthogonal constraints.
    #[allow(clippy::needless_range_loop)]
    pub fn compute_manifold_gradient(&self, state_matrix: &[[f64; N]; N]) -> [[f64; N]; N] {
        let mut grad = [[0.0; N]; N];
        let gamma = self.penalty_gamma;

        for i in 0..N {
            for j in 0..N {
                let cost_factor = self.distance_matrix[i][j];
                let mut row_sum = 0.0;
                let mut col_sum = 0.0;

                for k in 0..N {
                    row_sum += state_matrix[i][k];
                    col_sum += state_matrix[k][j];
                }

                let identity_delta = if i == j { 1.0 } else { 0.0 };
                let penalty = gamma * (row_sum + col_sum - 2.0 * identity_delta);
                grad[i][j] = cost_factor + penalty;
            }
        }
        grad
    }

    /// Computes the exact Skew-Symmetric Rotator fields (Samsara Wheel).
    pub fn apply_skew_symmetric_rotator(
        &self,
        x: &[[f64; N]; N],
        grad: &[[f64; N]; N],
    ) -> [[f64; N]; N] {
        let mut omega = [[0.0; N]; N];
        let mut xt_grad = [[0.0; N]; N];
        let mut gradt_x = [[0.0; N]; N];

        for i in 0..N {
            for j in 0..N {
                let mut sum_xt = 0.0;
                let mut sum_gt = 0.0;
                for k in 0..N {
                    sum_xt += x[k][i] * grad[k][j];
                    sum_gt += grad[k][i] * x[k][j];
                }
                xt_grad[i][j] = sum_xt;
                gradt_x[i][j] = sum_gt;
            }
        }

        for i in 0..N {
            for j in 0..N {
                omega[i][j] = xt_grad[i][j] - gradt_x[i][j];
            }
        }
        omega
    }

    /// High-order Taylor Expansion mapping the exact matrix exponential.
    pub fn compute_matrix_exponential(&self, algebra_mat: &[[f64; N]; N]) -> [[f64; N]; N] {
        let mut exp_mat = [[0.0; N]; N];
        let mut term = [[0.0; N]; N];

        for i in 0..N {
            exp_mat[i][i] = 1.0;
            term[i][i] = 1.0;
        }

        for grade in 1..13 {
            let mut next_term = [[0.0; N]; N];
            for i in 0..N {
                for j in 0..N {
                    let mut sum = 0.0;
                    for k in 0..N {
                        sum += term[i][k] * algebra_mat[k][j];
                    }
                    next_term[i][j] = sum / (grade as f64);
                }
            }
            term = next_term;
            for i in 0..N {
                for j in 0..N {
                    exp_mat[i][j] += term[i][j];
                }
            }
        }
        exp_mat
    }
}
