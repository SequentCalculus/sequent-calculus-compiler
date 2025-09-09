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
    Argument::{Consumer, Producer},
    Arguments,
};
pub use context::{Chirality, ContextBinding, TypingContext};
pub use declaration::{Codata, CodataDeclaration, CtorSig, Data, DataDeclaration, DtorSig};
pub use def::{Def, FsDef};
pub use names::{Covar, Name, Var, fresh_covar, fresh_name, fresh_var};
pub use program::{FsProg, Prog};
pub use statements::{
    Call, Cut, Exit, FsCall, FsCut, FsExit, FsIfC, FsStatement, IfC, IfSort, Statement,
};
pub use terms::{
    BinOp, Clause, Cns, FsOp, FsTerm, FsXtor, Literal, Mu, Op, Prd, PrdCns, Term, XCase, XVar, Xtor,
};
pub use types::Ty;
