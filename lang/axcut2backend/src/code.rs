use axcut::syntax::{ContextBinding, Name};

pub trait Instructions<Code, Temporary, Immediate> {
    fn comment(msg: String) -> Code;
    fn label(name: Name) -> Code;
    fn jump(temporary: Temporary, instructions: &mut Vec<Code>);
    fn jump_label(name: Name, instructions: &mut Vec<Code>);
    fn jump_label_fixed(name: Name, instructions: &mut Vec<Code>);
    fn jump_label_if_equal(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    );
    fn jump_label_if_less(fst: Temporary, snd: Temporary, name: Name, instructions: &mut Vec<Code>);
    fn jump_label_if_zero(temporary: Temporary, name: Name, instructions: &mut Vec<Code>);
    fn load_immediate(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>);
    fn load_label(temporary: Temporary, name: Name, instructions: &mut Vec<Code>);
    fn add_and_jump(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>);
    fn add(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    fn sub(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    fn mul(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    fn div(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    fn rem(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    fn mov(target_temporary: Temporary, source_temporary: Temporary, instructions: &mut Vec<Code>);
    fn println_i64(
        source_temporary: Temporary,
        context: &[ContextBinding],
        instructions: &mut Vec<Code>,
    );
}
