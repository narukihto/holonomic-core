use ark_penta_v_core::crypto::lwe::sign_manifold;
use ark_penta_v_core::core::tension::TensionMatrix;

fn main() {
    let raw_data = vec![vec![1.0, 0.5], vec![0.5, 1.0]];
    let matrix = TensionMatrix::new(raw_data);

    let signature = sign_manifold(&matrix);

    if signature.is_valid() {
        println!("LWE Compliance: VERIFIED");
    } else {
        std::process::exit(1);
    }
}
