//! This module implements some basic configurations.

use super::Backend;

use axcut2backend::config::{Config, TemporaryNumber};

use std::fmt;

/// The general-purpose machine registers are represented by their number between `0` and
/// [`REGISTER_NUM`]` - 1`. Since currently no spilling for variables is implemented, registers are
/// the only temporaries.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Register(pub usize);

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X{}", self.0)
    }
}

impl From<usize> for Register {
    fn from(value: usize) -> Self {
        Register(value)
    }
}

/// The number of general-purpose registers. Since some registers are [`RESERVED`], this means that
/// there can be at most 14 variables in the environment. This could be alleviated by spilling more
/// variables to memory.
pub const REGISTER_NUM: usize = 32;

/// The type of immediate integers (this will likely change when more built-in types are
/// supported).
pub type Immediate = i64;

/// The number of reserved registers.
pub const RESERVED: usize = 4;
/// The reserved register which is always zero.
pub const ZERO: Register = Register(0);
/// The reserved scratch register.
pub const TEMP: Register = Register(1);
/// The reserved register containing a heap pointer to an object which we can use directly.
pub const HEAP: Register = Register(2);
/// The reserved register containing a deferred-free-list pointer to objects that still need to be
/// freed.
pub const FREE: Register = Register(3);

/// The first register in which values are returned according to the standard calling convention.
pub const RETURN1: Register = Register(10);
/// The second register in which values are returned according to the standard calling convention.
pub const RETURN2: Register = Register(11);

/// The size of a field slot, i.e., the size of a pointer.
pub const FIELD_SLOT_SIZE: usize = 8;
/// This function calculates a memory address offset from a given logical slot offset into a memory
/// block.
pub const fn address(n: isize) -> Immediate {
    (FIELD_SLOT_SIZE as isize * n) as Immediate
}

/// The number of usable fields per memory block. One additional field is used as a header,
/// containing, for example, the reference count or the link to the next element in a free list.
pub const FIELDS_PER_BLOCK: usize = 3;

/// The address offset within a memory block of the reference count.
pub const REFERENCE_COUNT_OFFSET: Immediate = address(0);

/// The address offset within a memory block of the link to the next element in a free list.
pub const NEXT_ELEMENT_OFFSET: Immediate = address(0);

/// This function calculates the address offset within a memory block of either the first or the
/// second slot of a given field.
/// - `number` determines whether the first or the second slot of a field is needed.
/// - `field` is the logical offset of the field in the memory block.
#[allow(clippy::cast_possible_wrap)]
pub const fn field_offset(number: TemporaryNumber, field: usize) -> Immediate {
    address(2 + 2 * field as isize + number as isize)
}

impl Config<Register, Immediate> for Backend {
    fn i64_to_immediate(number: i64) -> Immediate {
        number
    }

    fn temp() -> Register {
        TEMP
    }

    fn heap() -> Register {
        HEAP
    }

    fn free() -> Register {
        FREE
    }

    fn return1() -> Register {
        RETURN1
    }

    fn return2() -> Register {
        RETURN2
    }

    #[allow(clippy::cast_possible_wrap)]
    fn jump_length(n: usize) -> Immediate {
        4 * n as Immediate
    }
}
