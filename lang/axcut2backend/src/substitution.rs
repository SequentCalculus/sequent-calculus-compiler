use crate::{
    config::{
        Config,
        TemporaryNumber::{Fst, Snd},
    },
    memory::Memory,
    parallel_moves::{parallel_moves, ParallelMoves},
    utils::Utils,
};
use axcut::syntax::{Chirality, ContextBinding, TypingContext, Var};

use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hash;

#[must_use]
pub fn transpose(
    rearrange: &[(Var, ContextBinding)],
    context: &TypingContext,
) -> BTreeMap<ContextBinding, Vec<Var>> {
    let mut target_map = BTreeMap::new();
    for binding in context {
        let targets = rearrange
            .iter()
            .filter(|mapping| binding.var == mapping.1.var)
            .map(|mapping| mapping.0.clone())
            .collect();
        let _ = target_map.insert(binding.clone(), targets);
    }
    target_map
}

pub fn code_exchange<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    target_map: &BTreeMap<ContextBinding, Vec<Var>>,
    context: &TypingContext,
    new_context: &TypingContext,
    backend: &Backend,
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate> + ParallelMoves<Code, Temporary> + Utils<Temporary>,
{
    fn connections<Backend, Temporary: Ord, Immediate>(
        target_map: &BTreeMap<ContextBinding, Vec<Var>>,
        context: &TypingContext,
        backend: &Backend,
        new_context: &TypingContext,
    ) -> BTreeMap<Temporary, BTreeSet<Temporary>>
    where
        Backend: Config<Temporary, Immediate> + Utils<Temporary>,
    {
        let mut target_list_temporaries = BTreeMap::new();
        for (binding, targets) in target_map {
            if binding.chi == Chirality::Ext {
                let _ = target_list_temporaries.insert(
                    backend.variable_temporary(Snd, context, &binding.var),
                    targets
                        .iter()
                        .map(|target| backend.variable_temporary(Snd, new_context, target))
                        .collect(),
                );
            } else {
                let _ = target_list_temporaries.insert(
                    backend.variable_temporary(Fst, context, &binding.var),
                    targets
                        .iter()
                        .map(|target| backend.variable_temporary(Fst, new_context, target))
                        .collect(),
                );
                let _ = target_list_temporaries.insert(
                    backend.variable_temporary(Snd, context, &binding.var),
                    targets
                        .iter()
                        .map(|target| backend.variable_temporary(Snd, new_context, target))
                        .collect(),
                );
            }
        }
        target_list_temporaries
    }

    parallel_moves(
        connections(target_map, context, backend, new_context),
        backend,
        instructions,
    );
}

pub fn code_weakening_contraction<Backend, Code, Temporary, Immediate>(
    target_map: &BTreeMap<ContextBinding, Vec<Var>>,
    context: &TypingContext,
    backend: &Backend,
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate> + Memory<Code, Temporary> + Utils<Temporary>,
{
    #[allow(clippy::cast_possible_wrap)]
    fn update_reference_count<Backend, Code, Temporary>(
        temporary: Temporary,
        new_count: usize,
        backend: &Backend,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Memory<Code, Temporary>,
    {
        match new_count {
            0 => backend.erase_block(temporary, instructions),
            1 => {}
            _ => backend.share_block_n(temporary, new_count - 1, instructions),
        }
    }

    // reversed order in iterator to adhere to Idris implementation
    for (binding, targets) in target_map.iter().rev() {
        if binding.chi != Chirality::Ext {
            update_reference_count(
                backend.variable_temporary(Fst, context, &binding.var),
                targets.len(),
                backend,
                instructions,
            );
        }
    }
}
