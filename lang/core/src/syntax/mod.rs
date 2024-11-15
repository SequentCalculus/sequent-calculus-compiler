pub mod clause;
pub mod context;
pub mod declaration;
pub mod def;
pub mod names;
pub mod program;
pub mod statement;
pub mod substitution;
pub mod term;
pub mod types;

pub use clause::Clause;
pub use context::{
    ContextBinding::{CovarBinding, VarBinding},
    TypingContext,
};
pub use declaration::{Codata, CodataDeclaration, CtorSig, Data, DataDeclaration, DtorSig};
pub use def::Def;
pub use names::{BinOp, Covar, Name, Var};
pub use program::Prog;
pub use statement::Statement;
pub use substitution::SubstitutionBinding::{ConsumerBinding, ProducerBinding};
pub use term::Term;
pub use types::Ty;
