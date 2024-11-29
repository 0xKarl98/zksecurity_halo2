pub use halo2_proofs::{
    circuit::{
        Layouter, 
        SimpleFloorPlanner, 
        AssignedCell, 
        Value
    },
    dev::MockProver,
    plonk::{
        self, 
        Circuit, 
        ConstraintSystem,
        Selector,
        Advice,
        Column
    },
    arithmetic::Field
};
use plonk::Error;
pub use std::marker::PhantomData;


#[derive(Clone, Debug)]
pub struct TestCircuit<F: Field + Clone>{
    _ph:PhantomData<F>,
}

#[derive(Clone, Debug)]
pub struct TestConfig<F: Field + Clone>{
    pub advice: Column<Advice>,
    pub q_mul: Selector,
    pub q_add: Selector,
    _ph: PhantomData<F>,
}


//This implements components in a general circuit 
//For circuit , it has multiplication gate and addition gate 
impl<F: Field> TestCircuit<F> {
    //Use this , so that we can create a new instance by using TestCircuit::new()
    pub fn new() -> Self {
        Self{_ph:PhantomData}
    }
    /// This region occupies 3 rows.
    fn mul(
        config: &<Self as Circuit<F>>::Config,
        layouter: &mut impl Layouter<F>,
        lhs: AssignedCell<F, F>, 
        rhs: AssignedCell<F, F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "mul",
            |mut region| {
                let v0 = lhs.value().cloned();
                let v1 = rhs.value().cloned();
                let v2 =
                    v0 //
                        .and_then(|v0| v1.and_then(|v1| Value::known(v0 * v1)));

                let w0 = region.assign_advice(
                    || "assign w0", //
                    config.advice,
                    0,
                    || v0,
                )?;

                let w1 = region.assign_advice(
                    || "assign w1", //
                    config.advice,
                    1,
                    || v1,
                )?;

                let w2 = region.assign_advice(
                    || "assign w2", //
                    config.advice,
                    2,
                    || v2,
                )?;

                // turn on the gate
                config.q_mul.enable(&mut region, 0)?;
                Ok(w2)
            },
        )
    }

    //assignment: impl add gate in the fancy of mul above
    fn add(
        config: &<Self as Circuit<F>>::Config,
        layouter: &mut impl Layouter<F>,
        lhs: AssignedCell<F, F>, //assign the constant and turn on the gate
        rhs: AssignedCell<F, F>, 
    ) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "add",
            |mut region| {
                let v0 = lhs.value().cloned();
                let v1 = rhs.value().cloned();
                let v2 = v0.and_then(|v0| v1.and_then(|v1| Value::known(v0 + v1)));
                //In halo2 circuit ,advice column contains witness values 
                //we can view it as flag 
                let w0 = region.assign_advice(
                    || "assign w0",
                    config.advice,
                    0,
                    || v0,
                )?;
                
                let w1 = region.assign_advice(
                    || "assign w1",
                    config.advice,
                    1,
                    || v1,
                )?;

                let w2 = region.assign_advice(
                    || "assign w2",
                    config.advice,
                    2,
                    || v2,
                )?;
                //after assign advice column , we need to enable the selector 
                config.q_add.enable(&mut region, 0)?;
                Ok(w2)
            },
        )
    }

}

//This implements the internal component for a circuit
//For circuit , the internal logic includes Configure and Synthesize 
//In the case of car , it performs as start() and stop()
//This is copied from : 
//https://halo2.zksecurity.xyz/halo-world/?highlight=Circuit%3CF%3E#how-to-do-nothing
impl<F: Field> Circuit<F> for TestCircuit<F> {
    type Config = TestConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        TestCircuit {_ph:PhantomData}
    }

    #[allow(unused_variables)]
    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        TestConfig { 
        advice: meta.advice_column(),
        q_mul: meta.selector(),
        q_add: meta.selector(),
        _ph: PhantomData }
    }

    #[allow(unused_variables)]
    fn synthesize(
        &self,
        config: Self::Config,
        layouter: impl Layouter<F>,
    ) -> Result<(), plonk::Error> {
        Ok(())
    }

}