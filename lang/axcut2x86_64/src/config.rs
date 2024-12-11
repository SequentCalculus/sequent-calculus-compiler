use super::Backend;

use axcut2backend::config::{Config, TemporaryNumber};
use printer::{DocAllocator, Print};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Register(pub usize);

impl Register {
    pub fn rbx() -> Self {
        Register(2)
    }

    pub fn rbp() -> Self {
        Register(3)
    }
}

impl Print for Register {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Register(0) => alloc.text("rsp"),
            Register(1) => alloc.text("rcx"),
            Register(2) => alloc.text("rbx"),
            Register(3) => alloc.text("rbp"),
            Register(4) => alloc.text("rax"),
            Register(5) => alloc.text("rdx"),
            Register(6) => alloc.text("rsi"),
            Register(7) => alloc.text("rdi"),
            Register(n) => alloc.text(format!("r{n}")),
        }
    }
}

impl From<usize> for Register {
    fn from(value: usize) -> Self {
        Register(value)
    }
}

pub const REGISTER_NUM: usize = 16;

#[derive(Debug, Copy, Clone)]
pub struct Immediate {
    pub val: i64,
}

impl Print for Immediate {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(format!("{}", self.val))
    }
}

impl From<i64> for Immediate {
    fn from(value: i64) -> Self {
        Immediate { val: value }
    }
}

// rsp is the stack pointer we use for register spills
// rcx is used for our purposes
// rbx is a heap pointer to an object which we can directly overwrite
// rbp is a deferred-free-list pointer to objects which we have to free
pub const RESERVED: usize = 4;

pub const STACK: Register = Register(0);
pub const TEMP: Register = Register(1);
pub const HEAP: Register = Register(2);
pub const FREE: Register = Register(3);

pub const RETURN1: Register = Register(4);
pub const RETURN2: Register = Register(5);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Spill(pub usize);

pub const SPILL_NUM: usize = 256;
#[allow(clippy::cast_possible_wrap)]
pub const SPILL_SPACE: i64 = SPILL_NUM as i64 * 8;

// one spot is used for our purposes
pub const RESERVED_SPILLS: usize = 1;

pub const SPILL_TEMP: Spill = Spill(0);

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub const fn stack_offset(position: Spill) -> Immediate {
    Immediate {
        val: SPILL_SPACE - (8 * (position.0 as i64 + 1)),
    }
}

// there can be at most 133 variables in the environment, unless a syscall is made (which are not
// yet implemented), then it might be up to 4 less (can be adapted via `SPILL_NUM`)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Temporary {
    Register(Register),
    Spill(Spill),
}

// the size of the memory is hardcoded and can be adapted via `heapsize` in
// `infrastructure/x86_64/driver*.c`
#[must_use]
pub const fn address(n: i64) -> i64 {
    8 * n
}

pub const FIELDS_PER_BLOCK: usize = 3;

pub const REFERENCE_COUNT_OFFSET: i64 = address(0);

pub const NEXT_ELEMENT_OFFSET: i64 = address(0);

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub const fn field_offset(number: TemporaryNumber, i: usize) -> Immediate {
    Immediate {
        val: address(2 + 2 * i as i64 + number as i64),
    }
}

#[must_use]
pub fn arg(number: usize) -> Register {
    match number {
        0 => Register(7),
        1 => Register(6),
        2 => Register(5),
        3 => Register(10),
        4 => Register(8),
        5 => Register(9),
        _ => panic!("syscalls can have 6 arguments at most"),
    }
}

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

    // there can be at most 64 constructors per type, to keep the instructions in the jump table
    // jmp rel8 with 2 bytes length
    // workarounds: - make all jumps in the jump table jmp near rel32 with 5 bytes length and adapt
    //                `jump_length`
    //              - use typing information to know number of constructors and adapt `jump_length`
    //                to account for the different lengths of instructions in the jump table
    //                (all 5 bytes except last 64 which are 2 bytes)
    #[allow(clippy::cast_possible_wrap)]
    fn jump_length(n: usize) -> Immediate {
        Immediate { val: 2 * n as i64 }
    }
}
