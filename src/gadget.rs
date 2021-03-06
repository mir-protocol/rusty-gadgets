#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::constraint::Constraint;
use crate::field::Field;
use crate::wire_values::WireValues;
use crate::witness_generator::WitnessGenerator;

/// An R1CS gadget.
pub struct Gadget<F: Field> {
    /// The set of rank-1 constraints which define the R1CS instance.
    pub constraints: Vec<Constraint<F>>,
    /// The set of generators used to generate a complete witness from inputs.
    pub witness_generators: Vec<WitnessGenerator<F>>,
}

impl<F: Field> Gadget<F> {
    /// The number of constraints in this gadget.
    pub fn size(&self) -> usize {
        self.constraints.len()
    }

    /// Execute the gadget, and return whether all constraints were satisfied.
    pub fn execute(&self, wire_values: &mut WireValues<F>) -> bool {
        let mut pending_generators: Vec<&WitnessGenerator<F>> = self.witness_generators.iter().collect();

        // TODO: This repeatedly enumerates all generators, whether or not any of their dependencies
        // have been generated. A better approach would be to create a map from wires to generators
        // which depend on those wires. Then when a wire is assigned a value, we could efficiently
        // check for generators which are now ready to run, and place them in a queue.
        loop {
            let mut made_progress = false;
            pending_generators.retain(|generator| {
                if wire_values.contains_all(generator.inputs()) {
                    generator.generate(wire_values);
                    made_progress = true;
                    false
                } else {
                    true
                }
            });

            if !made_progress {
                break;
            }
        }

        assert_eq!(pending_generators.len(), 0, "Some generators never received inputs");

        self.constraints.iter().all(|constraint| constraint.evaluate(wire_values))
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::Expression;
    use crate::gadget_builder::GadgetBuilder;
    use crate::test_util::F257;
    use crate::wire_values::WireValues;

    #[test]
    fn constraint_not_satisfied() {
        let mut builder = GadgetBuilder::<F257>::new();
        let (x, y) = (builder.wire(), builder.wire());
        builder.assert_equal(&Expression::from(x), &Expression::from(y));
        let gadget = builder.build();

        let mut values = values!(x => 42u8.into(), y => 43u8.into());
        let constraints_satisfied = gadget.execute(&mut values);
        assert!(!constraints_satisfied);
    }

    #[test]
    #[should_panic]
    fn missing_generator() {
        let mut builder = GadgetBuilder::<F257>::new();
        let (x, y, z) = (builder.wire(), builder.wire(), builder.wire());
        builder.assert_product(&Expression::from(x), &Expression::from(y), &Expression::from(z));
        let gadget = builder.build();

        let mut values = values!(x => 2u8.into(), y => 3u8.into());
        gadget.execute(&mut values);
    }

    #[test]
    #[should_panic]
    fn missing_input() {
        let mut builder = GadgetBuilder::<F257>::new();
        let x = builder.wire();
        builder.inverse(&Expression::from(x));
        let gadget = builder.build();

        let mut values = WireValues::new();
        gadget.execute(&mut values);
    }
}
