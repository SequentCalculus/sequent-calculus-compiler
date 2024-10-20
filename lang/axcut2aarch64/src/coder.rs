use super::code::Code;
use super::fresh_labels::set_counter;
use super::statements::CodeStatement;
use axcut::syntax::{Name, Prog};

const INSTRUCTION_CAPACITY_PER_LABEL: usize = 10000;

fn translate(program: Prog) -> Vec<Vec<Code>> {
    let mut instructions = Vec::with_capacity(program.defs.len());
    for def in program.defs {
        let mut is = Vec::with_capacity(INSTRUCTION_CAPACITY_PER_LABEL);
        def.body
            .code_statement(&program.types, def.context, &mut is);
        is.shrink_to_fit();
        instructions.push(is);
    }
    instructions
}

fn assemble(instructions: Vec<Vec<Code>>, names: Vec<Name>) -> Vec<Code> {
    let mut flattened_instructions =
        Vec::with_capacity(instructions.len() + instructions.iter().map(Vec::len).sum::<usize>());
    for (mut is, name) in instructions.into_iter().zip(names) {
        flattened_instructions.push(Code::LAB(name));
        flattened_instructions.append(&mut is);
    }
    flattened_instructions
}

#[must_use]
pub fn compile(program: Prog) -> (Vec<Code>, usize) {
    let names: Vec<Name> = program.defs.iter().map(|def| def.name.clone()).collect();

    // skip first labels to adhere to Idris implementation
    set_counter(names.len() - 1);

    let number_of_arguments = program.defs[0].context.len();
    (assemble(translate(program), names), number_of_arguments)
}
