//! This module implements the details of memory management. We use a lazy reference counting
//! scheme with fixed-size blocks. Each block consists of four fields with two (pointer-sized)
//! slots each. The first field serves as a header whose first slot contains the reference count
//! if the block is in use, or can be used as a link to the next element in a free list otherwise.
//! The other fields can contain the value bound to one variable each. If an object needs more
//! space than fits into one block, several memory blocks are linked together, with the link being
//! in the first slot of the last field. Each block gets its own reference count, so that they can
//! be treated uniformly. The reference count of the head of the linked list stands for the
//! reference count of the whole object and is the only one that is changed during the lifetime of
//! the object. The reference count of all other blocks is always zero (except temporarily during
//! loading), as the only reference to them is link from the  previous block.
//!
//! The number of fields and what fields serve what purpose can be configured in [`super::config`],
//! via [`super::config::FIELDS_PER_BLOCK`], [`super::config::REFERENCE_COUNT_OFFSET`],
//! [`super::config::NEXT_ELEMENT_OFFSET`] and [`super::config::field_offset`].
//!
//! We use two free lists.
//! - Register [`super::config::HEAP`] points to a free list whose blocks can be used immediately
//!   for a newly allocated object. It always contains at least one element. When a block with only
//!   one reference is consumed, it is put onto this list. As its fields are loaded into
//!   temporaries, there is no need to change the reference counts of the objects the fields point
//!   to. We also call this list the linear free list, since objects that are used linearly always
//!   end up on this list.
//! - Register [`super::config::FREE`] points to the lazy free list which can contain blocks that
//!   still need to be freed when used for a newly allocated object. A block that is erased by
//!   dropping the last reference to it is put onto this list, without recursively erasing its
//!   field however.
//!
//! At the beginning of the program, [`super::config::HEAP`] points to the beginning of the
//! contiguous heap which is filled with zeros, and [`super::config::FREE`] points to the address
//! one memory block further. We use bump allocation from this big chunk of unused memory whenever
//! the free lists are empty.

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

/// This function generates code for skipping instructions if the contents of a register is zero.
/// - `condition` is the register compared to zero.
/// - `to_skip` is the list of instructions to be skipped.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn skip_if_zero(condition: Register, mut to_skip: Vec<Code>, instructions: &mut Vec<Code>) {
    let fresh_label = format!("lab{}", fresh_label());
    instructions.push(Code::CMPI(condition, 0.into()));
    instructions.push(Code::BEQ(fresh_label.clone()));
    instructions.append(&mut to_skip);
    instructions.push(Code::LAB(fresh_label));
}

/// This function generates code for executing one of two blocks of instructions depending on
/// whether the contents of a register is zero.
/// - `condition` is the register compared to zero.
/// - `then_branch` is the list of instructions executed if the register contains zero.
/// - `else_branch` is the list of instructions executed if the register does not contain zero.
/// - `instructions` is the list of instructions to which the new instructions are appended.
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

/// This function acquires a memory block for a newly allocated object. To do so, we use the memory
/// block pointed to by [`super::config::HEAP`] and afterwards restore the invariant that
/// [`super::config::HEAP`] always points to a free block of memory, with one of the following
/// possibilities.
/// 1) If the block just acquired has in its first slot a non-zero pointer to another element,
///    i.e., the linear free list is not empty, then that next element is used.
/// 2) Otherwise, if the lazy free list is not empty, which is indicated by the first slot of the
///    block pointed to by [`super::config::FREE`] containing a non-zero pointer to the next block,
///    we use the block currently pointed to by [`super::config::FREE`] for [`super::config::HEAP`]
///    and make [`super::config::FREE`] point to the next block. The fields of the block now
///    pointed to by [`super::config::HEAP`] have to be erased to make the block directly usable.
/// 3) Otherwise, the first slot of the memory block pointed to by [`super::config::FREE`] is zero,
///    which means that the memory block is part of the big chunk of so far unused memory. In this
///    case we fall back to bump allocation from this big chunk.
/// - `new_block` is the temporary into which we acquire the new block.
/// - `instructions` is the list of instructions to which the new instructions are appended.
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

    // we use the block pointed to by heap
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

    // now we restore the invariant
    instructions.push(Code::COMMENT(
        "##get next free block into heap register".to_string(),
    ));
    instructions.push(Code::COMMENT(
        "###(1) check linear free list for next block".to_string(),
    ));
    instructions.push(Code::LDR(HEAP, HEAP, NEXT_ELEMENT_OFFSET));

    // the then branch consists of two branches again, one for possibility 3) in the then branch
    // ...
    let mut then_branch_free = Vec::with_capacity(2);
    //// at this point `HEAP` is the same as `FREE`, now we bump `FREE`
    then_branch_free.push(Code::COMMENT(
        "###(3) fall back to bump allocation".to_string(),
    ));
    then_branch_free.push(Code::ADDI(FREE, HEAP, field_offset(Fst, FIELDS_PER_BLOCK)));

    // ... and one for possibility 2) in the else branch
    let mut else_branch_free = Vec::with_capacity(64);
    //// at this point `HEAP` points to the block which was the first element of the non-empty lazy
    //// free list, so its first field contained a pointer to the next block in that list; we now
    //// store a zero there to indicate that the linear free list does not contain further blocks
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

    // the else branch is executed for possibility 1); since the first slot of the acquired block
    // contained a pointer to another element, we now have to store a zero for the reference count
    // there
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

/// This function generates code for prepending a memory block to the linear free list.
/// - `to_release` is the register pointing to the block.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn release_block(to_release: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::STR(HEAP, to_release, NEXT_ELEMENT_OFFSET));
    instructions.push(Code::MOVR(HEAP, to_release));
}

/// This function generates code for storing a zero into the first slot of some non-header field of
/// a memory block.
/// - `memory_block` is the register pointing to the block.
/// - `offset` is the offset of the field within the memory block.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn store_zero(memory_block: Register, offset: usize, instructions: &mut Vec<Code>) {
    instructions.push(Code::STR(
        Register::XZR,
        memory_block,
        field_offset(Fst, offset),
    ));
}

/// This function generates code for storing a zero into the first slot of the first several
/// non-header fields of a memory block.
/// - `free_fields` is the number of fields to store a zero into.
/// - `memory_block` is the register pointing to the block.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn store_zeros(free_fields: usize, memory_block: Register, instructions: &mut Vec<Code>) {
    for offset in 0..free_fields {
        store_zero(memory_block, offset, instructions);
    }
}

/// This function generates code for storing a value into the first or second slot of some
/// non-header field of a memory block. The value is in the first or second temporary after the
/// given context.
/// - `number` determines whether the first or the second slot is written.
/// - `context` is the given context.
/// - `memory_block` is the register pointing to the block.
/// - `offset` is the offset of the field within the memory block.
/// - `instructions` is the list of instructions to which the new instructions are appended.
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

/// This function generates code for loading a value from the first or second slot of some
/// non-header field of a memory block. The value is loaded into the first or second temporary
/// after the given context.
/// - `number` determines whether the first or the second slot is written.
/// - `context` is the given context.
/// - `memory_block` is the register pointing to the block.
/// - `offset` is the offset of the field within the memory block.
/// - `instructions` is the list of instructions to which the new instructions are appended.
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

/// This function generates code for storing a value into some non-header field of a memory block.
/// The value is in the first and second temporary after the remaining context.
/// - `to_store` is the variable the value is bound to.
/// - `remaining_context` is the remaining context after the store.
/// - `memory_block` is the register pointing to the block.
/// - `offset` is the offset of the field within the memory block.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn store_value(
    to_store: &ContextBinding,
    remaining_context: &TypingContext,
    memory_block: Register,
    offset: usize,
    instructions: &mut Vec<Code>,
) {
    store_field(Snd, remaining_context, memory_block, offset, instructions);
    // values of external types like integers occupy only the second temporary, so we zero the
    // first slot to indicate that there is no pointer to another memory block in this field
    if to_store.chi == Chirality::Ext {
        store_zero(memory_block, offset, instructions);
    } else {
        store_field(Fst, remaining_context, memory_block, offset, instructions);
    }
}

/// This enum encodes whether a memory block whose children are loaded into temporaries has further
/// references to it or not. In the latter case, we can simply release the block. In the former
/// case, we have to share the children as there is now a further reference to them in the
/// temporaries.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum LoadMode {
    Release,
    Share,
}

/// This function generates code for loading a value from some non-header field of a memory block.
/// The value is loaded into the first and second temporary after the existing context.
/// - `to_store` is the variable the value is bound to.
/// - `existing_context` is the existing context before the load.
/// - `memory_block` is the register pointing to the block.
/// - `offset` is the offset of the field within the memory block.
/// - `load_mode` decides whether the memory block the value is loaded from is released and thus
///   whether the loaded value must be shared.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn load_value(
    to_load: &ContextBinding,
    existing_context: &TypingContext,
    memory_block: Register,
    offset: usize,
    load_mode: LoadMode,
    instructions: &mut Vec<Code>,
) {
    load_field(Snd, existing_context, memory_block, offset, instructions);
    // values of external types like integers occupy only the second temporary, so we do not have to
    // load the first slot
    if to_load.chi != Chirality::Ext {
        load_field(Fst, existing_context, memory_block, offset, instructions);
        let register_to_share = match Backend::fresh_temporary(Fst, existing_context) {
            Temporary::Register(register) => register,
            // if the field was loaded to a spill position by `load_field`, its contents are is
            // still in `TEMP` at this point
            Temporary::Spill(_) => TEMP,
        };
        if load_mode == LoadMode::Share {
            // the part of the value loaded into the first temporary might point to memory
            Backend::share_block(Temporary::Register(register_to_share), instructions);
        }
    }
}

/// This function generates code for storing the temporaries of the right-most values of a context
/// into some non-header fields of a memory block. The fields not used are zeroed.
/// - `to_store` is the list of variables the values are bound to before the stores.
/// - `remaining_context` is the remaining context after the stores.
/// - `memory_block` is the register pointing to the block.
/// - `free_fields` is the number of free fields available in the memory block. It must be no less
///   than the length of the list of variables to store.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn store_values(
    mut to_store: TypingContext,
    remaining_context: &TypingContext,
    memory_block: Register,
    mut free_fields: usize,
    instructions: &mut Vec<Code>,
) {
    instructions.push(Code::COMMENT("##store values".to_string()));
    // we store the right-most value in the context into the right-most field first
    while let Some(binding) = to_store.bindings.pop() {
        // the context to the left after this store is the context remaining after all stores...
        let mut remaining_plus_rest = remaining_context.clone();
        // ... plus the context of the stores still pending
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

/// This function generates code for loading several values from some non-header fields of a memory
/// block into temporaries to the right of an existing context. The number of temporaries clobbered
/// to the right of the existing context is two times the number of values loaded.
/// - `to_load` is the list of variables the values are bound to after the loads.
/// - `existing_context` is the existing context before the loads.
/// - `memory_block` is the register pointing to the block. It must not be one of the temporaries
///   clobbered by the code this function generates, except the left-most one.
/// - `free_fields` is the number of free fields available in the memory block. It must be no less
///   than the length of the list of variables to load.
/// - `load_mode` decides whether the memory block the values are loaded from is released and thus
///   whether the loaded values must be shared.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn load_values(
    mut to_load: TypingContext,
    existing_context: &TypingContext,
    memory_block: Register,
    mut free_fields: usize,
    load_mode: LoadMode,
    instructions: &mut Vec<Code>,
) {
    instructions.push(Code::COMMENT("###load values".to_string()));
    // we load the right-most field into the temporaries of the right-most variable in the context
    // first; this allows the memory block to be located in the left-most temporary after the
    // context existing before the loads
    while let Some(binding) = to_load.bindings.pop() {
        // the context to the left of this load is the existing context before the loads ...
        let mut existing_plus_rest = existing_context.clone();
        // .. plus the context for the loads still pending
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

/// This enum encodes whether a memory block is the last one in a list of linked blocks or not.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum BlockPosition {
    Last = 0,
    Other = 1,
}

/// This function generates code for storing the temporaries of the right-most values of a context
/// into memory. If more than one memory block is needed, several blocks are linked together. The
/// link to the next block is stored into the first slot of the last field of each block. The
/// pointer to the new memory into which the values are stored will be put into the first temporary
/// after the remaining context.
/// - `to_store` is the list of variables the values are bound to before the stores.
/// - `remaining_context` is the remaining context after the stores.
/// - `block_position` determines whether the block into which we store the next variables is the
///   last one in the linked list. This should be [`BlockPosition::Last`] in non-recursive calls of
///   this function, since we store the right-most values into the last block first.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn store_fields(
    mut to_store: TypingContext,
    remaining_context: &TypingContext,
    block_position: BlockPosition,
    instructions: &mut Vec<Code>,
) {
    if !to_store.bindings.is_empty() {
        // the full context is the context remaining after all stores...
        let mut remaining_plus_to_store = remaining_context.clone();
        // ... plus the context of the stores
        remaining_plus_to_store
            .bindings
            .append(&mut to_store.bindings.clone());

        // if we do not currently store the last block, we have to store a link to the next block
        if block_position == BlockPosition::Other {
            instructions.push(Code::COMMENT("##store link to previous block".to_string()));
            store_field(
                Fst,
                &remaining_plus_to_store,
                HEAP,
                FIELDS_PER_BLOCK - 1,
                instructions,
            );
        }

        // we can store at most `FIELDS_PER_BLOCK` variables in the last memory block, or
        // `FIELDS_PER_BLOCK - 1` in all other blocks, since in the latter case the last field is
        // used for the link
        let rest_length = if to_store.bindings.len() <= FIELDS_PER_BLOCK - block_position as usize {
            0
        } else {
            to_store.bindings.len() - (FIELDS_PER_BLOCK - block_position as usize)
        };
        // we store the last variables first; if we need yet more memory, we have to link further
        // blocks
        let to_store_next = to_store.bindings.split_off(rest_length);

        // the context to the left after these stores is the context remaining after all stores...
        let mut remaining_plus_rest = remaining_context.clone();
        // ... plus the context of the stores still pending
        remaining_plus_rest
            .bindings
            .append(&mut to_store.bindings.clone());

        if block_position == BlockPosition::Last {
            instructions.push(Code::COMMENT("#allocate memory".to_string()));
        }
        store_values(
            to_store_next.into(),
            &remaining_plus_rest,
            HEAP,
            FIELDS_PER_BLOCK - block_position as usize,
            instructions,
        );

        instructions.push(Code::COMMENT(
            "##acquire free block from heap register".to_string(),
        ));
        // this puts the pointer to the memory block for the variables just stored into the first
        // free temporary after the remaining context
        acquire_block(
            Backend::fresh_temporary(Fst, &remaining_plus_rest),
            instructions,
        );

        store_fields(
            to_store,
            remaining_context,
            BlockPosition::Other,
            instructions,
        );
    } else {
        // if no memory is needed at all, we put a zero in the first temporary of the variable to
        // indicate this
        if block_position == BlockPosition::Last {
            instructions.push(Code::COMMENT("#mark no allocation".to_string()));
            Backend::load_immediate(
                Backend::fresh_temporary(Fst, remaining_context),
                0.into(),
                instructions,
            );
        }
    }
}

/// This function generates code for loading several values from memory into temporaries to the
/// right of an existing context. The pointer to the memory from which the values are to be
/// loaded is expected to be in the first temporary after the existing context. The memory is
/// expected to be linked together in a (possibly singleton) list, with the links stored in the
/// first slot of the last field within the blocks.
/// - `to_load` is the list of variables the values are bound to after the loads.
/// - `existing_context` is the existing context before the loads.
/// - `block_position` determines whether the block which we consider next for loading variables is
///   the last one in the linked list. This should be [`BlockPosition::Last`] in non-recursive calls
///   of this function, since we consider the blocks last-to-first.
/// - `load_mode` decides whether the memory blocks the values are loaded from are released and
///   thus whether the loaded values must be shared.
/// - `register_freed` tracks whether a register for memory blocks in a spill position has been
///   freed. This should be [`false`] in non-recursive calls of this function, since the register
///   is freed by need in a recursive call.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn load_fields(
    mut to_load: TypingContext,
    existing_context: &TypingContext,
    block_position: BlockPosition,
    load_mode: LoadMode,
    register_freed: &mut bool,
    instructions: &mut Vec<Code>,
) {
    if !to_load.bindings.is_empty() {
        // the context after the all loads is the existing context before all loads...
        let mut existing_plus_to_load = existing_context.clone();
        // ... plus the context of the all loads
        existing_plus_to_load
            .bindings
            .append(&mut to_load.bindings.clone());

        // there can be at most `FIELDS_PER_BLOCK` variables in the last memory block, or
        // `FIELDS_PER_BLOCK - 1` in all other blocks, since in the latter case the last field is
        // used for the link
        let rest_length = if to_load.bindings.len() <= FIELDS_PER_BLOCK - block_position as usize {
            0
        } else {
            to_load.bindings.len() - (FIELDS_PER_BLOCK - block_position as usize)
        };
        // we load the values in the current block only after the previous ones
        let to_load_next = to_load.bindings.split_off(rest_length);

        // the context to the left before these loads is the existing context before all loads ...
        let mut existing_plus_rest = existing_context.clone();
        // .. plus the context for the loads done before
        existing_plus_rest
            .bindings
            .append(&mut to_load.bindings.clone());

        // we load the previous fields first; this puts the link to the block for the next loads
        // into the first temporary after the context
        load_fields(
            to_load,
            existing_context,
            BlockPosition::Other,
            load_mode,
            register_freed,
            instructions,
        );

        // the pointer to the memory block from which to load the values now is in the first
        // temporary after the context, either because it was already there before the first call
        // of this function, or because the above recursive call loaded it there
        let memory_block = Backend::fresh_temporary(Fst, &existing_plus_rest);

        match memory_block {
            Temporary::Register(memory_block_register) => {
                if load_mode == LoadMode::Release {
                    instructions.push(Code::COMMENT("###release block".to_string()));
                    release_block(memory_block_register, instructions);
                }

                // if we do not currently load the last block, we have to load the link to the next block;
                // we have to load the link first, since loading the values will clobber the temporary
                // containing the pointer to the memory block
                if block_position == BlockPosition::Other {
                    instructions.push(Code::COMMENT("###load link to next block".to_string()));
                    load_field(
                        Fst,
                        &existing_plus_to_load,
                        memory_block_register,
                        FIELDS_PER_BLOCK - 1,
                        instructions,
                    );
                }

                load_values(
                    to_load_next.into(),
                    &existing_plus_rest,
                    memory_block_register,
                    FIELDS_PER_BLOCK - block_position as usize,
                    load_mode,
                    instructions,
                );
            }
            Temporary::Spill(memory_block_position) => {
                // the first time a memory block is in a spill position, we free a register for it
                // and only restore the register after the last loads (at the end of the first call
                // of this function), since all memory blocks after this one will also be in a
                // spill position
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

                // if we do not currently load the last block, we have to load the link to the next block;
                // we have to load the link first, since loading the values will clobber the temporary
                // containing the pointer to the memory block
                if block_position == BlockPosition::Other {
                    instructions.push(Code::COMMENT("###load link to next block".to_string()));
                    load_field(
                        Fst,
                        &existing_plus_to_load,
                        TEMPORARY_TEMP,
                        FIELDS_PER_BLOCK - 1,
                        instructions,
                    );
                }

                load_values(
                    to_load_next.into(),
                    &existing_plus_rest,
                    TEMPORARY_TEMP,
                    FIELDS_PER_BLOCK - block_position as usize,
                    load_mode,
                    instructions,
                );

                // after the last loads, we can restore the evacuated register
                if block_position == BlockPosition::Last {
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
        to_store: TypingContext,
        remaining_context: &TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        store_fields(
            to_store,
            remaining_context,
            BlockPosition::Last,
            instructions,
        );
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
            // tracks whether a register for memory blocks in a spill position has been freed
            let mut register_freed = false;

            // the then branch corresponds to the reference count of the object whose memory we
            // load being zero, so we can release the memory
            let mut then_branch = Vec::new();
            then_branch.push(Code::COMMENT(
                "##... or release blocks onto linear free list when loading".to_string(),
            ));
            load_fields(
                to_load.clone(),
                existing_context,
                BlockPosition::Last,
                LoadMode::Release,
                &mut register_freed,
                &mut then_branch,
            );

            // reset for call of `load_fields` in else branch
            register_freed = false;

            // the else branch corresponds to the reference count of the object whose memory we
            // load being greater than zero, so we decrement the reference count and share the
            // pointers to the children
            let mut else_branch = Vec::new();
            else_branch.push(Code::COMMENT(
                "##either decrement refcount and share children...".to_string(),
            ));
            // we expect the reference count to be in [`super::config::TEMP2`]
            else_branch.push(Code::SUBI(TEMP2, TEMP2, 1.into()));
            else_branch.push(Code::STR(TEMP2, memory_block, REFERENCE_COUNT_OFFSET));
            load_fields(
                to_load,
                existing_context,
                BlockPosition::Last,
                LoadMode::Share,
                &mut register_freed,
                &mut else_branch,
            );

            instructions.push(Code::COMMENT("##check refcount".to_string()));
            // we expect the reference count to be in [`super::config::TEMP2`]
            if_zero_then_else(TEMP2, then_branch, else_branch, instructions);
        }

        if !to_load.bindings.is_empty() {
            let memory_block = Backend::fresh_temporary(Fst, existing_context);

            instructions.push(Code::COMMENT("#load from memory".to_string()));
            match memory_block {
                Temporary::Register(memory_block_register) => {
                    // we put the reference count into [`super::config::TEMP2`], since
                    // `load_register` expects it to be there
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
                    // we put the reference count into [`super::config::TEMP2`], since
                    // `load_register` expects it to be there
                    instructions.push(Code::LDR(TEMP2, TEMP, REFERENCE_COUNT_OFFSET));
                    load_register(TEMP, to_load, existing_context, instructions);
                }
            }
        }
    }
}
