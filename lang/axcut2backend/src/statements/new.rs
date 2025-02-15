use printer::{tokens::NEW, Print};

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
use axcut::syntax::{statements::New, Chirality, ContextBinding, TypeDeclaration, TypingContext};

use std::hash::Hash;

impl CodeStatement for New {
    fn code_statement<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
        self,
        types: &[TypeDeclaration],
        mut context: TypingContext,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Memory<Code, Temporary>
            + ParallelMoves<Code, Temporary>
            + Utils<Temporary>,
    {
        let comment = format!(
            "{NEW} {}: {} = ({})\\{{ ... \\}};",
            self.var,
            self.ty.print_to_string(None),
            self.context.print_to_string(None)
        );
        instructions.push(Backend::comment(comment));

        let closure_environment = context.bindings.split_off(
            context.bindings.len()
                - self
                    .context
                    .expect("Closure environment must be annotated")
                    .len(),
        );
        Backend::store(closure_environment.clone().into(), &context, instructions);
        let fresh_label = format!(
            "{}_{}",
            self.ty
                .print_to_string(None)
                .replace('[', "_")
                .replace(", ", "_")
                .replace(']', ""),
            fresh_label()
        );
        context.bindings.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Cns,
            ty: self.ty,
        });

        instructions.push(Backend::comment("#load tag".to_string()));
        let table_temporary = Backend::variable_temporary(Snd, &context, &self.var);
        Backend::load_label(table_temporary, fresh_label.clone(), instructions);

        self.next
            .code_statement::<Backend, _, _, _>(types, context, instructions);

        let number_of_clauses = self.clauses.len();
        instructions.push(Backend::label(fresh_label.clone()));
        // the case < 1 cannot be triggered
        if number_of_clauses <= 1 {
        } else {
            code_table::<Backend, _, _, _>(&self.clauses, &fresh_label, instructions);
        }
        code_methods::<Backend, _, _, _>(
            &closure_environment.into(),
            self.clauses,
            &fresh_label,
            types,
            instructions,
        );
    }
}
