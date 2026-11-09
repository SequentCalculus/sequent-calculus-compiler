//! This crate contains optimization passes for axcut, namely inlining and cut elimination

use axcut::syntax::Prog;

use crate::{eliminate::program::eliminate, inline::program::inline};

pub mod inline;
pub mod eliminate;

pub const DEFAULT_NUM_PASSES: u64 = 10;

pub fn optimize(prog: &mut Prog, passes: u64) {
    // Inline first to not do cut elimination for clearly unused defs
    // Inline last to not have unnecessarily lifted clauses left 
    for _run in 0..passes {
        inline(prog);
        eliminate(prog);
    }
    inline(prog);
}
