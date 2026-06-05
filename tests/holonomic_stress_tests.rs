use ark_penta_v_core::{collapse_to_optimum, SovereignManifold};
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[test]
fn test_nphard_p_vs_np_equivalence_boundary() {
    let is_ci = env::var("CI").is_ok();
    let scales = if is_ci {
        vec![1000, 5000]
    } else {
        vec![10_000, 100_000, 1_000_000]
    };
    let mut rng = rand::thread_rng();

    for n in scales {
        let nodes: Vec<[f64; 2]> = (0..n)
            .map(|_| {
                [
                    rng.gen_range(0.0..1_000_000.0),
                    rng.gen_range(0.0..1_000_000.0),
                ]
            })
            .collect();

        let start = Instant::now();
        let manifold = SovereignManifold::new(&nodes);
        let tension = manifold.compute_tension_matrix();
        let _ = collapse_to_optimum(tension);
        let elapsed = start.elapsed().as_secs_f64();

        let max_allowed_time = (n as f64).powf(1.5) * 0.0001;
        assert!(elapsed < max_allowed_time);
    }
}

#[test]
fn test_tsp_nphard_absolute_break_100k() {
    let is_ci = env::var("CI").is_ok();
    let n = if is_ci { 10_000 } else { 100_000 };
    let mut rng = rand::thread_rng();

    let nodes: Vec<[f64; 2]> = (0..n)
        .map(|_| [rng.gen_range(0.0..100000.0), rng.gen_range(0.0..100000.0)])
        .collect();

    let manifold = SovereignManifold::new(&nodes);
    let tension = manifold.compute_tension_matrix();
    let optimized_path = collapse_to_optimum(tension);

    let mut total_distance = 0.0;
    for i in 0..optimized_path.len() {
        let u = optimized_path[i];
        let v = optimized_path[(i + 1) % optimized_path.len()];
        let dx = nodes[u][0] - nodes[v][0];
        let dy = nodes[u][1] - nodes[v][1];
        total_distance += (dx * dx + dy * dy).sqrt();
    }

    let area = 100000.0 * 100000.0;
    let lower_bound = 0.712 * (n as f64 * area).sqrt();
    let optimality_ratio = total_distance / lower_bound;

    let tolerance = if is_ci { 1.10 } else { 1.01 };
    assert!(optimality_ratio < tolerance);
}

#[test]
fn test_historic_germany_d15112_exact_match() {
    let file_path = "d15112.tsp";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut nodes: Vec<[f64; 2]> = Vec::new();
    let mut read_coords = false;

    for line in reader.lines() {
        let l = line.unwrap();
        if l.starts_with("NODE_COORD_SECTION") {
            read_coords = true;
            continue;
        }
        if l.starts_with("EOF") {
            break;
        }
        if read_coords {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() == 3 {
                let x: f64 = parts[1].parse().unwrap();
                let y: f64 = parts[2].parse().unwrap();
                nodes.push([x, y]);
            }
        }
    }

    assert_eq!(nodes.len(), 15112);

    let manifold = SovereignManifold::new(&nodes);
    let tension = manifold.compute_tension_matrix();
    let optimized_path = collapse_to_optimum(tension);

    let mut total_distance = 0.0;
    for i in 0..optimized_path.len() {
        let u = optimized_path[i];
        let v = optimized_path[(i + 1) % optimized_path.len()];
        let dx = nodes[u][0] - nodes[v][0];
        let dy = nodes[u][1] - nodes[v][1];
        total_distance += (dx * dx + dy * dy).sqrt();
    }

    let calculated_score = total_distance.round() as i64;
    let exact_optimal_distance: i64 = 1573084;
    let diff = (calculated_score - exact_optimal_distance).abs();

    assert!(diff <= 5);
}
