use std::fmt;

pub mod def;
pub mod program;
pub mod rewrite;
pub mod statements;

#[derive(Clone)]
pub struct OptimizationStats {
    num_passes: u64,
    num_create_lifts: u64,
    num_switch_lifts: u64,
}

impl fmt::Display for OptimizationStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Number of Passes: {}", self.num_passes)?;
        writeln!(f, "Lifted Create Clauses: {}", self.num_create_lifts)?;
        writeln!(f, "Lifted Switch Clauses: {}", self.num_switch_lifts)?;
        Ok(())
    }
}
