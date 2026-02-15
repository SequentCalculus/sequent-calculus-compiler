//! This module contains some functions needed for generating code for explicit substitutions.

use crate::{
    code::Instructions,
    config::{
        Config,
        TemporaryNumber::{Fst, Snd},
    },
    memory::Memory,
    parallel_moves::{ParallelMoves, parallel_moves},
    utils::Utils,
};
use axcut::syntax::{Chirality, ContextBinding, Ident, TypingContext};

use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hash;

/// This function takes a list of pairs each of which associates a variable for a new typing
/// context with a binding from a given context. It then "transposes" this list, returning a mapping
/// for each binding in the given context to the associated variables in the new context.
/// - `rearrange` is the list of pairs.
/// - `context` is the given context.
pub fn transpose(
    rearrange: &[(ContextBinding, Ident)],
    context: &TypingContext,
) -> BTreeMap<ContextBinding, Vec<Ident>> {
    let mut target_map = BTreeMap::new();
    for binding in &context.bindings {
        let targets = rearrange
            .iter()
            .filter(|(_, old)| binding.var == *old)
            .map(|(new, _)| new.var.clone())
            .collect();
        let _ = target_map.insert(binding.clone(), targets);
    }
    target_map
}

/// This function performs the parallel-moves algorithm for the given mapping of old variables to
/// new variables.
/// - `target_map` maps the variables in the old context to the targets in the new context.
/// - `context` is the old context.
/// - `new_context` is the resulting context.
/// - `instructions` is the list of instructions to which the new instructions are appended.
pub fn code_exchange<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    target_map: &BTreeMap<ContextBinding, Vec<Ident>>,
    context: &TypingContext,
    new_context: &TypingContext,
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    /// This function transforms a mapping of variables to a corresponding mapping of temporaries.
    fn connections<Backend, Temporary: Ord, Immediate>(
        target_map: &BTreeMap<ContextBinding, Vec<Ident>>,
        context: &TypingContext,
        new_context: &TypingContext,
    ) -> BTreeMap<Temporary, BTreeSet<Temporary>>
    where
        Backend: Config<Temporary, Immediate> + Utils<Temporary>,
    {
        let mut target_list_temporaries = BTreeMap::new();
        for (binding, targets) in target_map {
            // values of external types like integers occupy only one temporary
            if binding.chi == Chirality::Ext {
                let _ = target_list_temporaries.insert(
                    Backend::variable_temporary(Snd, context, &binding.var),
                    targets
                        .iter()
                        .map(|target| Backend::variable_temporary(Snd, new_context, target))
                        .collect(),
                );
            } else {
                let _ = target_list_temporaries.insert(
                    Backend::variable_temporary(Fst, context, &binding.var),
                    targets
                        .iter()
                        .map(|target| Backend::variable_temporary(Fst, new_context, target))
                        .collect(),
                );
                let _ = target_list_temporaries.insert(
                    Backend::variable_temporary(Snd, context, &binding.var),
                    targets
                        .iter()
                        .map(|target| Backend::variable_temporary(Snd, new_context, target))
                        .collect(),
                );
            }
        }
        target_list_temporaries
    }

    parallel_moves::<Backend, _, _, _>(
        connections::<Backend, _, _>(target_map, context, new_context),
        instructions,
    );
}

/// This function performs updates of reference counts for the given mapping of old variables to new
/// variables.
/// - `target_map` maps the variables in the old context to the targets in the new context.
/// - `context` is the old context.
/// - `instructions` is the list of instructions to which the new instructions are appended.
pub fn code_weakening_contraction<Backend, Code, Temporary, Immediate>(
    target_map: &BTreeMap<ContextBinding, Vec<Ident>>,
    context: &TypingContext,
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + Utils<Temporary>,
{
    #[allow(clippy::cast_possible_wrap)]
    fn update_reference_count<Backend, Code, Temporary, Immediate>(
        variable: &Ident,
        context: &TypingContext,
        new_count: usize,
        instructions: &mut Vec<Code>,
    ) where
        Backend:
            Memory<Code, Temporary> + Instructions<Code, Temporary, Immediate> + Utils<Temporary>,
    {
        let temporary = Backend::variable_temporary(Fst, context, variable);
        match new_count {
            0 => {
                instructions.push(Backend::comment(format!("#erase {variable}")));
                Backend::erase_block(temporary, instructions);
            }
            1 => {}
            _ => {
                instructions.push(Backend::comment(format!("#share {variable}")));
                Backend::share_block_n(temporary, new_count - 1, instructions);
            }
        }
    }

    for (binding, targets) in target_map {
        // values of external types like integers have no reference count
        if binding.chi != Chirality::Ext {
            update_reference_count::<Backend, _, _, _>(
                &binding.var,
                context,
                targets.len(),
                instructions,
            );
        }
    }
}
