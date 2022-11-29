mod prover;
use prover::{prove_work, verify_work};

fn main() {
    let (result, proof) = prove_work();
    verify_work(result, proof);
}
