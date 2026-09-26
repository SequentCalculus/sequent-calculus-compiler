//! This module contains some utility functions used during code generation.

use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber},
    memory::Memory,
    parallel_moves::ParallelMoves,
    statements::CodeStatement,
};
use axcut::syntax::{ID, Identifier, Ty, TypeDeclaration, TypingContext, statements::Clause};

use std::hash::Hash;

/// This trait abstracts some utilities used during code generation.
pub trait Utils<Temporary> {
    /// This method calculates the temporaries corresponding to a variable in a typing context.
    /// - `number` decides whether the first or second temporary for the variable is returned.
    /// - `context` is the typing context the variable is in.
    /// - `variable_id` is the [`ID`] of the variable for which the temporary is calculated.
    ///
    /// # Panics
    ///
    /// A panic is caused if the `program` contains too many live variables at some point, so that we
    /// run out of temporaries.
    fn variable_temporary(
        number: TemporaryNumber,
        context: &TypingContext,
        variable_id: ID,
    ) -> Temporary;
    /// This method calculates the next free temporaries for a variable after a given typing
    /// context.
    /// - `number` decides whether the first or second temporary for the variable is returned.
    /// - `context` is the typing context.
    ///
    /// # Panics
    ///
    /// A panic is caused if the `program` contains too many live variables at some point, so that we
    /// run out of temporaries.
    fn fresh_temporary(number: TemporaryNumber, context: &TypingContext) -> Temporary;
}

/// Builds the assembler-safe form of an identifier's name, replacing every character a human-
/// facing identifier can contain but an assembly label cannot: `[` and `, ` (from
/// `mono::naming_table`'s bracket-form specialization names) become `_`, `]` is dropped entirely,
/// and `#` (from `splitting::split_table`'s `Box#1`-style equivalence-class names) becomes `__`,
/// plus the `_id` suffix `Print for Identifier` appends for a non-zero `id`.
///
/// Built directly from the identifier's own `name`/`id` fields, not by patching its already
/// human-facing printed form, so `Print for Identifier` itself can stay readable (`Box#1`, not
/// `Box__1`) without that readability ever reaching an actual label.
pub fn asm_safe_name(name: &Identifier) -> String {
    let sanitized = name
        .name
        .replace('[', "_")
        .replace(", ", "_")
        .replace(']', "")
        .replace('#', "__");
    if name.id == 0 {
        sanitized
    } else {
        format!("{sanitized}_{}", name.id)
    }
}

/// [`asm_safe_name`] for a [`Ty`]: `i64` needs no sanitizing, `Ty::Decl` defers to its identifier.
pub fn asm_safe_ty_name(ty: &Ty) -> String {
    match ty {
        Ty::I64 => "i64".to_string(),
        Ty::Decl(name) => asm_safe_name(name),
    }
}

/// This function generates a jump table for a list of clauses.
/// - `clauses` is the lists of clauses.
/// - `base_label` is the base name of the labels in the table.
/// - `instructions` is the list of instructions to which the new instructions are appended.
pub fn code_table<Backend, Code, Temporary, Immediate>(
    clauses: &Vec<Clause>,
    base_label: &str,
    instructions: &mut Vec<Code>,
) where
    Backend: Instructions<Code, Temporary, Immediate>,
{
    for clause in clauses {
        Backend::jump_label_fixed(
            base_label.to_string() + "_" + &asm_safe_name(&clause.xtor),
            instructions,
        );
    }
}

/// This function generates code for a clause used in a [`axcut::syntax::statements::Switch`]. It
/// first loads the variables stored in the scrutinee on top of the variables in the given typing
/// context.
/// - `context` is the given typing context.
/// - `clause` is the clause.
/// - `types` is the list of type declarations in the program.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn code_clause<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    mut context: TypingContext,
    mut clause: Clause,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    Backend::load(clause.context.clone(), &context, instructions);
    context.bindings.append(&mut clause.context.bindings);
    clause
        .body
        .code_statement::<Backend, _, _, _>(types, context, instructions);
}

/// This function generates code for a clause used in a [`axcut::syntax::statements::Create`]. It
/// first loads the variables stored in the closure on top of the variables in the typing context
/// the clause abstracts (which must already be in temporaries).
/// - `closure_environment` are the variables stored in the closure.
/// - `clause` is the clause.
/// - `types` is the list of type declarations in the program.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn code_method<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    mut closure_environment: TypingContext,
    mut clause: Clause,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    Backend::load(closure_environment.clone(), &clause.context, instructions);
    clause
        .context
        .bindings
        .append(&mut closure_environment.bindings);
    clause
        .body
        .code_statement::<Backend, _, _, _>(types, clause.context, instructions);
}

/// This function generates code for the clauses of a [`axcut::syntax::statements::Switch`]. The
/// code for each clause start with a fresh label which is the target of a jump in a jump table.
/// - `context` is the given typing context.
/// - `clauses` is the list of clauses.
/// - `base_label` is the base name of the labels for the clauses.
/// - `types` is the list of type declarations in the program.
/// - `instructions` is the list of instructions to which the new instructions are appended.
pub fn code_clauses<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    context: &TypingContext,
    clauses: Vec<Clause>,
    base_label: &str,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    for clause in clauses {
        instructions.push(Backend::label(
            base_label.to_string() + "_" + &asm_safe_name(&clause.xtor),
        ));
        code_clause::<Backend, _, _, _>(context.clone(), clause, types, instructions);
    }
}

/// This function generates code for the clauses of a [`axcut::syntax::statements::Create`]. The
/// code for each clause start with a fresh label which is the target of a jump in a jump table.
/// - `closure_environment` are the variables stored in the closure.
/// - `clauses` is the list of clauses.
/// - `base_label` is the base name of the labels for the clauses.
/// - `types` is the list of type declarations in the program.
/// - `instructions` is the list of instructions to which the new instructions are appended.
pub fn code_methods<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    closure_environment: &TypingContext,
    clauses: Vec<Clause>,
    base_label: &str,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    for clause in clauses {
        instructions.push(Backend::label(
            base_label.to_string() + "_" + &asm_safe_name(&clause.xtor),
        ));
        code_method::<Backend, _, _, _>(closure_environment.clone(), clause, types, instructions);
    }
}
