use super::CodeStatement;
use crate::fresh_labels::fresh_label;
use crate::utils::{code_methods, code_table};
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{Chirality, ContextBinding, New, TypeDeclaration, TypingContext};

use std::hash::Hash;

impl CodeStatement for New {
    fn code_statement<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
        self,
        types: &[TypeDeclaration],
        mut context: TypingContext,
        backend: &Backend,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Memory<Code, Temporary>
            + ParallelMoves<Code, Temporary>
            + Utils<Temporary>,
    {
        let closure_environment = context.split_off(
            context.len()
                - self
                    .context
                    .expect("Closure environment must be annotated")
                    .len(),
        );
        backend.store(closure_environment.clone(), &context, instructions);
        let fresh_label = format!("{}{}", self.ty, fresh_label());
        context.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Cns,
            ty: self.ty,
        });
        let table_temporary = backend.variable_temporary(Snd, &context, &self.var);
        backend.load_label(table_temporary, fresh_label.clone(), instructions);
        self.next
            .code_statement(types, context, backend, instructions);
        let number_of_clauses = self.clauses.len();
        instructions.push(backend.label(fresh_label.clone()));
        if number_of_clauses <= 1 {
        } else {
            code_table(&self.clauses, &fresh_label, backend, instructions);
        }
        code_methods(
            &closure_environment,
            self.clauses,
            &fresh_label,
            types,
            backend,
            instructions,
        );
    }
}
