use std::ops::Not;

use super::Backend;

use axcut2backend::config::{Config, TemporaryNumber};

use printer::{theme::ThemeExt, Print};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Register {
    /// General purpose 64-Bit register
    X(usize),
    /// Stack pointer
    SP,
    /// Zero register
    XZR,
}

// MacOS (for example) reserves register X18, so we cannot use it at all.
impl Print for Register {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
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

pub const REGISTER_NUM: usize = 30;

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

// X2 and X3 are used for our purposes
// X0 is a heap pointer to an object which we can directly overwrite AND the first part of the return value
// X1 is a deferred-free-list pointer to objects which we have to free AND the second part of the return value
pub const RESERVED: usize = 4;

pub const TEMP: Register = Register::X(2);
pub const TEMP2: Register = Register::X(3);
pub const HEAP: Register = Register::X(0);
pub const FREE: Register = Register::X(1);

pub const RETURN1: Register = Register::X(0);
pub const RETURN2: Register = Register::X(1);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Spill(pub usize);

pub const SPILL_NUM: usize = 256;
#[allow(clippy::cast_possible_wrap)]
pub const SPILL_SPACE: i64 = SPILL_NUM as i64 * 8;

// one spot is used for our purposes
pub const RESERVED_SPILLS: usize = 1;

pub const SPILL_TEMP: Spill = Spill(0);
// this register can be evacuated to `SPILL_TEMP` if need be
pub const TEMPORARY_TEMP: Register = Register::X(10);

#[allow(clippy::cast_possible_wrap)]
pub const fn stack_offset(position: Spill) -> Immediate {
    Immediate {
        val: SPILL_SPACE - (8 * (position.0 as i64 + 1)),
    }
}

// there can be at most 140 variables in the environment (can be adapted via `SPILL_NUM`)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Temporary {
    Register(Register),
    Spill(Spill),
}

// the size of the memory is hardcoded and can be adapted via `heapsize` in
// `infrastructure/driver-template.c`
pub const fn address(n: i64) -> i64 {
    8 * n
}

pub const FIELDS_PER_BLOCK: usize = 3;

pub const REFERENCE_COUNT_OFFSET: i64 = address(0);

pub const NEXT_ELEMENT_OFFSET: i64 = address(0);

#[allow(clippy::cast_possible_wrap)]
pub const fn field_offset(number: TemporaryNumber, i: usize) -> Immediate {
    Immediate {
        val: address(2 + 2 * i as i64 + number as i64),
    }
}

// no need to save X2 and X3 as they are our scratch registers `TEMP` and `TEMP2`
pub const CALLER_SAVE_FIRST: usize = 4;
pub const CALLER_SAVE_LAST: usize = 17;

impl Config<Temporary, Immediate> for Backend {
    fn i64_to_immediate(number: i64) -> Immediate {
        Immediate { val: number }
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
