use super::Backend;

use axcut2backend::config::{Config, TemporaryNumber};

use std::fmt;

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

pub const REGISTER_NUM: usize = 32;

pub type Immediate = i64;

// x0 is always zero
// x1 is used for our purposes
// x2 is a heap pointer to an object which we can directly overwrite
// x3 is a deferred-free-list pointer to objects which we have to free

pub const RESERVED: usize = 4;

pub const ZERO: Register = Register(0);
pub const TEMP: Register = Register(1);
pub const HEAP: Register = Register(2);
pub const FREE: Register = Register(3);

pub const RETURN1: Register = Register(10);
pub const RETURN2: Register = Register(11);

#[must_use]
pub const fn address(n: i64) -> i64 {
    8 * n
}

pub const FIELDS_PER_BLOCK: usize = 3;

pub const REFERENCE_COUNT_OFFSET: i64 = address(0);

pub const NEXT_ELEMENT_OFFSET: i64 = address(0);

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub const fn field_offset(number: TemporaryNumber, i: usize) -> i64 {
    address(2 + 2 * i as i64 + number as i64)
}

impl Config<Register, Immediate> for Backend {
    fn i64_to_immediate(&self, number: i64) -> Immediate {
        number
    }

    fn temp(&self) -> Register {
        TEMP
    }

    fn heap(&self) -> Register {
        HEAP
    }

    fn free(&self) -> Register {
        FREE
    }

    fn return1(&self) -> Register {
        RETURN1
    }

    fn return2(&self) -> Register {
        RETURN2
    }

    #[allow(clippy::cast_possible_wrap)]
    fn jump_length(&self, n: usize) -> Immediate {
        4 * n as Immediate
    }
}
