//! This module defines infrastructure traits for the intermediate representation Core, as well as
//! the [`Focusing`] trait for transforming a Core program into the focused fragment of Core.

pub mod focus;
pub mod substitution;
pub mod typed;
pub mod typed_free_vars;
pub mod uniquify;

pub use focus::{Bind, Continuation, Focusing, bind_many};
pub use substitution::{Subst, SubstVar};
pub use typed::Typed;
pub use typed_free_vars::TypedFreeVars;
pub use uniquify::Uniquify;
