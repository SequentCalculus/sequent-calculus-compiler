//! This module contains the syntax of AxCut. At its core, it is a one-sided sequent calculus with
//! user-defined (co)data types and top-level function definitions. It is fully focused, meaning
//! that all terms in argument positions are variables. The only terms are statements which
//! represent computations. The statements are implemented in such a way that they support a
//! non-linearized version with arbitrary variable usage and a linearized version with explicit
//! substitutions.

pub mod context;
pub mod declaration;
pub mod def;
pub mod names;
pub mod program;
pub mod statements;
pub mod types;

pub use context::{Chirality, ContextBinding, TypingContext};
pub use declaration::{TypeDeclaration, XtorSig};
pub use def::Def;
pub use names::Ident;
pub use program::Prog;
pub use statements::{BinOp, Statement};
pub use types::Ty;
