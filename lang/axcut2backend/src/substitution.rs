use crate::{
    code::Instructions,
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
    for binding in &context.bindings {
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
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    fn connections<Backend, Temporary: Ord, Immediate>(
        target_map: &BTreeMap<ContextBinding, Vec<Var>>,
        context: &TypingContext,
        new_context: &TypingContext,
    ) -> BTreeMap<Temporary, BTreeSet<Temporary>>
    where
        Backend: Config<Temporary, Immediate> + Utils<Temporary>,
    {
        let mut target_list_temporaries = BTreeMap::new();
        for (binding, targets) in target_map {
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

pub fn code_weakening_contraction<Backend, Code, Temporary, Immediate>(
    target_map: &BTreeMap<ContextBinding, Vec<Var>>,
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
        variable: &Var,
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

    for (binding, targets) in target_map.iter() {
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
