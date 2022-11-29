use winterfell::{
    math::{fields::f128::BaseElement, FieldElement},
    FieldExtension, HashFunction, ProofOptions, Prover, StarkProof, Trace, TraceTable,
};

mod air;
use air::FibAir;

// Fill the execution trace into a trace table
pub fn build_do_work_trace(n: usize) -> TraceTable<BaseElement> {}

// Our prover needs to hold STARK protocol parameters which are specified via ProofOptions
// struct.
struct AirProver {
    options: ProofOptions,
}

impl AirProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }
}

// When implementing Prover trait we set the `Air` associated type to the AIR of the
// computation we defined previously, and set the `Trace` associated type to `TraceTable`
// struct as we don't need to define a custom trace for our computation.
impl Prover for AirProver {
    type BaseField = BaseElement;
    type Air = FibAir;
    type Trace = TraceTable<Self::BaseField>;

    // Get the public inputs
    fn get_pub_inputs(&self, trace: &Self::Trace) -> BaseElement {}

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}

pub fn prove_work() -> (BaseElement, StarkProof) {
    // Hardcoded base parameters for this example
    let n = 1_048_576;

    // Define proof options; these will be enough for ~96-bit security level.
    let options = ProofOptions::new(
        32, // number of queries
        8,  // blowup factor
        0,  // grinding factor
        HashFunction::Blake3_256,
        FieldExtension::None,
        8,   // FRI folding factor
        128, // FRI max remainder length
    );
}

pub fn verify_work(result: BaseElement, proof: StarkProof) {}
