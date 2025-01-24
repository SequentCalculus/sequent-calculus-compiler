use super::code::Code;
use super::config::{
    field_offset, Immediate, Register, FIELDS_PER_BLOCK, FREE, HEAP, NEXT_ELEMENT_OFFSET,
    REFERENCE_COUNT_OFFSET, TEMP, ZERO,
};
use super::Backend;

use axcut::syntax::{Chirality, ContextBinding, TypingContext};
use axcut2backend::{
    config::TemporaryNumber, fresh_labels::fresh_label, memory::Memory, utils::Utils,
};
use TemporaryNumber::{Fst, Snd};

fn skip_if_zero(condition: Register, mut to_skip: Vec<Code>, instructions: &mut Vec<Code>) {
    let fresh_label = format!("lab{}", fresh_label());
    instructions.push(Code::BEQ(condition, ZERO, fresh_label.clone()));
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
    instructions.push(Code::BEQ(condition, ZERO, fresh_label_then.clone()));
    instructions.append(&mut else_branch);
    instructions.push(Code::JAL(ZERO, fresh_label_else.clone()));
    instructions.push(Code::LAB(fresh_label_then));
    instructions.append(&mut then_branch);
    instructions.push(Code::LAB(fresh_label_else));
}

#[allow(clippy::vec_init_then_push)]
fn acquire_block(new_block: Register, additional_temp: Register, instructions: &mut Vec<Code>) {
    fn erase_fields(to_erase: Register, additional_temp: Register, instructions: &mut Vec<Code>) {
        // reversed order in iterator to adhere to Idris implementation
        for offset in (0..FIELDS_PER_BLOCK).rev() {
            instructions.push(Code::COMMENT(format!(
                "#####check child {} for erasure",
                offset + 1
            )));
            instructions.push(Code::LW(
                additional_temp,
                to_erase,
                field_offset(Fst, offset),
            ));
            Backend::erase_block(additional_temp, instructions);
        }
    }

    instructions.push(Code::MV(new_block, HEAP));

    instructions.push(Code::COMMENT(
        "##get next free block into heap register".to_string(),
    ));
    instructions.push(Code::COMMENT(
        "###(1) check linear free list for next block".to_string(),
    ));
    instructions.push(Code::LW(HEAP, HEAP, NEXT_ELEMENT_OFFSET));

    let mut then_branch_free = Vec::with_capacity(2);
    then_branch_free.push(Code::COMMENT(
        "###(3) fall back to bump allocation".to_string(),
    ));
    then_branch_free.push(Code::ADDI(FREE, HEAP, field_offset(Fst, FIELDS_PER_BLOCK)));

    let mut else_branch_free = Vec::with_capacity(64);
    else_branch_free.push(Code::COMMENT("####mark linear free list empty".to_string()));
    else_branch_free.push(Code::SW(ZERO, HEAP, NEXT_ELEMENT_OFFSET));
    else_branch_free.push(Code::COMMENT(
        "####erase children of next block".to_string(),
    ));
    erase_fields(HEAP, additional_temp, &mut else_branch_free);

    let mut then_branch = Vec::with_capacity(64);
    then_branch.push(Code::COMMENT(
        "###(2) check non-linear lazy free list for next block".to_string(),
    ));
    then_branch.push(Code::MV(HEAP, FREE));
    then_branch.push(Code::LW(FREE, FREE, NEXT_ELEMENT_OFFSET));
    if_zero_then_else(FREE, then_branch_free, else_branch_free, &mut then_branch);

    let mut else_branch = Vec::with_capacity(3);
    else_branch.push(Code::COMMENT(
        "####initialize refcount of just acquired block".to_string(),
    ));
    else_branch.push(Code::SW(ZERO, new_block, REFERENCE_COUNT_OFFSET));

    if_zero_then_else(HEAP, then_branch, else_branch, instructions);
}

fn release_block(to_release: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::SW(HEAP, to_release, NEXT_ELEMENT_OFFSET));
    instructions.push(Code::MV(HEAP, to_release));
}

fn store_zero(memory_block: Register, offset: usize, instructions: &mut Vec<Code>) {
    instructions.push(Code::SW(ZERO, memory_block, field_offset(Fst, offset)));
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
) -> Code {
    Code::SW(
        Backend::fresh_temporary(number, context),
        memory_block,
        field_offset(number, offset),
    )
}

fn load_field(
    number: TemporaryNumber,
    context: &TypingContext,
    memory_block: Register,
    offset: usize,
) -> Code {
    Code::LW(
        Backend::fresh_temporary(number, context),
        memory_block,
        field_offset(number, offset),
    )
}

fn store_value(
    to_store: &ContextBinding,
    remaining_context: &TypingContext,
    memory_block: Register,
    offset: usize,
    instructions: &mut Vec<Code>,
) {
    instructions.push(store_field(Snd, remaining_context, memory_block, offset));
    if to_store.chi == Chirality::Ext {
        store_zero(memory_block, offset, instructions);
    } else {
        instructions.push(store_field(Fst, remaining_context, memory_block, offset));
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
    instructions.push(load_field(Snd, existing_context, memory_block, offset));
    if to_load.chi != Chirality::Ext {
        instructions.push(load_field(Fst, existing_context, memory_block, offset));
        match load_mode {
            LoadMode::Release => {}
            LoadMode::Share => {
                Backend::share_block(
                    Backend::fresh_temporary(Fst, existing_context),
                    instructions,
                );
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
    instructions.push(Code::COMMENT("###load values".to_string()));
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

        instructions.push(Code::COMMENT("##store link to previous block".to_string()));
        instructions.push(store_field(
            Fst,
            &remaining_plus_to_store,
            HEAP,
            FIELDS_PER_BLOCK - 1,
        ));

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
            Backend::fresh_temporary(Snd, &remaining_plus_rest),
            instructions,
        );

        store_rest(to_store, remaining_context, instructions);
    }
}

fn load_fields_rest(
    mut to_load: TypingContext,
    existing_context: &TypingContext,
    load_mode: LoadMode,
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

        load_fields_rest(to_load, existing_context, load_mode, instructions);

        let memory_block = Backend::fresh_temporary(Fst, &existing_plus_rest);

        match load_mode {
            LoadMode::Release => {
                instructions.push(Code::COMMENT("###release block".to_string()));
                release_block(memory_block, instructions);
            }
            LoadMode::Share => {}
        }

        instructions.push(Code::COMMENT("###load link to next block".to_string()));
        instructions.push(load_field(
            Fst,
            &existing_plus_to_load,
            memory_block,
            FIELDS_PER_BLOCK - 1,
        ));

        load_binders(
            to_load_next.into(),
            &existing_plus_rest,
            memory_block,
            FIELDS_PER_BLOCK - 1,
            load_mode,
            instructions,
        );
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

        load_fields_rest(to_load, existing_context, load_mode, instructions);

        let memory_block = Backend::fresh_temporary(Fst, &existing_plus_rest);

        match load_mode {
            LoadMode::Release => {
                instructions.push(Code::COMMENT("###release block".to_string()));
                release_block(memory_block, instructions);
            }
            LoadMode::Share => {}
        }

        load_binders(
            to_load_last.into(),
            &existing_plus_rest,
            memory_block,
            FIELDS_PER_BLOCK,
            load_mode,
            instructions,
        );
    }
}

impl Memory<Code, Register> for Backend {
    #[allow(clippy::vec_init_then_push)]
    fn erase_block(to_erase: Register, instructions: &mut Vec<Code>) {
        let mut to_skip = Vec::with_capacity(10);
        to_skip.push(Code::COMMENT("######check refcount".to_string()));
        to_skip.push(Code::LW(TEMP, to_erase, REFERENCE_COUNT_OFFSET));

        let mut then_branch = Vec::with_capacity(3);
        then_branch.push(Code::COMMENT(
            "######... or add block to lazy free list".to_string(),
        ));
        then_branch.push(Code::SW(FREE, to_erase, NEXT_ELEMENT_OFFSET));
        then_branch.push(Code::MV(FREE, to_erase));

        let mut else_branch = Vec::with_capacity(3);
        else_branch.push(Code::COMMENT(
            "######either decrement refcount ...".to_string(),
        ));
        else_branch.push(Code::ADDI(TEMP, TEMP, -1));
        else_branch.push(Code::SW(TEMP, to_erase, REFERENCE_COUNT_OFFSET));

        if_zero_then_else(TEMP, then_branch, else_branch, &mut to_skip);

        skip_if_zero(to_erase, to_skip, instructions);
    }

    #[allow(clippy::vec_init_then_push)]
    #[allow(clippy::cast_possible_wrap)]
    fn share_block_n(to_share: Register, n: usize, instructions: &mut Vec<Code>) {
        let mut to_skip = Vec::with_capacity(4);
        to_skip.push(Code::COMMENT("####increment refcount".to_string()));
        to_skip.push(Code::LW(TEMP, to_share, REFERENCE_COUNT_OFFSET));
        to_skip.push(Code::ADDI(TEMP, TEMP, n as Immediate));
        to_skip.push(Code::SW(TEMP, to_share, REFERENCE_COUNT_OFFSET));
        skip_if_zero(to_share, to_skip, instructions);
    }

    fn load(
        to_load: TypingContext,
        existing_context: &TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        if !to_load.bindings.is_empty() {
            let memory_block = Backend::fresh_temporary(Fst, existing_context);

            instructions.push(Code::COMMENT("#load from memory".to_string()));
            instructions.push(Code::LW(TEMP, memory_block, REFERENCE_COUNT_OFFSET));

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
            else_branch.push(Code::ADDI(TEMP, TEMP, -1));
            else_branch.push(Code::SW(TEMP, memory_block, REFERENCE_COUNT_OFFSET));
            load_fields(to_load, existing_context, LoadMode::Share, &mut else_branch);

            instructions.push(Code::COMMENT("##check refcount".to_string()));
            if_zero_then_else(TEMP, then_branch, else_branch, instructions);
        }
    }

    fn store(
        mut to_store: TypingContext,
        remaining_context: &TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        if to_store.bindings.is_empty() {
            instructions.push(Code::COMMENT("#mark no allocation".to_string()));
            instructions.push(Code::MV(
                Backend::fresh_temporary(Fst, remaining_context),
                ZERO,
            ));
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
                Backend::fresh_temporary(Snd, &remaining_plus_rest),
                instructions,
            );

            store_rest(to_store, remaining_context, instructions);
        }
    }
}
