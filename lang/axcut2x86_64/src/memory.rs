use super::code::{compare_immediate, Code};
use super::config::{
    field_offset, stack_offset, Immediate, Register, Temporary, FIELDS_PER_BLOCK, FREE, HEAP,
    NEXT_ELEMENT_OFFSET, REFERENCE_COUNT_OFFSET, RETURN1, SPILL_TEMP, STACK, TEMP,
};
use super::Backend;

use axcut::syntax::{Chirality, ContextBinding, TypingContext};
use axcut2backend::{
    code::Instructions, config::TemporaryNumber, fresh_labels::fresh_label, memory::Memory,
    utils::Utils,
};
use TemporaryNumber::{Fst, Snd};

fn skip_if_zero(condition: Temporary, mut to_skip: Vec<Code>, instructions: &mut Vec<Code>) {
    let fresh_label = format!("lab{}", fresh_label());
    compare_immediate(condition, 0.into(), instructions);
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
        Some(offset) => instructions.push(Code::CMPIM(condition, offset.into(), 0.into())),
        None => instructions.push(Code::CMPI(condition, Immediate { val: 0 })),
    }

    instructions.push(Code::JEL(fresh_label_then.clone()));
    instructions.append(&mut else_branch);
    instructions.push(Code::JMPL(fresh_label_else.clone()));
    instructions.push(Code::LAB(fresh_label_then));
    instructions.append(&mut then_branch);
    instructions.push(Code::LAB(fresh_label_else));
}

#[allow(clippy::vec_init_then_push)]
fn acquire_block(new_block: Temporary, instructions: &mut Vec<Code>) {
    fn erase_fields(to_erase: Register, instructions: &mut Vec<Code>) {
        // reversed order in iterator to adhere to Idris implementation
        for offset in (0..FIELDS_PER_BLOCK).rev() {
            instructions.push(Code::COMMENT(format!(
                "     check child {} for erasure",
                offset + 1
            )));
            instructions.push(Code::MOVL(TEMP, to_erase, field_offset(Fst, offset)));
            Backend::erase_block(Temporary::Register(TEMP), instructions);
        }
    }

    match new_block {
        Temporary::Register(new_block_register) => {
            instructions.push(Code::MOV(new_block_register, HEAP));
        }
        Temporary::Spill(new_block_position) => {
            // this moves the memory block both to `TEMP` and to its spill position for better
            // performance in the fast path, but executes the first instruction unnecessarily in the
            // slow path
            instructions.push(Code::MOV(TEMP, HEAP));
            instructions.push(Code::MOVS(HEAP, STACK, stack_offset(new_block_position)));
        }
    }

    instructions.push(Code::COMMENT(
        "  get next free block into heap register".to_string(),
    ));
    instructions.push(Code::COMMENT(
        "   (1) check linear free list for next block".to_string(),
    ));
    instructions.push(Code::MOVL(HEAP, HEAP, NEXT_ELEMENT_OFFSET.into()));

    let mut then_branch_free = Vec::with_capacity(3);
    then_branch_free.push(Code::COMMENT(
        "   (3) fall back to bump allocation".to_string(),
    ));
    then_branch_free.push(Code::MOV(FREE, HEAP));
    then_branch_free.push(Code::ADDI(FREE, field_offset(Fst, FIELDS_PER_BLOCK)));

    let mut else_branch_free = Vec::with_capacity(64);
    else_branch_free.push(Code::COMMENT("    mark linear free list empty".to_string()));
    else_branch_free.push(Code::MOVIM(HEAP, NEXT_ELEMENT_OFFSET.into(), 0.into()));
    else_branch_free.push(Code::COMMENT(
        "    erase children of next block".to_string(),
    ));
    erase_fields(HEAP, &mut else_branch_free);

    let mut then_branch = Vec::with_capacity(64);
    then_branch.push(Code::COMMENT(
        "   (2) check non-linear lazy free list for next block".to_string(),
    ));
    then_branch.push(Code::MOV(HEAP, FREE));
    then_branch.push(Code::MOVL(FREE, FREE, NEXT_ELEMENT_OFFSET.into()));
    if_zero_then_else(
        FREE,
        None,
        then_branch_free,
        else_branch_free,
        &mut then_branch,
    );

    let mut else_branch = Vec::with_capacity(3);
    else_branch.push(Code::COMMENT(
        "    initialize refcount of just acquired block".to_string(),
    ));
    match new_block {
        Temporary::Register(new_block_register) => {
            else_branch.push(Code::MOVIM(
                new_block_register,
                REFERENCE_COUNT_OFFSET.into(),
                0.into(),
            ));
        }
        Temporary::Spill(_new_block_position) => {
            // this instruction would be needed if the above optimization for the fast path would
            // not be made
            //else_branch.push(Code::MOVL(TEMP, STACK, stack_offset(new_block_position)));
            else_branch.push(Code::MOVIM(TEMP, REFERENCE_COUNT_OFFSET.into(), 0.into()));
        }
    }

    if_zero_then_else(HEAP, None, then_branch, else_branch, instructions);
}

fn release_block(to_release: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::MOVS(HEAP, to_release, NEXT_ELEMENT_OFFSET.into()));
    instructions.push(Code::MOV(HEAP, to_release));
}

fn store_zero(memory_block: Register, offset: usize, instructions: &mut Vec<Code>) {
    instructions.push(Code::MOVIM(
        memory_block,
        field_offset(Fst, offset),
        0.into(),
    ));
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
    match Backend::fresh_temporary(number, context) {
        Temporary::Register(register) => instructions.push(Code::MOVS(
            register,
            memory_block,
            field_offset(number, offset),
        )),
        Temporary::Spill(position) => {
            instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position)));
            instructions.push(Code::MOVS(TEMP, memory_block, field_offset(number, offset)));
        }
    }
}

fn load_field(
    number: TemporaryNumber,
    context: &TypingContext,
    memory_block: Register,
    offset: usize,
    instructions: &mut Vec<Code>,
) {
    match Backend::fresh_temporary(number, context) {
        Temporary::Register(register) => {
            instructions.push(Code::MOVL(
                register,
                memory_block,
                field_offset(number, offset),
            ));
        }
        Temporary::Spill(position) => {
            instructions.push(Code::MOVL(TEMP, memory_block, field_offset(number, offset)));
            instructions.push(Code::MOVS(TEMP, STACK, stack_offset(position)));
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

#[derive(Debug, Clone, Copy)]
enum LoadMode {
    Release,
    Share,
}

fn load_binder(
    to_load: &ContextBinding,
    existing_context: &TypingContext,
    memory_block: Register,
    offset: usize,
    load_mode: LoadMode,
    instructions: &mut Vec<Code>,
) {
    load_field(Snd, existing_context, memory_block, offset, instructions);
    if to_load.chi != Chirality::Ext {
        load_field(Fst, existing_context, memory_block, offset, instructions);
        let register_to_share = match Backend::fresh_temporary(Fst, existing_context) {
            Temporary::Register(register) => register,
            // if field was loaded to spill position, it is still in `TEMP` here
            Temporary::Spill(_) => TEMP,
        };
        match load_mode {
            LoadMode::Release => {
                // skip label in release mode to adhere to Idris implementation
                fresh_label();
            }
            LoadMode::Share => {
                Backend::share_block(Temporary::Register(register_to_share), instructions);
            }
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
    instructions.push(Code::COMMENT("  store values".to_string()));
    while let Some(binding) = to_store.bindings.pop() {
        let mut remaining_plus_rest = remaining_context.clone();
        remaining_plus_rest
            .bindings
            .append(&mut to_store.bindings.clone());

        store_value(
            &binding,
            &remaining_plus_rest,
            memory_block,
            free_fields - 1,
            instructions,
        );

        free_fields -= 1;
    }

    if free_fields > 0 {
        instructions.push(Code::COMMENT("  mark unused fields with null".to_string()));
    }
    store_zeroes(free_fields, memory_block, instructions);
}

fn load_binders(
    mut to_load: TypingContext,
    existing_context: &TypingContext,
    memory_block: Register,
    mut free_fields: usize,
    load_mode: LoadMode,
    instructions: &mut Vec<Code>,
) {
    while let Some(binding) = to_load.bindings.pop() {
        let mut existing_plus_rest = existing_context.clone();
        existing_plus_rest
            .bindings
            .append(&mut to_load.bindings.clone());

        load_binder(
            &binding,
            &existing_plus_rest,
            memory_block,
            free_fields - 1,
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
    if !to_store.bindings.is_empty() {
        let mut remaining_plus_to_store = remaining_context.clone();
        remaining_plus_to_store
            .bindings
            .append(&mut to_store.bindings.clone());

        store_field(
            Fst,
            &remaining_plus_to_store,
            HEAP,
            FIELDS_PER_BLOCK - 1,
            instructions,
        );

        let rest_length = if to_store.bindings.len() < FIELDS_PER_BLOCK {
            0
        } else {
            to_store.bindings.len() - (FIELDS_PER_BLOCK - 1)
        };
        let to_store_next = to_store.bindings.split_off(rest_length);

        let mut remaining_plus_rest = remaining_context.clone();
        remaining_plus_rest
            .bindings
            .append(&mut to_store.bindings.clone());

        store_values(
            to_store_next.into(),
            &remaining_plus_rest,
            HEAP,
            FIELDS_PER_BLOCK - 1,
            instructions,
        );

        acquire_block(
            Backend::fresh_temporary(Fst, &remaining_plus_rest),
            instructions,
        );

        store_rest(to_store, remaining_context, instructions);
    }
}

fn load_fields_rest(
    mut to_load: TypingContext,
    existing_context: &TypingContext,
    load_mode: LoadMode,
    register_freed: &mut bool,
    instructions: &mut Vec<Code>,
) {
    if !to_load.bindings.is_empty() {
        let mut existing_plus_to_load = existing_context.clone();
        existing_plus_to_load
            .bindings
            .append(&mut to_load.bindings.clone());

        let rest_length = if to_load.bindings.len() < FIELDS_PER_BLOCK {
            0
        } else {
            to_load.bindings.len() - (FIELDS_PER_BLOCK - 1)
        };
        let to_load_next = to_load.bindings.split_off(rest_length);

        let mut existing_plus_rest = existing_context.clone();
        existing_plus_rest
            .bindings
            .append(&mut to_load.bindings.clone());

        load_fields_rest(
            to_load,
            existing_context,
            load_mode,
            register_freed,
            instructions,
        );

        let memory_block = Backend::fresh_temporary(Fst, &existing_plus_rest);

        match memory_block {
            Temporary::Register(memory_block_register) => {
                match load_mode {
                    LoadMode::Release => release_block(memory_block_register, instructions),
                    LoadMode::Share => {}
                }

                load_field(
                    Fst,
                    &existing_plus_to_load,
                    memory_block_register,
                    FIELDS_PER_BLOCK - 1,
                    instructions,
                );

                load_binders(
                    to_load_next.into(),
                    &existing_plus_rest,
                    memory_block_register,
                    FIELDS_PER_BLOCK - 1,
                    load_mode,
                    instructions,
                );
            }
            Temporary::Spill(memory_block_position) => {
                // the first time a memory block is in a spill position, we free a register for it
                // and only restore the register after the last load in `load_fields`, since all
                // memory blocks after this one will also be in a spill position
                if !*register_freed {
                    instructions.push(Code::MOVS(RETURN1, STACK, stack_offset(SPILL_TEMP)));
                    *register_freed = true;
                }

                instructions.push(Code::MOVL(
                    RETURN1,
                    STACK,
                    stack_offset(memory_block_position),
                ));
                match load_mode {
                    LoadMode::Release => release_block(RETURN1, instructions),
                    LoadMode::Share => {}
                }

                load_field(
                    Fst,
                    &existing_plus_to_load,
                    RETURN1,
                    FIELDS_PER_BLOCK - 1,
                    instructions,
                );

                load_binders(
                    to_load_next.into(),
                    &existing_plus_rest,
                    RETURN1,
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
    if !to_load.bindings.is_empty() {
        let rest_length = if to_load.bindings.len() <= FIELDS_PER_BLOCK {
            0
        } else {
            to_load.bindings.len() - FIELDS_PER_BLOCK
        };
        let to_load_last = to_load.bindings.split_off(rest_length);

        let mut existing_plus_rest = existing_context.clone();
        existing_plus_rest
            .bindings
            .append(&mut to_load.bindings.clone());

        // tracks whether a register for memory blocks in a spill position has been freed
        let mut register_freed = false;

        load_fields_rest(
            to_load,
            existing_context,
            load_mode,
            &mut register_freed,
            instructions,
        );

        let memory_block = Backend::fresh_temporary(Fst, &existing_plus_rest);

        match memory_block {
            Temporary::Register(memory_block_register) => {
                match load_mode {
                    LoadMode::Release => release_block(memory_block_register, instructions),
                    LoadMode::Share => {}
                }

                load_binders(
                    to_load_last.into(),
                    &existing_plus_rest,
                    memory_block_register,
                    FIELDS_PER_BLOCK,
                    load_mode,
                    instructions,
                );
            }
            Temporary::Spill(memory_block_position) => {
                // free register for memory block if not already done
                if !register_freed {
                    instructions.push(Code::MOVS(RETURN1, STACK, stack_offset(SPILL_TEMP)));
                }

                instructions.push(Code::MOVL(
                    RETURN1,
                    STACK,
                    stack_offset(memory_block_position),
                ));
                match load_mode {
                    LoadMode::Release => release_block(RETURN1, instructions),
                    LoadMode::Share => {}
                }

                load_binders(
                    to_load_last.into(),
                    &existing_plus_rest,
                    RETURN1,
                    FIELDS_PER_BLOCK,
                    load_mode,
                    instructions,
                );

                // restore register freed for memory block
                instructions.push(Code::MOVL(RETURN1, STACK, stack_offset(SPILL_TEMP)));
            }
        }
    }
}

impl Memory<Code, Temporary> for Backend {
    fn erase_block(to_erase: Temporary, instructions: &mut Vec<Code>) {
        #[allow(clippy::vec_init_then_push)]
        fn erase_valid_object(to_erase: Register, instructions: &mut Vec<Code>) {
            let mut then_branch = Vec::with_capacity(3);
            then_branch.push(Code::COMMENT(
                "      ... or add block to lazy free list".to_string(),
            ));
            then_branch.push(Code::MOVS(FREE, to_erase, NEXT_ELEMENT_OFFSET.into()));
            then_branch.push(Code::MOV(FREE, to_erase));

            let mut else_branch = Vec::with_capacity(2);
            else_branch.push(Code::COMMENT(
                "      either decrement refcount ...".to_string(),
            ));
            else_branch.push(Code::ADDIM(
                to_erase,
                REFERENCE_COUNT_OFFSET.into(),
                (-1).into(),
            ));

            if_zero_then_else(
                to_erase,
                Some(REFERENCE_COUNT_OFFSET),
                then_branch,
                else_branch,
                instructions,
            );
        }

        let mut to_skip = Vec::with_capacity(10);
        to_skip.push(Code::COMMENT("      check refcount".to_string()));

        match to_erase {
            Temporary::Register(to_erase_register) => {
                erase_valid_object(to_erase_register, &mut to_skip);
                skip_if_zero(to_erase, to_skip, instructions);
            }
            Temporary::Spill(to_erase_position) => {
                to_skip.push(Code::MOVL(TEMP, STACK, stack_offset(to_erase_position)));
                erase_valid_object(TEMP, &mut to_skip);
                skip_if_zero(to_erase, to_skip, instructions);
            }
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn share_block_n(to_share: Temporary, n: usize, instructions: &mut Vec<Code>) {
        let mut to_skip = Vec::with_capacity(4);
        match to_share {
            Temporary::Register(to_share_register) => {
                to_skip.push(Code::ADDIM(
                    to_share_register,
                    REFERENCE_COUNT_OFFSET.into(),
                    (n as i64).into(),
                ));
                skip_if_zero(to_share, to_skip, instructions);
            }
            Temporary::Spill(to_share_position) => {
                to_skip.push(Code::MOVL(TEMP, STACK, stack_offset(to_share_position)));
                to_skip.push(Code::ADDIM(
                    TEMP,
                    REFERENCE_COUNT_OFFSET.into(),
                    (n as i64).into(),
                ));
                skip_if_zero(to_share, to_skip, instructions);
            }
        }
    }

    fn load(
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
            else_branch.push(Code::ADDIM(
                memory_block,
                Immediate {
                    val: REFERENCE_COUNT_OFFSET,
                },
                Immediate { val: -1 },
            ));
            load_fields(to_load, existing_context, LoadMode::Share, &mut else_branch);

            if_zero_then_else(
                memory_block,
                Some(REFERENCE_COUNT_OFFSET),
                then_branch,
                else_branch,
                instructions,
            );
        }

        if !to_load.bindings.is_empty() {
            let memory_block = Backend::fresh_temporary(Fst, existing_context);

            match memory_block {
                Temporary::Register(memory_block_register) => load_register(
                    memory_block_register,
                    to_load,
                    existing_context,
                    instructions,
                ),
                Temporary::Spill(memory_block_position) => {
                    instructions.push(Code::MOVL(TEMP, STACK, stack_offset(memory_block_position)));
                    load_register(TEMP, to_load, existing_context, instructions);
                }
            }
        }
    }

    fn store(
        mut to_store: TypingContext,
        remaining_context: &TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        if to_store.bindings.is_empty() {
            instructions.push(Code::COMMENT(" nothing to store".to_string()));
            Backend::load_immediate(
                Backend::fresh_temporary(Fst, remaining_context),
                0.into(),
                instructions,
            );
        } else {
            let rest_length = if to_store.bindings.len() <= FIELDS_PER_BLOCK {
                0
            } else {
                to_store.bindings.len() - FIELDS_PER_BLOCK
            };
            let to_store_first = to_store.bindings.split_off(rest_length);

            let mut remaining_plus_rest = remaining_context.clone();
            remaining_plus_rest
                .bindings
                .append(&mut to_store.bindings.clone());

            instructions.push(Code::COMMENT(" allocate memory".to_string()));
            store_values(
                to_store_first.into(),
                &remaining_plus_rest,
                HEAP,
                FIELDS_PER_BLOCK,
                instructions,
            );

            instructions.push(Code::COMMENT(
                "  acquire free block from heap register".to_string(),
            ));
            acquire_block(
                Backend::fresh_temporary(Fst, &remaining_plus_rest),
                instructions,
            );

            store_rest(to_store, remaining_context, instructions);
        }
    }
}
