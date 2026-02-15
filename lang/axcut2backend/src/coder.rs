//! This module contains the basic functions for generating machine code for an [AxCut](axcut)
//! program.

use crate::{
    code::Instructions, config::Config, memory::Memory, parallel_moves::ParallelMoves,
    statements::CodeStatement, utils::Utils,
};
use axcut::syntax::{Ident, Prog};

use printer::{DocAllocator, Print};

use std::hash::Hash;

const INSTRUCTION_CAPACITY_PER_LABEL: usize = 10000;

/// This function translates each top-level definition of an [AxCut](axcut) program to assembly
/// code, returning a list of these assembly instruction blocks.
/// - `program` is the [AxCut](axcut) program to translate.
fn translate<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    program: Prog,
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
            .code_statement::<Backend, _, _, _>(&program.types, def.context, &mut is);
        is.shrink_to_fit();
        instructions.push(is);
    }
    instructions
}

/// This function flattens a list of assembly instruction blocks, endowing each block with its
/// label.
/// - `instructions` is the list of assembly blocks
/// - `names` is the list of labels
fn assemble<Backend, Code, Temporary, Immediate>(
    instructions: Vec<Vec<Code>>,
    names: Vec<Ident>,
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
        flattened_instructions.push(Backend::label(name + "_"));
        flattened_instructions.append(&mut is);
    }
    flattened_instructions
}

/// An assembly program together with the number of command-line arguments it takes.
pub struct AssemblyProg<Code> {
    pub instructions: Vec<Code>,
    pub number_of_arguments: usize,
}

impl<Code: Print> Print for AssemblyProg<Code> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let instructions = self
            .instructions
            .iter()
            .map(|instruction| instruction.print(cfg, alloc));
        alloc.intersperse(instructions, alloc.line())
    }
}

/// This function compiles an [AxCut](axcut) program to assembly code, also calculating the number
/// of command-line arguments. The type parameters are to be instantiated with the implementations
/// of the corresponding traits of the respective `Backend` platform.
/// - `program` is the [AxCut](axcut) program to compile.
///
/// # Panics
///
/// A panic is caused if the `program` contains too many live variables at some point, so that we
/// run out of temporaries.
pub fn compile<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    program: Prog,
) -> AssemblyProg<Code>
where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    let names: Vec<Ident> = program.defs.iter().map(|def| def.name.clone()).collect();

    let number_of_arguments = program.defs[0].context.bindings.len();
    AssemblyProg {
        instructions: assemble::<Backend, _, _, _>(translate::<Backend, _, _, _>(program), names),
        number_of_arguments,
    }
}
