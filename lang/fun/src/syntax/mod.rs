//! This module defines the syntax for the surface language Fun.

pub mod arguments;
pub mod context;
pub mod declarations;
pub mod names;
pub mod program;
pub mod terms;
pub mod types;

pub use arguments::Arguments;
pub use context::{
    Chirality, Chirality::Cns, Chirality::Prd, TypeContext, TypingContext, VarContext,
};
pub use declarations::*;
pub use names::{Name, Var};
pub use terms::*;
pub use types::{OptTyped, Ty, TypeArgs};
