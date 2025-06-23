//! This module provides an abstraction for some memory management operations.

use axcut::syntax::TypingContext;

/// This trait abstracts some operations for memory management.
pub trait Memory<Code, Temporary> {
    /// This method erases the memory block pointed to by the temporary.
    fn erase_block(to_erase: Temporary, instructions: &mut Vec<Code>);
    /// This method shares the memory block pointed to by the temporary `n` times.
    fn share_block_n(to_share: Temporary, n: usize, instructions: &mut Vec<Code>);
    /// This method shares the memory block pointed to by the temporary once.
    fn share_block(to_share: Temporary, instructions: &mut Vec<Code>) {
        Self::share_block_n(to_share, 1, instructions);
    }
    /// This method loads the variables in a typing context on top of the existing context.
    fn load(to_load: TypingContext, existing_context: &TypingContext, instructions: &mut Vec<Code>);
    /// This method stores the variables in a part of the typing context keeping the remaining ones.
    fn store(
        to_store: TypingContext,
        remaining_context: &TypingContext,
        instructions: &mut Vec<Code>,
    );
}
