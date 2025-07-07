//! This crate contains the AxCut intermediate representation. The [syntax] is implemented in
//! such a way that it supports a non-linearized version of AxCut, which can be targeted from a
//! higher-level language, and a linearized version. In the non-linearized version, variables can
//! be used in arbitrary ways, while the linearized version uses explicit substitutions for
//! exchanging, duplicating, and dropping variables, i.e., making the structural rules of exchange,
//! weakening, and contraction explicit. A [linearization](traits::linearize) pass infers explicit
//! substitutions appropriately, translating from the non-linearized version into the linearized
//! one. The linearization assumes all variable bindings in each path through a program to be
//! unique and maintains this invariant. Thus, this property should be ensured to hold when
//! translating into AxCut.

pub mod syntax;
pub mod traits;
