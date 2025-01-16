pub mod context;
pub mod declaration;
pub mod def;
pub mod names;
pub mod program;
pub mod statement;
pub mod substitution;
pub mod term;
pub mod types;

pub use context::{
    Context, ContextBinding,
    ContextBinding::{CovarBinding, VarBinding},
    TypingContext,
};
pub use declaration::{Codata, CodataDeclaration, CtorSig, Data, DataDeclaration, DtorSig};
pub use def::{Def, FsDef};
pub use names::{fresh_covar, fresh_name, fresh_var, Covar, Name, Var};
pub use program::Prog;
pub use statement::BinOp;
pub use statement::{FsStatement, Statement};
pub use substitution::{
    Substitution,
    SubstitutionBinding::{ConsumerBinding, ProducerBinding},
};
pub use term::{FsTerm, Term};
pub use types::Ty;
