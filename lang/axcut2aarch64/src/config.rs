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

pub const REGISTER_NUM: usize = 31;

pub type Immediate = i64;

// x2 is used for our purposes
// x0 is a heap pointer to an object which we can directly overwrite AND the first part of the return va
// x1 is a deferred-free-list pointer to objects which we have to free AND the second part of the return

pub const RESERVED: usize = 3;

pub const TEMP: Register = Register(2);
pub const HEAP: Register = Register(0);
pub const FREE: Register = Register(1);

pub const RETURN1: Register = Register(0);
pub const RETURN2: Register = Register(1);

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub const fn jump_length(n: usize) -> i64 {
    4 * n as i64
}

#[must_use]
pub const fn address(n: i64) -> i64 {
    8 * n
}

pub const FIELDS_PER_BLOCK: usize = 3;

pub const REFERENCE_COUNT_OFFSET: i64 = address(0);

pub const NEXT_ELEMENT_OFFSET: i64 = address(0);

#[derive(Copy, Clone)]
pub enum RegisterNumber {
    Fst = 0,
    Snd = 1,
}

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub const fn field_offset(number: RegisterNumber, i: usize) -> i64 {
    address(2 + 2 * i as i64 + number as i64)
}
