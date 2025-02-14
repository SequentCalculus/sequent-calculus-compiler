use crate::{
    code::Instructions, config::Config, memory::Memory, parallel_moves::ParallelMoves,
    statements::CodeStatement, utils::Utils,
};
use axcut::syntax::{Name, Prog};

use printer::{DocAllocator, Print};

use std::hash::Hash;

const INSTRUCTION_CAPACITY_PER_LABEL: usize = 10000;

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

fn assemble<Backend, Code, Temporary, Immediate>(
    instructions: Vec<Vec<Code>>,
    names: Vec<Name>,
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
        flattened_instructions.push(Backend::label(name));
        flattened_instructions.append(&mut is);
    }
    flattened_instructions
}

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
    let names: Vec<Name> = program.defs.iter().map(|def| def.name.clone()).collect();

    let number_of_arguments = program.defs[0].context.bindings.len();
    AssemblyProg {
        instructions: assemble::<Backend, _, _, _>(translate::<Backend, _, _, _>(program), names),
        number_of_arguments,
    }
}
