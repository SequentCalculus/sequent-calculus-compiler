//! This module contains the linearization pass translating the non-linearized version of AxCut
//! into the linearized one, along with the infrastructure needed to do so. This infrastructure
//! includes the computation of free variables of a term and an implementation of substituting
//! variables for other variables. The linearization and substitution assume all variable bindings
//! in each path through a program to be unique and maintain this invariant. Substitution further
//! assumes that the variables substituted into a statement are fresh for this statement.

pub mod free_vars;
pub mod linearize;
pub mod substitution;
