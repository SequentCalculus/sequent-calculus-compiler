//! This module contains the syntax of Core. In essence, it is a sequent calculus with user-defined
//! data and codata types and top-level function definitions. This module also contains the fully
//! focused fragment of Core in which all terms in argument positions are (co)variables.

pub mod arguments;
pub mod context;
pub mod declaration;
pub mod def;
pub mod names;
pub mod program;
pub mod statements;
pub mod terms;
pub mod types;

pub use arguments::{
    ArgumentEntry::{ConsumerEntry, ProducerEntry},
    Arguments,
};
pub use context::{Chirality, ContextBinding, TypingContext};
pub use declaration::{Codata, CodataDeclaration, CtorSig, Data, DataDeclaration, DtorSig};
pub use def::{Def, FsDef};
pub use names::{Covar, Name, Var, fresh_covar, fresh_name, fresh_var};
pub use program::Prog;
pub use statements::{FsStatement, Statement};
pub use terms::{BinOp, FsTerm, Term};
pub use types::Ty;
