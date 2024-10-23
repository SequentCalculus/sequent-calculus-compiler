use super::code::Code;
use super::config::{
    variable_register, Immediate, Register,
    RegisterNumber::{Fst, Snd},
};
use super::memory::{erase_block, share_block_n};
use super::parallel_moves::parallel_moves;

use axcut::syntax::{Chirality, ContextBinding, TypingContext};

use std::collections::{BTreeMap, BTreeSet};

#[must_use]
pub fn transpose(
    rearrange: &[(ContextBinding, ContextBinding)],
    context: &TypingContext,
) -> BTreeMap<ContextBinding, Vec<ContextBinding>> {
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

pub fn code_exchange(
    target_map: &BTreeMap<ContextBinding, Vec<ContextBinding>>,
    context: &TypingContext,
    new_context: &TypingContext,
    instructions: &mut Vec<Code>,
) {
    fn connections(
        target_map: &BTreeMap<ContextBinding, Vec<ContextBinding>>,
        context: &TypingContext,
        new_context: &TypingContext,
    ) -> BTreeMap<Register, BTreeSet<Register>> {
        let mut target_list_registers = BTreeMap::new();
        for (binding, targets) in target_map {
            if binding.chi == Chirality::Ext {
                let _ = target_list_registers.insert(
                    variable_register(Snd, context, &binding.var),
                    targets
                        .iter()
                        .map(|target| variable_register(Snd, new_context, &target.var))
                        .collect(),
                );
            } else {
                let _ = target_list_registers.insert(
                    variable_register(Fst, context, &binding.var),
                    targets
                        .iter()
                        .map(|target| variable_register(Fst, new_context, &target.var))
                        .collect(),
                );
                let _ = target_list_registers.insert(
                    variable_register(Snd, context, &binding.var),
                    targets
                        .iter()
                        .map(|target| variable_register(Snd, new_context, &target.var))
                        .collect(),
                );
            }
        }
        target_list_registers
    }

    parallel_moves(connections(target_map, context, new_context), instructions);
}

pub fn code_weakening_contraction(
    target_map: &BTreeMap<ContextBinding, Vec<ContextBinding>>,
    context: &TypingContext,
    instructions: &mut Vec<Code>,
) {
    #[allow(clippy::cast_possible_wrap)]
    fn update_reference_count(register: Register, new_count: usize, instructions: &mut Vec<Code>) {
        match new_count {
            0 => erase_block(register, instructions),
            1 => {}
            _ => share_block_n(register, new_count as Immediate - 1, instructions),
        }
    }

    // reversed order in iterator to adhere to Idris implementation
    for (binding, targets) in target_map.iter().rev() {
        if binding.chi != Chirality::Ext {
            update_reference_count(
                variable_register(Fst, context, &binding.var),
                targets.len(),
                instructions,
            );
        }
    }
}
