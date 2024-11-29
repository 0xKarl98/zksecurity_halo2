mod circuit;
use circuit::
{
    TestCircuit,
    PhantomData,
    MockProver
};
use halo2curves::bn256::Fr;

fn main() {

    let circuit = TestCircuit::<Fr>::new();
    
    let prover = MockProver::run(8, &circuit, vec![]).unwrap();

    assert!(prover.verify().is_ok());
}