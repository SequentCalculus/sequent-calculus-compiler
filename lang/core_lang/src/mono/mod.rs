//! Core typechecking infrastructure.

use crate::{mono::constraints::ConstraintCollector, syntax::program::Prog};
pub mod constraints;
pub mod errors;

pub fn monomorphize_program(program: Prog) {
    let constraints = program
        .collect_constraints(&program.data_types, &program.codata_types)
        .unwrap();

    dbg!(constraints);
}
