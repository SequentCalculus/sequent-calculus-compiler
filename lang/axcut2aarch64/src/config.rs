//! This module implements some basic configurations.

use std::ops::Not;

use super::Backend;

use axcut2backend::config::{Config, TemporaryNumber};

use printer::{Print, theme::ThemeExt};

/// The general-purpose machine registers are represented by a number between `0` and
/// [`REGISTER_NUM`], since we do not use register `X18` as it is reserved on some platforms (e.g.,
/// MacOS). The special stack pointer and zero register are separate.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Register {
    /// General purpose 64-Bit register.
    X(usize),
    /// Stack pointer.
    SP,
    /// Zero register.
    XZR,
}

impl Print for Register {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        // Some platforms (e.g., MacOS) reserves register X18, so we cannot use it at all.
        match self {
            Register::X(r) if *r < 18 => alloc.ctor(&format!("X{r}")),
            Register::X(r) => alloc.ctor(&format!("X{}", r + 1)),
            Register::SP => alloc.ctor("SP"),
            Register::XZR => alloc.ctor("XZR"),
        }
    }
}

impl From<usize> for Register {
    fn from(value: usize) -> Self {
        Register::X(value)
    }
}

/// The number of general-purpose registers. Since some registers are [`RESERVED`], this means that
/// there can be at most 13 variables in the environment. This is alleviated by spilling more
/// variables to memory.
pub const REGISTER_NUM: usize = 30;

/// The type of immediate integers (this will likely change when more built-in types are
/// supported).
#[derive(Debug, Copy, Clone)]
pub struct Immediate {
    pub val: i64,
}

impl From<i64> for Immediate {
    fn from(value: i64) -> Self {
        Immediate { val: value }
    }
}

impl Not for Immediate {
    type Output = Immediate;

    fn not(self) -> Self::Output {
        Immediate { val: !self.val }
    }
}

impl Print for Immediate {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.typ(&format!("{}", self.val))
    }
}

/// The number of reserved registers.
pub const RESERVED: usize = 4;
/// The first reserved scratch register.
pub const TEMP: Register = Register::X(2);
/// The second reserved scratch register.
pub const TEMP2: Register = Register::X(3);
/// The reserved register containing a heap pointer to an object which we can use directly.
pub const HEAP: Register = Register::X(0);
/// The reserved register containing a deferred-free-list pointer to objects that still need to be
/// freed.
pub const FREE: Register = Register::X(1);

/// The first register in which values are returned according to the standard calling convention.
pub const RETURN1: Register = Register::X(0);
/// The second register in which values are returned according to the standard calling convention.
pub const RETURN2: Register = Register::X(1);

/// The variables that do not fit ito registers are spilled to memory. Each spill spot is
/// represented by a number between `0` and [`SPILL_NUM`]` - 1`. As we do not make use of the stack
/// otherwise, we use it for spilling.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Spill(pub usize);

/// The number of spill positions. Since some spots are [`RESERVED_SPILLS`], this means that there
/// can be at most 127 variables in spill positions.
pub const SPILL_NUM: usize = 256;
/// The amount of memory needed for spills.
#[allow(clippy::cast_possible_wrap)]
pub const SPILL_SPACE: i64 = SPILL_NUM as i64 * 8;

/// The number of reserved spill spots.
pub const RESERVED_SPILLS: usize = 1;
/// The reserved scratch spill spot.
pub const SPILL_TEMP: Spill = Spill(0);
/// The temporary scratch register which can be evacuated to [`SPILL_TEMP`] if need be.
pub const TEMPORARY_TEMP: Register = Register::X(10);

/// This functions calculates the offset from the stack pointer for a given spill position.
#[allow(clippy::cast_possible_wrap)]
pub const fn stack_offset(position: Spill) -> Immediate {
    Immediate {
        val: SPILL_SPACE - (8 * (position.0 as i64 + 1)),
    }
}

/// Temporaries are either registers or spill spots. There can be at most 140 variables live in the
/// environment. This can be adapted via [`SPILL_NUM`].
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Temporary {
    Register(Register),
    Spill(Spill),
}

/// The size of a field slot, i.e., the size of a pointer.
pub const FIELD_SLOT_SIZE: usize = 8;
/// This function calculates a memory address offset from a given logical slot offset into a memory
/// block.
pub const fn address(n: isize) -> Immediate {
    Immediate {
        val: (FIELD_SLOT_SIZE as isize * n) as i64,
    }
}

/// The number of usable fields per memory block. One additional field is used as a header,
/// containing, for example, the reference count or the link to the next element in a free list.
pub const FIELDS_PER_BLOCK: usize = 3;

/// The address offset within a memory block of the reference count.
pub const REFERENCE_COUNT_OFFSET: Immediate = address(0);

/// The address offset within a memory block of the link to the next element in a free list.
pub const NEXT_ELEMENT_OFFSET: Immediate = address(0);

/// This function calculates the address offset within a memory block of either the first or the
/// second slot of a given field. The very first field of a memory block serves as a header and is
/// hence always added to the offset.
/// - `number` determines whether the first or the second slot of a field is needed.
/// - `field` is the logical offset of the field in the memory block. It must be between `0` and
///   [`FIELDS_PER_BLOCK`].
#[allow(clippy::cast_possible_wrap)]
pub const fn field_offset(number: TemporaryNumber, i: usize) -> Immediate {
    address(2 + 2 * i as isize + number as isize)
}

/// The number of the first caller-save register to be evacuated during a function call according
/// to the standard calling convention. We do not have to save registers `X2` and `X3` as they are
/// our scratch registers [`TEMP`] and [`TEMP2`].
pub const CALLER_SAVE_FIRST: usize = 4;
/// The number of the last caller-save register to be evacuated during a function call according to
/// the standard calling convention.
pub const CALLER_SAVE_LAST: usize = 17;

impl Config<Temporary, Immediate> for Backend {
    fn i64_to_immediate(number: i64) -> Immediate {
        number.into()
    }

    fn temp() -> Temporary {
        Temporary::Register(TEMP)
    }

    fn heap() -> Temporary {
        Temporary::Register(HEAP)
    }

    fn free() -> Temporary {
        Temporary::Register(FREE)
    }

    fn return1() -> Temporary {
        Temporary::Register(RETURN1)
    }

    fn return2() -> Temporary {
        Temporary::Register(RETURN2)
    }

    #[allow(clippy::cast_possible_wrap)]
    fn jump_length(n: usize) -> Immediate {
        (4 * n as i64).into()
    }
}
