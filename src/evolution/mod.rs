use crate::observer::HolonomicSystemState;
use crate::physics::QuantumBundleConfig;

/// Core Hybrid Topological-Heuristic TSP Solver operating via Cosmic Gauge Guidance.
pub struct HolonomicQuantumSolver<const N: usize> {
    pub config: QuantumBundleConfig<N>,
    pub state: HolonomicSystemState,
}

impl<const N: usize> HolonomicQuantumSolver<N> {
    pub fn new(config: QuantumBundleConfig<N>) -> Self {
        Self {
            config,
            state: HolonomicSystemState::default(),
        }
    }

    /// Models the Adiabatic Transformation protecting the minimum spectral gap invariant.
    pub fn simulate_adiabatic_evolution(
        &self,
        s: f64,
        base_energy: f64,
        accumulated_holonomy: f64,
    ) -> f64 {
        let n_dimensional_factor = N as f64;
        let protected_spectral_gap = 1.0 / n_dimensional_factor.powf(2.0);

        let h_0 = (1.0 - s) * base_energy;
        let target_energy_field = base_energy + (accumulated_holonomy * 0.01);
        let h_tsp = s * (target_energy_field * protected_spectral_gap);

        h_0 + h_tsp
    }

    /// NEW: Cosmic Conceptual Collapse Engine (TACO Across Timelines)
    #[allow(clippy::needless_range_loop)]
    pub fn execute_topological_collapse(&self) -> f64 {
        let total_ants = 20;
        let mut global_best_cost = f64::INFINITY;

        let mut state_matrix = [[0.0; N]; N];
        for i in 0..N {
            state_matrix[i][i] = 1.0;
        }

        let grad = self.config.compute_manifold_gradient(&state_matrix);
        let omega = self
            .config
            .apply_skew_symmetric_rotator(&state_matrix, &grad);
        let propagator = self.config.compute_matrix_exponential(&omega);

        for ant in 0..total_ants {
            let root_node = ant % N;
            let mut system_path = vec![root_node];
            let mut unvisited = vec![true; N];
            unvisited[root_node] = false;

            let mut current_energy = 0.0;
            let mut evolution_step = 0.0;
            let step_delta = 1.0 / (N as f64);

            for _ in 1..N {
                let current_node = system_path[system_path.len() - 1];
                let gauge_factor = self.config.compute_berry_phase_gauge(&system_path);
                let adiabatic_blend = self.simulate_adiabatic_evolution(
                    evolution_step,
                    current_energy,
                    gauge_factor,
                );

                let mut best_next_node = usize::MAX;
                let mut maximum_field_potential = f64::MIN;

                for candidate in 0..N {
                    if unvisited[candidate] {
                        let distance = self.config.distance_matrix[current_node][candidate];
                        let heuristic_force = 1.0 / (distance + 1e-6);
                        let topological_force = propagator[current_node][candidate].abs();

                        let total_force = (heuristic_force * 0.4)
                            + (topological_force * 0.6 * adiabatic_blend);

                        if total_force > maximum_field_potential {
                            maximum_field_potential = total_force;
                            best_next_node = candidate;
                        }
                    }
                }

                if best_next_node == usize::MAX {
                    break;
                }

                unvisited[best_next_node] = false;
                current_energy += self.config.distance_matrix[current_node][best_next_node];
                system_path.push(best_next_node);
                step_delta_acc(&mut evolution_step, step_delta);
            }

            let final_node = system_path[system_path.len() - 1];
            let total_tour_cost =
                current_energy + self.config.distance_matrix[final_node][root_node];

            if total_tour_cost < global_best_cost {
                global_best_cost = total_tour_cost;
                let state_key = format!("AntPath_Best_{}", ant);
                self.state
                    .state_observer
                    .insert(state_key, global_best_cost);
            }
        }

        self.state
            .state_observer
            .insert("System_Ground_State".to_string(), global_best_cost);
        global_best_cost
    }
}

/// Helper function to encapsulate accumulation to avoid inline alignment clashing
#[inline]
fn step_delta_acc(evolution_step: &mut f64, step_delta: f64) {
    *evolution_step += step_delta;
}
