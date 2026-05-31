use ark_penta_v_core::core::tension::TensionMatrix;
use ark_penta_v_core::crypto::lwe::sign_manifold_async;
use std::sync::Arc;
use tokio::sync::mpsc;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let raw_data = vec![vec![1.0, 0.5], vec![0.5, 1.0]];
        let matrix = Arc::new(TensionMatrix::new(raw_data));
        let (tx, mut rx) = mpsc::channel(1);

        let matrix_clone = Arc::clone(&matrix);
        tokio::spawn(async move {
            sign_manifold_async(&matrix_clone, tx).await;
        });

        if let Some(signature) = rx.recv().await {
            if signature.is_valid {
                println!("LWE Compliance: VERIFIED");
            } else {
                std::process::exit(1);
            }
        }
    });
}
