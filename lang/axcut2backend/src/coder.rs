use crate::{
    code::Instructions, config::Config, fresh_labels::set_counter, memory::Memory,
    parallel_moves::ParallelMoves, statements::CodeStatement, utils::Utils,
};
use axcut::syntax::{Name, Prog};

use std::hash::Hash;

const INSTRUCTION_CAPACITY_PER_LABEL: usize = 10000;

fn translate<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    program: Prog,
    backend: &Backend,
) -> Vec<Vec<Code>>
where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    let mut instructions = Vec::with_capacity(program.defs.len());
    for def in program.defs {
        let mut is = Vec::with_capacity(INSTRUCTION_CAPACITY_PER_LABEL);
        def.body
            .code_statement(&program.types, def.context, backend, &mut is);
        is.shrink_to_fit();
        instructions.push(is);
    }
    instructions
}

fn assemble<Backend, Code, Temporary, Immediate>(
    instructions: Vec<Vec<Code>>,
    names: Vec<Name>,
    backend: &Backend,
) -> Vec<Code>
where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    let mut flattened_instructions =
        Vec::with_capacity(instructions.len() + instructions.iter().map(Vec::len).sum::<usize>());
    for (mut is, name) in instructions.into_iter().zip(names) {
        flattened_instructions.push(backend.label(name));
        flattened_instructions.append(&mut is);
    }
    flattened_instructions
}

#[must_use]
pub fn compile<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    program: Prog,
    backend: &Backend,
) -> (Vec<Code>, usize)
where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    let names: Vec<Name> = program.defs.iter().map(|def| def.name.clone()).collect();

    // skip first labels to adhere to Idris implementation
    set_counter(names.len() - 1);

    let number_of_arguments = program.defs[0].context.len();
    (
        assemble(translate(program, backend), names, backend),
        number_of_arguments,
    )
}
