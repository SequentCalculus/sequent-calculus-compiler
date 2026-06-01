//! Core typechecking infrastructure.

use printer::{Print, PrintCfg};

use crate::{mono::constraints::ConstraintCollector, syntax::program::Prog};
pub mod constraints;
pub mod errors;

pub fn monomorphize_program(program: Prog) {
    let constraints = program
        .collect_constraints(&program.data_types, &program.codata_types)
        .unwrap();

    let forced_set_cfg = PrintCfg {
        width: 0,
        allow_linebreaks: true,
        indent: 4,
        ..PrintCfg::default()
    };

    println!(
        "{}",
        constraints.print_to_colored_string(Some(&forced_set_cfg))
    );
}
