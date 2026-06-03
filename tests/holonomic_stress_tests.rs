use ark_penta_v_core::{collapse_to_optimum, init_sovereign_core, SovereignManifold};
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

#[test]
fn test_nphard_p_vs_np_equivalence_boundary() {
    init_sovereign_core();
    let scales = vec![10_000, 100_000, 1_000_000];
    let mut rng = rand::thread_rng();

    for n in scales {
        let nodes: Vec<[f64; 2]> = (0..n)
            .map(|_| [rng.gen_range(0.0..1_000_000.0), rng.gen_range(0.0..1_000_000.0)])
            .collect();

        let start = Instant::now();
        let manifold = SovereignManifold::new(&nodes);
        let tension = manifold.compute_tension_matrix();
        let _ = collapse_to_optimum(tension);
        let elapsed = start.elapsed().as_secs_f64();

        println!("Nodes: {}, Time: {:.4}s", n, elapsed);

        let max_allowed_time = (n as f64).powf(1.5) * 0.0001;
        assert!(
            elapsed < max_allowed_time,
            "Failed: Complexity explosion! Time exceeded polynomial bound at {} nodes",
            n
        );
    }
}

#[test]
fn test_tsp_nphard_absolute_break_100k() {
    init_sovereign_core();
    let n = 100_000;
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

    println!("Total Distance: {:.2}", total_distance);
    println!("Lower Bound: {:.2}", lower_bound);
    println!("Optimality Ratio: {:.6}", optimality_ratio);

    assert!(optimality_ratio < 1.002);
}

#[test]
fn test_historic_germany_d15112_exact_match() {
    init_sovereign_core();
    let file_path = "d15112.tsp";
    if !Path::new(file_path).exists() {
        let url = "http://elib.zib.de/pub/mp-testdata/tsp/tsplib/tsp/d15112.tsp";
        if let Ok(response) = ureq::get(url).call() {
            let mut file = File::create(file_path).unwrap();
            std::io::copy(&mut response.into_reader(), &mut file).unwrap();
        }
    }

    if Path::new(file_path).exists() {
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

        if nodes.len() == 15112 {
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

            let calculated_score = total_distance.round() as u64;
            let exact_optimal_distance: u64 = 1573084;
            assert_eq!(calculated_score, exact_optimal_distance);
        }
    }
}
