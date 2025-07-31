//! This crate implements the translation from the surface language [Fun](fun) into the high-level
//! intermediate representation [Core](core_lang). It is similar to a CPS translation and uses
//! similar techniques to avoid administrative redexes.
//!
//! # Usage
//!
//! Function [`program::compile_prog`] translates a typechecked [Fun](fun) program into a
//! [Core](core_lang) program. It assumes that the types of terms in the [Fun](fun) program are
//! annotated. Thus, this should be ensured before applying the function.

pub mod compile;
pub mod context;
pub mod declaration;
pub mod def;
pub mod program;
pub mod substitution;
pub mod terms;
pub mod types;
