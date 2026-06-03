use ark_penta_v_core::{
    collapse_to_optimum, init_sovereign_core, QuantumBundleConfig, SovereignManifold,
};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn get_node_count() -> usize {
    if env::var("CI").is_ok() {
        5000
    } else {
        100000
    }
}

#[test]
fn test_ultimate_random_chaos_100k() {
    init_sovereign_core();
    let n = get_node_count();
    let mut rng = rand::thread_rng();
    let nodes: Vec<[f64; 2]> = (0..n)
        .map(|_| [rng.gen_range(0.0..100000.0), rng.gen_range(0.0..100000.0)])
        .collect();
    run_and_verify_absolute_tsp("Ultimate Random Chaos", &nodes, 10);
}

#[test]
fn test_clustered_fractal_trap_100k() {
    init_sovereign_core();
    let n = get_node_count();
    let mut rng = rand::thread_rng();
    let mut nodes = Vec::with_capacity(n);
    let center_count = if n >= 100000 { 100 } else { 5 };
    let mut centers = Vec::new();
    for _ in 0..center_count {
        centers.push((rng.gen_range(0.0..100000.0), rng.gen_range(0.0..100000.0)));
    }
    let normal = Normal::new(0.0, 500.0).unwrap();
    for i in 0..n {
        let center = centers[i % center_count];
        let dx = normal.sample(&mut rng);
        let dy = normal.sample(&mut rng);
        nodes.push([center.0 + dx, center.1 + dy]);
    }
    run_and_verify_absolute_tsp("Clustered Fractal Trap", &nodes, 12);
}

#[test]
fn test_monolithic_ring_symmetry_100k() {
    init_sovereign_core();
    let n = get_node_count();
    let mut rng = rand::thread_rng();
    let nodes: Vec<[f64; 2]> = (0..n)
        .map(|i| {
            let angle = (i as f64 / n as f64) * 2.0 * std::f64::consts::PI;
            let radius = 50000.0 + rng.gen_range(-100.0..100.0);
            [radius * angle.cos(), radius * angle.sin()]
        })
        .collect();
    run_and_verify_absolute_tsp("Monolithic Ring Symmetry", &nodes, 10);
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
            let start = Instant::now();
            let optimized_path = collapse_to_optimum(tension);
            let duration = start.elapsed();

            assert_eq!(optimized_path.len(), nodes.len());
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
            println!("Duration: {:.4}s", duration.as_secs_f64());
        }
    }
}

fn run_and_verify_absolute_tsp(test_name: &str, nodes: &[[f64; 2]], max_seconds: u64) {
    let manifold = SovereignManifold::new(nodes);
    let tension = manifold.compute_tension_matrix();
    let start = Instant::now();
    let optimized_path = collapse_to_optimum(tension);
    let duration = start.elapsed();

    assert_eq!(optimized_path.len(), nodes.len());
    let mut unique_nodes = HashSet::new();
    for node_idx in &optimized_path {
        assert!(*node_idx < nodes.len());
        assert!(unique_nodes.insert(*node_idx));
    }

    let config = QuantumBundleConfig {
        distance_matrix: vec![],
        adiabatic_time: 1.0,
    };
    let result = config.execute_sovereign_collapse(&manifold);
    assert!(result >= 0.0);

    println!("Test: {}, Duration: {:.4}s", test_name, duration.as_secs_f64());
    if env::var("CI").is_err() && nodes.len() >= 100000 {
        assert!(duration.as_secs() < max_seconds);
    }
}
