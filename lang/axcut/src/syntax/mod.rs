pub mod call;
pub mod chirality;
pub mod clause;
pub mod context;
pub mod declaration;
pub mod def;
pub mod ifz;
pub mod invoke;
pub mod leta;
pub mod literal;
pub mod names;
pub mod new;
pub mod op;
pub mod program;
pub mod ret;
pub mod statement;
pub mod substitute;
pub mod switch;
pub mod types;

pub use call::Call;
pub use chirality::Chirality;
pub use clause::Clause;
pub use context::{ContextBinding, TypingContext};
pub use declaration::{TypeDeclaration, XtorSig};
pub use def::Def;
pub use ifz::IfZ;
pub use invoke::Invoke;
pub use leta::Leta;
pub use literal::Literal;
pub use names::{BinOp, Name, Var};
pub use new::New;
pub use op::Op;
pub use program::Prog;
pub use ret::Return;
pub use statement::Statement;
pub use substitute::Substitute;
pub use switch::Switch;
pub use types::Ty;

use std::fmt;

pub fn stringify_and_join<T: fmt::Display>(vec: &[T], separator: &str) -> String {
    vec.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(separator)
}
