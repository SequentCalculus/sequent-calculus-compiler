//! This module provides an abstraction for some memory management operations.

use axcut::syntax::TypingContext;

/// This trait abstracts some operations for memory management.
pub trait Memory<Code, Temporary> {
    /// This method generates code for erasing the pointer to the memory block in the given
    /// temporary `to_erase`. If the pointer was the last reference, the block is put onto the lazy
    /// free list. The pointer must either point to a valid memory block or be zero. In the latter
    /// case, the generated code is skipped.
    fn erase_block(to_erase: Temporary, instructions: &mut Vec<Code>);
    /// This method generates code for sharing the pointer to the memory block in by the given
    /// temporary `to_share` `n` times. The pointer must either point to a valid memory block or be
    /// zero. In the latter case, the generated code is skipped.
    fn share_block_n(to_share: Temporary, n: usize, instructions: &mut Vec<Code>);
    /// This method generates code for sharing the pointer to the memory block in by the given
    /// temporary `to_share` once. The pointer must either point to a valid memory block or be
    /// zero. In the latter case, the generated code is skipped.
    fn share_block(to_share: Temporary, instructions: &mut Vec<Code>) {
        Self::share_block_n(to_share, 1, instructions);
    }
    /// This method generates code for storing the temporaries of the right-most values of a
    /// context into memory. The pointer to the memory into which the values are stored will be put
    /// into the first temporary after the context remaining after the stores.
    /// - `to_store` is the list of variables the values are bound to before the stores.
    /// - `remaining_context` is the remaining context after the stores.
    /// - `instructions` is the list of instructions to which the new instructions are appended.
    fn store(
        to_store: TypingContext,
        remaining_context: &TypingContext,
        instructions: &mut Vec<Code>,
    );
    /// This method generates code for loading several values from memory into temporaries to the
    /// right of an existing context. The pointer to the memory from which the values are to be
    /// loaded is expected to be in the first temporary after the existing context.
    /// - `to_load` is the list of variables the values are bound to after the loads.
    /// - `existing_context` is the existing context before the loads.
    /// - `instructions` is the list of instructions to which the new instructions are appended.
    fn load(to_load: TypingContext, existing_context: &TypingContext, instructions: &mut Vec<Code>);
}
