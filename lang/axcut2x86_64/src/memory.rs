use super::code::Code;
use super::config::{
    field_offset, fresh_temporary, stack_offset, Immediate, Register, Spill, Temporary,
    TemporaryNumber, FIELDS_PER_BLOCK, FREE, HEAP, NEXT_ELEMENT_OFFSET, REFERENCE_COUNT_OFFSET,
    SPILL_TEMP, STACK, TEMP,
};
use super::fresh_labels::fresh_label;
use super::utils::{compare_immediate, load_immediate};
use axcut::syntax::{Chirality, ContextBinding, TypingContext};
use TemporaryNumber::{Fst, Snd};

fn skip_if_zero(condition: Temporary, mut to_skip: Vec<Code>, instructions: &mut Vec<Code>) {
    let fresh_label = format!("lab{}", fresh_label());
    compare_immediate(condition, 0, instructions);
    instructions.push(Code::JEL(fresh_label.clone()));
    instructions.append(&mut to_skip);
    instructions.push(Code::LAB(fresh_label));
}

fn if_zero_then_else(
    condition: Register,
    offset: Option<i64>,
    mut then_branch: Vec<Code>,
    mut else_branch: Vec<Code>,
    instructions: &mut Vec<Code>,
) {
    let fresh_label_then = format!("lab{}", fresh_label());
    let fresh_label_else = format!("lab{}", fresh_label());

    match offset {
        Some(offset) => instructions.push(Code::CMPIM(condition, offset, 0)),
        None => instructions.push(Code::CMPI(condition, 0)),
    }

    instructions.push(Code::JEL(fresh_label_then.clone()));
    instructions.append(&mut else_branch);
    instructions.push(Code::JMPL(fresh_label_else.clone()));
    instructions.push(Code::LAB(fresh_label_then));
    instructions.append(&mut then_branch);
    instructions.push(Code::LAB(fresh_label_else));
}

pub fn share_block_n(to_share: Temporary, n: Immediate, instructions: &mut Vec<Code>) {
    let mut to_skip = Vec::with_capacity(4);
    match to_share {
        Temporary::R(to_share_register) => {
            to_skip.push(Code::ADDIM(to_share_register, REFERENCE_COUNT_OFFSET, n));
            skip_if_zero(to_share, to_skip, instructions);
        }
        Temporary::S(to_share_position) => {
            to_skip.push(Code::MOVL(TEMP, STACK, stack_offset(to_share_position)));
            to_skip.push(Code::ADDIM(TEMP, REFERENCE_COUNT_OFFSET, n));
            skip_if_zero(to_share, to_skip, instructions);
        }
    }
}

fn share_block(to_share: Temporary, instructions: &mut Vec<Code>) {
    share_block_n(to_share, 1, instructions);
}

#[allow(clippy::vec_init_then_push)]
pub fn erase_block(to_erase: Temporary, instructions: &mut Vec<Code>) {
    fn erase_vaid_object(to_erase: Register, instructions: &mut Vec<Code>) {
        let mut then_branch = Vec::with_capacity(2);
        then_branch.push(Code::MOVS(FREE, to_erase, NEXT_ELEMENT_OFFSET));
        then_branch.push(Code::MOV(FREE, to_erase));

        let mut else_branch = Vec::with_capacity(1);
        else_branch.push(Code::ADDIM(to_erase, REFERENCE_COUNT_OFFSET, -1));

        if_zero_then_else(
            to_erase,
            Some(REFERENCE_COUNT_OFFSET),
            then_branch,
            else_branch,
            instructions,
        );
    }

    let mut to_skip = Vec::with_capacity(10);

    match to_erase {
        Temporary::R(to_erase_register) => {
            erase_vaid_object(to_erase_register, &mut to_skip);
            skip_if_zero(to_erase, to_skip, instructions);
        }
        Temporary::S(to_erase_position) => {
            to_skip.push(Code::MOVL(TEMP, STACK, stack_offset(to_erase_position)));
            erase_vaid_object(TEMP, &mut to_skip);
            skip_if_zero(to_erase, to_skip, instructions);
        }
    }
}

#[allow(clippy::vec_init_then_push)]
fn acquire_block(new_block: Register, instructions: &mut Vec<Code>) {
    fn erase_fields(to_erase: Register, instructions: &mut Vec<Code>) {
        // reversed order in iterator to adhere to Idris implementation
        for offset in (0..FIELDS_PER_BLOCK).rev() {
            instructions.push(Code::MOVL(TEMP, to_erase, field_offset(Fst, offset)));
            erase_block(Temporary::R(TEMP), instructions);
        }
    }

    let block_is_temp = new_block == TEMP;

    if block_is_temp {
        instructions.push(Code::MOVS(HEAP, STACK, stack_offset(SPILL_TEMP)));
    } else {
        instructions.push(Code::MOV(new_block, HEAP));
    }
    instructions.push(Code::MOVL(HEAP, HEAP, NEXT_ELEMENT_OFFSET));

    let mut then_branch_free = Vec::with_capacity(2);
    then_branch_free.push(Code::MOV(FREE, HEAP));
    then_branch_free.push(Code::ADDI(FREE, field_offset(Fst, FIELDS_PER_BLOCK)));

    let mut else_branch_free = Vec::with_capacity(64);
    else_branch_free.push(Code::MOVIM(HEAP, NEXT_ELEMENT_OFFSET, 0));
    erase_fields(HEAP, &mut else_branch_free);

    let mut then_branch = Vec::with_capacity(64);
    then_branch.push(Code::MOV(HEAP, FREE));
    then_branch.push(Code::MOVL(FREE, FREE, NEXT_ELEMENT_OFFSET));
    if_zero_then_else(
        FREE,
        None,
        then_branch_free,
        else_branch_free,
        &mut then_branch,
    );

    let mut else_branch = Vec::with_capacity(2);
    else_branch.push(Code::MOVIM(new_block, REFERENCE_COUNT_OFFSET, 0));

    if_zero_then_else(HEAP, None, then_branch, else_branch, instructions);

    if block_is_temp {
        instructions.push(Code::MOVL(TEMP, STACK, stack_offset(SPILL_TEMP)));
    }
}

fn release_block(to_release: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::MOVS(HEAP, to_release, NEXT_ELEMENT_OFFSET));
    instructions.push(Code::MOV(HEAP, to_release));
}

fn store_zero(memory_block: Register, offset: usize, instructions: &mut Vec<Code>) {
    instructions.push(Code::MOVIM(memory_block, field_offset(Fst, offset), 0));
}

fn store_zeroes(free_fields: usize, memory_block: Register, instructions: &mut Vec<Code>) {
    // reversed order in iterator to adhere to Idris implementation
    for offset in (0..free_fields).rev() {
        store_zero(memory_block, offset, instructions);
    }
}

fn store_field(
    number: TemporaryNumber,
    context: &TypingContext,
    memory_block: Register,
    offset: usize,
    instructions: &mut Vec<Code>,
) {
    match fresh_temporary(number, context) {
        Temporary::R(register) => instructions.push(Code::MOVS(
            register,
            memory_block,
            field_offset(number, offset),
        )),
        Temporary::S(position) => {
            instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position)));
            instructions.push(Code::MOVS(TEMP, memory_block, field_offset(number, offset)));
        }
    }
}

/// If the `TEMP` register is clobbered by loading the field to a spill spot, then the memory
/// block pointer is restored if it was itself in `TEMP`, unless it was the final load, after which
/// the memory pointer is gone anyway. A better way would probably be to spill another register to
/// `SPILL_TEMP` and use this register for the memory block if the latter is itself in a spill
/// spot, since then all fields are loaded to spill spots, too, but the register must then only be
/// restored once after the final load. The alternative way can be handled in `load_fields` and
/// `load_fields_rest`.
fn load_field_fst(
    context: &TypingContext,
    memory_block: Register,
    memory_block_spill: Option<Spill>,
    offset: usize,
    final_load: FinalLoad,
    load_mode: LoadMode,
    instructions: &mut Vec<Code>,
) {
    match fresh_temporary(Fst, context) {
        Temporary::R(register) => {
            instructions.push(Code::MOVL(
                register,
                memory_block,
                field_offset(Fst, offset),
            ));
            match load_mode {
                LoadMode::Release => {}
                LoadMode::Share => share_block(Temporary::R(register), instructions),
            }
        }
        Temporary::S(position) => {
            instructions.push(Code::MOVL(TEMP, memory_block, field_offset(Fst, offset)));
            instructions.push(Code::MOVS(TEMP, STACK, stack_offset(position)));
            match load_mode {
                LoadMode::Release => {}
                LoadMode::Share => share_block(Temporary::R(TEMP), instructions),
            }
            match memory_block_spill {
                None => {}
                Some(memory_block_position) => {
                    if !final_load {
                        instructions.push(Code::MOVL(
                            TEMP,
                            STACK,
                            stack_offset(memory_block_position),
                        ));
                    }
                }
            }
        }
    }
}

fn load_field_snd(
    context: &TypingContext,
    memory_block: Register,
    memory_block_spill: Option<Spill>,
    offset: usize,
    instructions: &mut Vec<Code>,
) {
    match fresh_temporary(Snd, context) {
        Temporary::R(register) => {
            instructions.push(Code::MOVL(
                register,
                memory_block,
                field_offset(Snd, offset),
            ));
        }
        Temporary::S(position) => {
            instructions.push(Code::MOVL(TEMP, memory_block, field_offset(Snd, offset)));
            instructions.push(Code::MOVS(TEMP, STACK, stack_offset(position)));
            match memory_block_spill {
                None => {}
                Some(memory_block_position) => {
                    instructions.push(Code::MOVL(TEMP, STACK, stack_offset(memory_block_position)));
                }
            }
        }
    }
}

fn store_value(
    to_store: &ContextBinding,
    remaining_context: &TypingContext,
    memory_block: Register,
    offset: usize,
    instructions: &mut Vec<Code>,
) {
    store_field(Snd, remaining_context, memory_block, offset, instructions);
    if to_store.chi == Chirality::Ext {
        store_zero(memory_block, offset, instructions);
    } else {
        store_field(Fst, remaining_context, memory_block, offset, instructions);
    }
}

type FinalLoad = bool;

#[derive(Debug, Clone, Copy)]
enum LoadMode {
    Release,
    Share,
}

#[allow(clippy::too_many_arguments)]
fn load_binder(
    to_load: &ContextBinding,
    existing_context: &TypingContext,
    memory_block: Register,
    memory_block_spill: Option<Spill>,
    offset: usize,
    final_load: FinalLoad,
    load_mode: LoadMode,
    instructions: &mut Vec<Code>,
) {
    load_field_snd(
        existing_context,
        memory_block,
        memory_block_spill,
        offset,
        instructions,
    );
    // skip label to adhere to Idris implementation
    fresh_label();
    if to_load.chi != Chirality::Ext {
        load_field_fst(
            existing_context,
            memory_block,
            memory_block_spill,
            offset,
            final_load,
            load_mode,
            instructions,
        );
        // skip label in release mode to adhere to Idris implementation
        match load_mode {
            LoadMode::Release => {
                fresh_label();
            }
            LoadMode::Share => {}
        }
    }
}

fn store_values(
    mut to_store: TypingContext,
    remaining_context: &TypingContext,
    memory_block: Register,
    mut free_fields: usize,
    instructions: &mut Vec<Code>,
) {
    while let Some(binding) = to_store.pop() {
        let mut remaining_plus_rest = remaining_context.clone();
        remaining_plus_rest.append(&mut to_store.clone());

        store_value(
            &binding,
            &remaining_plus_rest,
            memory_block,
            free_fields - 1,
            instructions,
        );

        free_fields -= 1;
    }

    store_zeroes(free_fields, memory_block, instructions);
}

fn load_binders(
    mut to_load: TypingContext,
    existing_context: &TypingContext,
    memory_block: Register,
    memory_block_position: Option<Spill>,
    mut free_fields: usize,
    load_mode: LoadMode,
    instructions: &mut Vec<Code>,
) {
    while let Some(binding) = to_load.pop() {
        let mut existing_plus_rest = existing_context.clone();
        existing_plus_rest.append(&mut to_load.clone());

        load_binder(
            &binding,
            &existing_plus_rest,
            memory_block,
            memory_block_position,
            free_fields - 1,
            to_load.is_empty(),
            load_mode,
            instructions,
        );

        free_fields -= 1;
    }
}

fn store_rest(
    mut to_store: TypingContext,
    remaining_context: &TypingContext,
    instructions: &mut Vec<Code>,
) {
    if !to_store.is_empty() {
        let mut remaining_plus_to_store = remaining_context.clone();
        remaining_plus_to_store.append(&mut to_store.clone());

        store_field(
            Fst,
            &remaining_plus_to_store,
            HEAP,
            FIELDS_PER_BLOCK - 1,
            instructions,
        );

        let rest_length = if to_store.len() < FIELDS_PER_BLOCK {
            0
        } else {
            to_store.len() - (FIELDS_PER_BLOCK - 1)
        };
        let to_store_next = to_store.split_off(rest_length);

        let mut remaining_plus_rest = remaining_context.clone();
        remaining_plus_rest.append(&mut to_store.clone());

        store_values(
            to_store_next,
            &remaining_plus_rest,
            HEAP,
            FIELDS_PER_BLOCK - 1,
            instructions,
        );

        match fresh_temporary(Fst, &remaining_plus_rest) {
            Temporary::R(memory_block_register) => {
                acquire_block(memory_block_register, instructions);
            }
            Temporary::S(memory_block_position) => {
                acquire_block(TEMP, instructions);
                instructions.push(Code::MOVS(TEMP, STACK, stack_offset(memory_block_position)));
            }
        }

        store_rest(to_store, remaining_context, instructions);
    }
}

pub fn store(
    mut to_store: TypingContext,
    remaining_context: &TypingContext,
    instructions: &mut Vec<Code>,
) {
    if to_store.is_empty() {
        load_immediate(fresh_temporary(Fst, remaining_context), 0, instructions);
    } else {
        let rest_length = if to_store.len() <= FIELDS_PER_BLOCK {
            0
        } else {
            to_store.len() - FIELDS_PER_BLOCK
        };
        let to_store_first = to_store.split_off(rest_length);

        let mut remaining_plus_rest = remaining_context.clone();
        remaining_plus_rest.append(&mut to_store.clone());

        store_values(
            to_store_first,
            &remaining_plus_rest,
            HEAP,
            FIELDS_PER_BLOCK,
            instructions,
        );

        match fresh_temporary(Fst, &remaining_plus_rest) {
            Temporary::R(memory_block_register) => {
                acquire_block(memory_block_register, instructions);
            }
            Temporary::S(memory_block_position) => {
                acquire_block(TEMP, instructions);
                instructions.push(Code::MOVS(TEMP, STACK, stack_offset(memory_block_position)));
            }
        }

        store_rest(to_store, remaining_context, instructions);
    }
}

fn load_fields_rest(
    mut to_load: TypingContext,
    existing_context: &TypingContext,
    load_mode: LoadMode,
    instructions: &mut Vec<Code>,
) {
    if !to_load.is_empty() {
        let mut existing_plus_to_load = existing_context.clone();
        existing_plus_to_load.append(&mut to_load.clone());

        let rest_length = if to_load.len() < FIELDS_PER_BLOCK {
            0
        } else {
            to_load.len() - (FIELDS_PER_BLOCK - 1)
        };
        let to_load_next = to_load.split_off(rest_length);

        let mut existing_plus_rest = existing_context.clone();
        existing_plus_rest.append(&mut to_load.clone());

        load_fields_rest(to_load, existing_context, load_mode, instructions);

        let memory_block = fresh_temporary(Fst, &existing_plus_rest);

        match memory_block {
            Temporary::R(memory_block_register) => {
                match load_mode {
                    LoadMode::Release => release_block(memory_block_register, instructions),
                    LoadMode::Share => {}
                }

                // skip label to adhere to Idris implementation
                fresh_label();
                load_field_fst(
                    &existing_plus_to_load,
                    memory_block_register,
                    None,
                    FIELDS_PER_BLOCK - 1,
                    false,
                    LoadMode::Release,
                    instructions,
                );

                load_binders(
                    to_load_next,
                    &existing_plus_rest,
                    memory_block_register,
                    None,
                    FIELDS_PER_BLOCK - 1,
                    load_mode,
                    instructions,
                );
            }
            Temporary::S(memory_block_position) => {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(memory_block_position)));
                match load_mode {
                    LoadMode::Release => release_block(TEMP, instructions),
                    LoadMode::Share => {}
                }

                // skip label to adhere to Idris implementation
                fresh_label();
                load_field_fst(
                    &existing_plus_to_load,
                    TEMP,
                    Some(memory_block_position),
                    FIELDS_PER_BLOCK - 1,
                    false,
                    LoadMode::Release,
                    instructions,
                );

                load_binders(
                    to_load_next,
                    &existing_plus_rest,
                    TEMP,
                    Some(memory_block_position),
                    FIELDS_PER_BLOCK - 1,
                    load_mode,
                    instructions,
                );
            }
        }
    }
}

fn load_fields(
    mut to_load: TypingContext,
    existing_context: &TypingContext,
    load_mode: LoadMode,
    instructions: &mut Vec<Code>,
) {
    if !to_load.is_empty() {
        let rest_length = if to_load.len() <= FIELDS_PER_BLOCK {
            0
        } else {
            to_load.len() - FIELDS_PER_BLOCK
        };
        let to_load_last = to_load.split_off(rest_length);

        let mut existing_plus_rest = existing_context.clone();
        existing_plus_rest.append(&mut to_load.clone());

        load_fields_rest(to_load, existing_context, load_mode, instructions);

        let memory_block = fresh_temporary(Fst, &existing_plus_rest);

        match memory_block {
            Temporary::R(memory_block_register) => {
                match load_mode {
                    LoadMode::Release => release_block(memory_block_register, instructions),
                    LoadMode::Share => {}
                }

                load_binders(
                    to_load_last,
                    &existing_plus_rest,
                    memory_block_register,
                    None,
                    FIELDS_PER_BLOCK,
                    load_mode,
                    instructions,
                );
            }
            Temporary::S(memory_block_position) => {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(memory_block_position)));
                match load_mode {
                    LoadMode::Release => release_block(TEMP, instructions),
                    LoadMode::Share => {}
                }

                load_binders(
                    to_load_last,
                    &existing_plus_rest,
                    TEMP,
                    Some(memory_block_position),
                    FIELDS_PER_BLOCK,
                    load_mode,
                    instructions,
                );
            }
        }
    }
}

pub fn load(
    to_load: TypingContext,
    existing_context: &TypingContext,
    instructions: &mut Vec<Code>,
) {
    fn load_register(
        memory_block: Register,
        to_load: TypingContext,
        existing_context: &TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        let mut then_branch = Vec::new();
        load_fields(
            to_load.clone(),
            existing_context,
            LoadMode::Release,
            &mut then_branch,
        );

        let mut else_branch = Vec::new();
        else_branch.push(Code::ADDIM(memory_block, REFERENCE_COUNT_OFFSET, -1));
        load_fields(to_load, existing_context, LoadMode::Share, &mut else_branch);

        if_zero_then_else(
            memory_block,
            Some(REFERENCE_COUNT_OFFSET),
            then_branch,
            else_branch,
            instructions,
        );
    }

    if !to_load.is_empty() {
        let memory_block = fresh_temporary(Fst, existing_context);

        match memory_block {
            Temporary::R(memory_block_register) => load_register(
                memory_block_register,
                to_load,
                existing_context,
                instructions,
            ),
            Temporary::S(memory_block_position) => {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(memory_block_position)));
                load_register(TEMP, to_load, existing_context, instructions);
            }
        }
    }
}
