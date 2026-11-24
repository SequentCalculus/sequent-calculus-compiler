//! This crate contains optimization passes for axcut, namely inlining and cut elimination

use axcut::syntax::Prog;

use crate::{eliminate::program::eliminate, inline::program::inline, dead_code::program::dead_code};

mod eliminate;
mod dead_code;
mod inline;

pub const DEFAULT_NUM_PASSES: u64 = 10;

pub enum OptMode {
    Normal(u64),
    InlineOnly,
    EliminateOnly,
    DeadCodeOnly
}

pub fn optimize(prog: &mut Prog, mode: &OptMode) {
    match mode {
        OptMode::Normal(passes) => {
            // TODO: what is a good order of these passes?
            for _run in 0..*passes {
                dead_code(prog);
                inline(prog);
                eliminate(prog);
            }
        }
        OptMode::EliminateOnly => eliminate(prog),
        OptMode::InlineOnly => inline(prog),
        OptMode::DeadCodeOnly => dead_code(prog),
    }
}
