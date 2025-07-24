/// Type Alias for Variables
pub type Var = String;
/// Type Alias for Covariables
pub type Covar = String;
/// Type Alias for Names (of definitions)
pub type Name = String;

pub mod context;
pub mod declarations;
pub mod substitution;
pub mod terms;
pub mod types;
