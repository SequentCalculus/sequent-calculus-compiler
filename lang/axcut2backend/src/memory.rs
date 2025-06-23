//! This module provides an abstraction for some memory management operations.

use axcut::syntax::TypingContext;

/// This trait abstracts some operations for memory management.
pub trait Memory<Code, Temporary> {
    fn erase_block(to_erase: Temporary, instructions: &mut Vec<Code>);
    fn share_block_n(to_share: Temporary, n: usize, instructions: &mut Vec<Code>);
    fn share_block(to_share: Temporary, instructions: &mut Vec<Code>) {
        Self::share_block_n(to_share, 1, instructions);
    }
    fn load(to_load: TypingContext, existing_context: &TypingContext, instructions: &mut Vec<Code>);
    fn store(
        to_store: TypingContext,
        remaining_context: &TypingContext,
        instructions: &mut Vec<Code>,
    );
}
