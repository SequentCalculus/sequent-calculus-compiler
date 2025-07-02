//! This module provides an abstraction over machine instructions.

use axcut::syntax::{ContextBinding, Name};

/// This trait aims to abstract over machine instructions for several platforms. The instructions
/// are used to implement most of the logic for code generation.
pub trait Instructions<Code, Temporary, Immediate> {
    /// This generates an assembly comment `msg`.
    fn comment(msg: String) -> Code;
    /// This generates an assembly label `name`.
    fn label(name: Name) -> Code;
    /// This instruction jumps to the address in `temporary`.
    fn jump(temporary: Temporary, instructions: &mut Vec<Code>);
    /// This instruction jumps to the address of the label `name`.
    fn jump_label(name: Name, instructions: &mut Vec<Code>);
    /// This instruction jumps to the address of the label `name` and is guaranteed to have a fixed
    /// size, which not all platforms guarantee by default (e.g., x86_64).
    fn jump_label_fixed(name: Name, instructions: &mut Vec<Code>);
    /// This instruction jumps to the address of the label `name` if the two temporaries are equal.
    fn jump_label_if_equal(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    );
    /// This instruction jumps to the address of the label `name` if the two temporaries are not
    /// equal.
    fn jump_label_if_not_equal(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    );
    /// This instruction jumps to the address of the label `name` if the first temporary is less
    /// than the second one.
    fn jump_label_if_less(fst: Temporary, snd: Temporary, name: Name, instructions: &mut Vec<Code>);
    /// This instruction jumps to the address of the label `name` if the first temporary is less
    /// than or equal to the second one.
    fn jump_label_if_less_or_equal(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    );
    /// This instruction jumps to the address of the label `name` if the temporary is zero.
    fn jump_label_if_zero(temporary: Temporary, name: Name, instructions: &mut Vec<Code>);
    /// This instruction jumps to the address of the label `name` if the temporary is not zero.
    fn jump_label_if_not_zero(temporary: Temporary, name: Name, instructions: &mut Vec<Code>);
    /// This instruction loads the immediate into the temporary.
    fn load_immediate(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>);
    /// This instruction loads the address of the label `name` into the temporary.
    fn load_label(temporary: Temporary, name: Name, instructions: &mut Vec<Code>);
    /// This instruction adds an immediate to the temporary and then jumps to the resulting
    /// address. This may clobber `temporary`.
    fn add_and_jump(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>);
    /// This instruction adds the two source temporaries.
    fn add(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    /// This instruction subtracts the second source temporary from the first source temporary.
    fn sub(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    /// This instruction multiplies the two source temporaries.
    fn mul(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    /// This instruction divides the first source temporary by the second source temporary.
    fn div(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    /// This instruction calculates the remainder when dividing the first source temporary by the
    /// second source temporary.
    fn rem(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    );
    /// This instruction moves the source temporary into the target temporary.
    fn mov(target_temporary: Temporary, source_temporary: Temporary, instructions: &mut Vec<Code>);
    /// This instruction prints a 64-bit integer.
    fn print_i64(
        newline: bool,
        source_temporary: Temporary,
        context: &[ContextBinding],
        instructions: &mut Vec<Code>,
    );
}
