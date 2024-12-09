use axcut::syntax::Name;

pub trait Instructions<Code, Temporary, Immediate> {
    fn comment(&self, msg: String) -> Code;
    fn label(&self, name: Name) -> Code;
    fn jump(&self, temporary: Temporary, instructions: &mut Vec<Code>);
    fn jump_label(&self, name: Name, instructions: &mut Vec<Code>);
    fn jump_label_if_equal(
        &self,
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    );
    fn jump_label_if_less(
        &self,
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    );
    fn jump_label_if_zero(&self, temporary: Temporary, name: Name, instructions: &mut Vec<Code>);
    fn load_immediate(
        &self,
        temporary: Temporary,
        immediate: Immediate,
        instructions: &mut Vec<Code>,
    );
    fn load_label(&self, temporary: Temporary, name: Name, instructions: &mut Vec<Code>);
    fn add_and_jump(
        &self,
        temporary: Temporary,
        immediate: Immediate,
        instructions: &mut Vec<Code>,
    );
    fn add(
        &self,
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    fn sub(
        &self,
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    fn mul(
        &self,
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    fn div(
        &self,
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    fn rem(
        &self,
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    fn mov(
        &self,
        target_temporary: Temporary,
        source_temporary: Temporary,
        instructions: &mut Vec<Code>,
    );
}
