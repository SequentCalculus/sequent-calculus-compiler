//! Syntax for the core language
pub mod context;
pub mod declaration;
pub mod def;
pub mod names;
pub mod program;
pub mod statements;
pub mod substitution;
pub mod terms;
pub mod types;

pub use context::{Chirality, ContextBinding, TypingContext};
pub use declaration::{Codata, CodataDeclaration, CtorSig, Data, DataDeclaration, DtorSig};
pub use def::{Def, FsDef};
pub use names::{fresh_covar, fresh_name, fresh_var, Covar, Name, Var};
pub use program::Prog;
pub use statements::{FsStatement, Statement};
pub use substitution::{
    Substitution,
    SubstitutionBinding::{ConsumerBinding, ProducerBinding},
};
pub use terms::{BinOp, FsTerm, Term};
pub use types::Ty;
