use axcut::syntax::TypingContext;

pub trait Memory<Code, Temporary> {
    fn erase_block(&self, to_erase: Temporary, instructions: &mut Vec<Code>);
    fn share_block_n(&self, to_share: Temporary, n: usize, instructions: &mut Vec<Code>);
    fn share_block(&self, to_share: Temporary, instructions: &mut Vec<Code>) {
        self.share_block_n(to_share, 1, instructions);
    }
    fn load(
        &self,
        to_load: TypingContext,
        existing_context: &TypingContext,
        instructions: &mut Vec<Code>,
    );
    fn store(
        &self,
        to_store: TypingContext,
        remaining_context: &TypingContext,
        instructions: &mut Vec<Code>,
    );
}
