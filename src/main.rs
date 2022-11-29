mod prover;
use std::time::Instant;

use prover::{prove_work, verify_work};

fn main() {
    let now = Instant::now();
    let (result, proof) = prove_work();
    let elapsed = now.elapsed();
    println!("Computing fibonacci and proof took: {:.2?}", elapsed);

    let then = Instant::now();
    verify_work(result, proof);
    let fin = then.elapsed();
    println!("Proof verification took: {:.2?}", fin);
}
