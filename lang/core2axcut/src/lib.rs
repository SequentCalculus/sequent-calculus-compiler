//! This crate implements the translation from the focused version of the [Core](core_lang)
//! intermediate representation into the non-linearized version of the [AxCut](axcut) intermediate
//! representation. It thus performs the [shrinking] transformation, which eliminates redundant
//! constructs of [Core](core_lang) leaving certain forms of statements, and at the same time
//! collapses the system to the unified types and terms of the one-sided [AxCut](axcut) system.
//!
//! # Usage
//!
//! Function [`program::shrink_prog`] translates a focused [Core](core_lang) program into a
//! non-linearized [AxCut](axcut) program. It assumes all variable bindings in each path through a
//! program to be unique and maintains this invariant. Thus, this property should be ensured to
//! hold before applying the function.

pub mod context;
pub mod declaration;
pub mod def;
pub mod names;
pub mod program;
pub mod shrinking;
pub mod statements;
pub mod types;
