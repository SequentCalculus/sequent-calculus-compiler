use super::Backend;
use super::code::Code;
use super::config::{
    FIELDS_PER_BLOCK, FREE, HEAP, NEXT_ELEMENT_OFFSET, REFERENCE_COUNT_OFFSET, Register,
    SPILL_TEMP, TEMP, TEMP2, TEMPORARY_TEMP, Temporary, field_offset, stack_offset,
};

use TemporaryNumber::{Fst, Snd};
use axcut::syntax::{Chirality, ContextBinding, TypingContext};
use axcut2backend::{
    code::Instructions, config::TemporaryNumber, fresh_labels::fresh_label, memory::Memory,
    utils::Utils,
};

fn skip_if_zero(condition: Register, mut to_skip: Vec<Code>, instructions: &mut Vec<Code>) {
    let fresh_label = format!("lab{}", fresh_label());
    instructions.push(Code::CMPI(condition, 0.into()));
    instructions.push(Code::BEQ(fresh_label.clone()));
    instructions.append(&mut to_skip);
    instructions.push(Code::LAB(fresh_label));
}

fn if_zero_then_else(
    condition: Register,
    mut then_branch: Vec<Code>,
    mut else_branch: Vec<Code>,
    instructions: &mut Vec<Code>,
) {
    let fresh_label_then = format!("lab{}", fresh_label());
    let fresh_label_else = format!("lab{}", fresh_label());

    instructions.push(Code::CMPI(condition, 0.into()));
    instructions.push(Code::BEQ(fresh_label_then.clone()));
    instructions.append(&mut else_branch);
    instructions.push(Code::B(fresh_label_else.clone()));
    instructions.push(Code::LAB(fresh_label_then));
    instructions.append(&mut then_branch);
    instructions.push(Code::LAB(fresh_label_else));
}

#[allow(clippy::vec_init_then_push)]
fn acquire_block(new_block: Temporary, instructions: &mut Vec<Code>) {
    fn erase_fields(to_erase: Register, instructions: &mut Vec<Code>) {
        for offset in 0..FIELDS_PER_BLOCK {
            instructions.push(Code::COMMENT(format!(
                "#####check child {} for erasure",
                offset + 1
            )));
            instructions.push(Code::LDR(TEMP, to_erase, field_offset(Fst, offset)));
            Backend::erase_block(Temporary::Register(TEMP), instructions);
        }
    }

    match new_block {
        Temporary::Register(new_block_register) => {
            instructions.push(Code::MOVR(new_block_register, HEAP));
        }
        Temporary::Spill(new_block_position) => {
            // this moves the memory block both to `TEMP` and to its spill position for better
            // performance in the fast path, but executes the first instruction unnecessarily in the
            // slow path
            instructions.push(Code::MOVR(TEMP, HEAP));
            instructions.push(Code::STR(
                HEAP,
                Register::SP,
                stack_offset(new_block_position),
            ));
        }
    }

    instructions.push(Code::COMMENT(
        "##get next free block into heap register".to_string(),
    ));
    instructions.push(Code::COMMENT(
        "###(1) check linear free list for next block".to_string(),
    ));
    instructions.push(Code::LDR(HEAP, HEAP, NEXT_ELEMENT_OFFSET));

    let mut then_branch_free = Vec::with_capacity(2);
    then_branch_free.push(Code::COMMENT(
        "###(3) fall back to bump allocation".to_string(),
    ));
    then_branch_free.push(Code::ADDI(FREE, HEAP, field_offset(Fst, FIELDS_PER_BLOCK)));

    let mut else_branch_free = Vec::with_capacity(64);
    else_branch_free.push(Code::COMMENT("####mark linear free list empty".to_string()));
    else_branch_free.push(Code::STR(Register::XZR, HEAP, NEXT_ELEMENT_OFFSET));
    else_branch_free.push(Code::COMMENT(
        "####erase children of next block".to_string(),
    ));
    erase_fields(HEAP, &mut else_branch_free);

    let mut then_branch = Vec::with_capacity(64);
    then_branch.push(Code::COMMENT(
        "###(2) check non-linear lazy free list for next block".to_string(),
    ));
    then_branch.push(Code::MOVR(HEAP, FREE));
    then_branch.push(Code::LDR(FREE, FREE, NEXT_ELEMENT_OFFSET));
    if_zero_then_else(FREE, then_branch_free, else_branch_free, &mut then_branch);

    let mut else_branch = Vec::with_capacity(3);
    else_branch.push(Code::COMMENT(
        "####initialize refcount of just acquired block".to_string(),
    ));
    match new_block {
        Temporary::Register(new_block_register) => {
            else_branch.push(Code::STR(
                Register::XZR,
                new_block_register,
                REFERENCE_COUNT_OFFSET,
            ));
        }
        Temporary::Spill(_new_block_position) => {
            // this instruction would be needed without the above optimization for the fast path
            //else_branch.push(Code::LDR(TEMP, Register::SP, stack_offset(new_block_position)));
            else_branch.push(Code::STR(Register::XZR, TEMP, REFERENCE_COUNT_OFFSET));
        }
    }

    if_zero_then_else(HEAP, then_branch, else_branch, instructions);
}

fn release_block(to_release: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::STR(HEAP, to_release, NEXT_ELEMENT_OFFSET));
    instructions.push(Code::MOVR(HEAP, to_release));
}

fn store_zero(memory_block: Register, offset: usize, instructions: &mut Vec<Code>) {
    instructions.push(Code::STR(
        Register::XZR,
        memory_block,
        field_offset(Fst, offset),
    ));
}

fn store_zeros(free_fields: usize, memory_block: Register, instructions: &mut Vec<Code>) {
    for offset in 0..free_fields {
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
        Temporary::Register(register) => instructions.push(Code::STR(
            register,
            memory_block,
            field_offset(number, offset),
        )),
        Temporary::Spill(position) => {
            instructions.push(Code::LDR(TEMP, Register::SP, stack_offset(position)));
            instructions.push(Code::STR(TEMP, memory_block, field_offset(number, offset)));
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
            instructions.push(Code::LDR(
                register,
                memory_block,
                field_offset(number, offset),
            ));
        }
        Temporary::Spill(position) => {
            instructions.push(Code::LDR(TEMP, memory_block, field_offset(number, offset)));
            instructions.push(Code::STR(TEMP, Register::SP, stack_offset(position)));
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum LoadMode {
    Release,
    Share,
}

fn load_value(
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
            // if field was loaded to spill position by `load_field`, it is still in `TEMP` here
            Temporary::Spill(_) => TEMP,
        };
        if load_mode == LoadMode::Share {
            Backend::share_block(Temporary::Register(register_to_share), instructions);
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
    instructions.push(Code::COMMENT("##store values".to_string()));
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
        instructions.push(Code::COMMENT("##mark unused fields with null".to_string()));
    }
    store_zeros(free_fields, memory_block, instructions);
}

fn load_values(
    mut to_load: TypingContext,
    existing_context: &TypingContext,
    memory_block: Register,
    mut free_fields: usize,
    load_mode: LoadMode,
    instructions: &mut Vec<Code>,
) {
    instructions.push(Code::COMMENT("###load values".to_string()));
    while let Some(binding) = to_load.bindings.pop() {
        let mut existing_plus_rest = existing_context.clone();
        existing_plus_rest
            .bindings
            .append(&mut to_load.bindings.clone());

        load_value(
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

        instructions.push(Code::COMMENT("##store link to previous block".to_string()));
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

        instructions.push(Code::COMMENT(
            "##acquire free block from heap register".to_string(),
        ));
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
                if load_mode == LoadMode::Release {
                    instructions.push(Code::COMMENT("###release block".to_string()));
                    release_block(memory_block_register, instructions);
                }

                instructions.push(Code::COMMENT("###load link to next block".to_string()));
                load_field(
                    Fst,
                    &existing_plus_to_load,
                    memory_block_register,
                    FIELDS_PER_BLOCK - 1,
                    instructions,
                );

                load_values(
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
                    instructions.push(Code::COMMENT(
                        "###evacuate additional scratch register for memory block".to_string(),
                    ));
                    instructions.push(Code::STR(
                        TEMPORARY_TEMP,
                        Register::SP,
                        stack_offset(SPILL_TEMP),
                    ));
                    *register_freed = true;
                }

                instructions.push(Code::LDR(
                    TEMPORARY_TEMP,
                    Register::SP,
                    stack_offset(memory_block_position),
                ));
                if load_mode == LoadMode::Release {
                    instructions.push(Code::COMMENT("###release block".to_string()));
                    release_block(TEMPORARY_TEMP, instructions);
                }

                instructions.push(Code::COMMENT("###load link to next block".to_string()));
                load_field(
                    Fst,
                    &existing_plus_to_load,
                    TEMPORARY_TEMP,
                    FIELDS_PER_BLOCK - 1,
                    instructions,
                );

                load_values(
                    to_load_next.into(),
                    &existing_plus_rest,
                    TEMPORARY_TEMP,
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
                if load_mode == LoadMode::Release {
                    instructions.push(Code::COMMENT("###release block".to_string()));
                    release_block(memory_block_register, instructions);
                }

                load_values(
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
                    instructions.push(Code::COMMENT(
                        "###evacuate additional scratch register for memory block".to_string(),
                    ));
                    instructions.push(Code::STR(
                        TEMPORARY_TEMP,
                        Register::SP,
                        stack_offset(SPILL_TEMP),
                    ));
                }

                instructions.push(Code::LDR(
                    TEMPORARY_TEMP,
                    Register::SP,
                    stack_offset(memory_block_position),
                ));
                if load_mode == LoadMode::Release {
                    instructions.push(Code::COMMENT("###release block".to_string()));
                    release_block(TEMPORARY_TEMP, instructions);
                }

                load_values(
                    to_load_last.into(),
                    &existing_plus_rest,
                    TEMPORARY_TEMP,
                    FIELDS_PER_BLOCK,
                    load_mode,
                    instructions,
                );

                instructions.push(Code::COMMENT("###restore evacuated register".to_string()));
                instructions.push(Code::LDR(
                    TEMPORARY_TEMP,
                    Register::SP,
                    stack_offset(SPILL_TEMP),
                ));
            }
        }
    }
}

impl Memory<Code, Temporary> for Backend {
    #[allow(clippy::vec_init_then_push)]
    fn erase_block(to_erase: Temporary, instructions: &mut Vec<Code>) {
        #[allow(clippy::vec_init_then_push)]
        fn erase_valid_object(to_erase: Register, instructions: &mut Vec<Code>) {
            let mut then_branch = Vec::with_capacity(3);
            then_branch.push(Code::COMMENT(
                "######... or add block to lazy free list".to_string(),
            ));
            then_branch.push(Code::STR(FREE, to_erase, NEXT_ELEMENT_OFFSET));
            then_branch.push(Code::MOVR(FREE, to_erase));

            let mut else_branch = Vec::with_capacity(3);
            else_branch.push(Code::COMMENT(
                "######either decrement refcount ...".to_string(),
            ));
            else_branch.push(Code::SUBI(TEMP2, TEMP2, 1.into()));
            else_branch.push(Code::STR(TEMP2, to_erase, REFERENCE_COUNT_OFFSET));

            if_zero_then_else(TEMP2, then_branch, else_branch, instructions);
        }

        let mut to_skip = Vec::with_capacity(10);
        to_skip.push(Code::COMMENT("######check refcount".to_string()));

        match to_erase {
            Temporary::Register(to_erase_register) => {
                to_skip.push(Code::LDR(TEMP2, to_erase_register, REFERENCE_COUNT_OFFSET));
                erase_valid_object(to_erase_register, &mut to_skip);
                skip_if_zero(to_erase_register, to_skip, instructions);
            }
            Temporary::Spill(to_erase_position) => {
                instructions.push(Code::LDR(
                    TEMP,
                    Register::SP,
                    stack_offset(to_erase_position),
                ));
                to_skip.push(Code::LDR(TEMP2, TEMP, REFERENCE_COUNT_OFFSET));
                erase_valid_object(TEMP, &mut to_skip);
                skip_if_zero(TEMP, to_skip, instructions);
            }
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn share_block_n(to_share: Temporary, n: usize, instructions: &mut Vec<Code>) {
        let mut to_skip = Vec::with_capacity(5);
        to_skip.push(Code::COMMENT("####increment refcount".to_string()));

        match to_share {
            Temporary::Register(to_share_register) => {
                to_skip.push(Code::LDR(TEMP2, to_share_register, REFERENCE_COUNT_OFFSET));
                to_skip.push(Code::ADDI(TEMP2, TEMP2, (n as i64).into()));
                to_skip.push(Code::STR(TEMP2, to_share_register, REFERENCE_COUNT_OFFSET));
                skip_if_zero(to_share_register, to_skip, instructions);
            }
            Temporary::Spill(to_share_position) => {
                instructions.push(Code::LDR(
                    TEMP,
                    Register::SP,
                    stack_offset(to_share_position),
                ));
                to_skip.push(Code::LDR(TEMP2, TEMP, REFERENCE_COUNT_OFFSET));
                to_skip.push(Code::ADDI(TEMP2, TEMP2, (n as i64).into()));
                to_skip.push(Code::STR(TEMP2, TEMP, REFERENCE_COUNT_OFFSET));
                skip_if_zero(TEMP, to_skip, instructions);
            }
        }
    }

    fn store(
        mut to_store: TypingContext,
        remaining_context: &TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        if to_store.bindings.is_empty() {
            instructions.push(Code::COMMENT("#mark no allocation".to_string()));
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

            instructions.push(Code::COMMENT("#allocate memory".to_string()));
            store_values(
                to_store_first.into(),
                &remaining_plus_rest,
                HEAP,
                FIELDS_PER_BLOCK,
                instructions,
            );

            instructions.push(Code::COMMENT(
                "##acquire free block from heap register".to_string(),
            ));
            acquire_block(
                Backend::fresh_temporary(Fst, &remaining_plus_rest),
                instructions,
            );

            store_rest(to_store, remaining_context, instructions);
        }
    }

    fn load(
        to_load: TypingContext,
        existing_context: &TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        #[allow(clippy::vec_init_then_push)]
        fn load_register(
            memory_block: Register,
            to_load: TypingContext,
            existing_context: &TypingContext,
            instructions: &mut Vec<Code>,
        ) {
            let mut then_branch = Vec::new();
            then_branch.push(Code::COMMENT(
                "##... or release blocks onto linear free list when loading".to_string(),
            ));
            load_fields(
                to_load.clone(),
                existing_context,
                LoadMode::Release,
                &mut then_branch,
            );

            let mut else_branch = Vec::new();
            else_branch.push(Code::COMMENT(
                "##either decrement refcount and share children...".to_string(),
            ));
            else_branch.push(Code::SUBI(TEMP2, TEMP2, 1.into()));
            else_branch.push(Code::STR(TEMP2, memory_block, REFERENCE_COUNT_OFFSET));
            load_fields(to_load, existing_context, LoadMode::Share, &mut else_branch);

            instructions.push(Code::COMMENT("##check refcount".to_string()));
            if_zero_then_else(TEMP2, then_branch, else_branch, instructions);
        }

        if !to_load.bindings.is_empty() {
            let memory_block = Backend::fresh_temporary(Fst, existing_context);

            instructions.push(Code::COMMENT("#load from memory".to_string()));
            match memory_block {
                Temporary::Register(memory_block_register) => {
                    instructions.push(Code::LDR(
                        TEMP2,
                        memory_block_register,
                        REFERENCE_COUNT_OFFSET,
                    ));
                    load_register(
                        memory_block_register,
                        to_load,
                        existing_context,
                        instructions,
                    )
                }
                Temporary::Spill(memory_block_position) => {
                    instructions.push(Code::LDR(
                        TEMP,
                        Register::SP,
                        stack_offset(memory_block_position),
                    ));
                    instructions.push(Code::LDR(TEMP2, TEMP, REFERENCE_COUNT_OFFSET));
                    load_register(TEMP, to_load, existing_context, instructions);
                }
            }
        }
    }
}
