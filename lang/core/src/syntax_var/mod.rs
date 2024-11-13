pub mod chirality;
pub mod clause;
pub mod context;
pub mod declaration;
pub mod def;
pub mod names;
pub mod program;
pub mod statement;
pub mod term;
pub mod types;

pub use chirality::Chirality;
pub use clause::Clause;
pub use context::{ContextBinding, TypingContext};
pub use declaration::{cont_int, TypeDeclaration, XtorSig};
pub use def::Def;
pub use names::{BinOp, Name, Var};
pub use program::Prog;
pub use statement::Statement;
pub use term::Term;
pub use types::Ty;

use std::fmt;

fn stringify_and_join<T: fmt::Display>(vec: &[T], separator: &str) -> String {
    vec.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(separator)
}
