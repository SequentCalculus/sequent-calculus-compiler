use std::fmt;

pub mod def;
pub mod program;
pub mod rewrite;
pub mod statements;

#[derive(Clone)]
pub struct OptimizationStats {
    num_passes: u64,
}

impl fmt::Display for OptimizationStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number of Passes: {}", self.num_passes)
    }
}
