use codespan::Span;
use derivative::Derivative;
use printer::{tokens::FAT_ARROW, DocAllocator, Print};

mod case;
mod cocase;
mod constructor;
mod destructor;
mod fun;
mod goto;
mod ifz;
mod label;
mod lit;
mod local_let;
mod op;
mod paren;
mod var;
pub use case::*;
pub use cocase::*;
pub use constructor::*;
pub use destructor::*;
pub use fun::*;
pub use goto::*;
pub use ifz::*;
pub use label::*;
pub use lit::*;
pub use local_let::*;
pub use op::*;
pub use paren::*;
pub use var::*;

use super::context::TypingContext;

// Clause
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Clause<T> {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub xtor: T,
    pub context: TypingContext,
    pub rhs: Term,
}

impl<T: Print> Print for Clause<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.context.is_empty() {
            self.xtor
                .print(cfg, alloc)
                .append(alloc.space())
                .append(FAT_ARROW)
                .append(alloc.space())
                .append(self.rhs.print(cfg, alloc))
        } else {
            self.xtor
                .print(cfg, alloc)
                .append(self.context.print(cfg, alloc).parens())
                .append(alloc.space())
                .append(FAT_ARROW)
                .append(alloc.space())
                .append(self.rhs.print(cfg, alloc))
        }
    }
}

// Term
//
/// Covariables (used in label, goto and toplevel calls) start with ' but this is not saved in the name string
/// that is, in source code 'a is a valid covariable, but in the AST the name is saved as a

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Lit(Lit),
    Op(Op),
    IfZ(IfZ),
    Let(Let),
    Fun(Fun),
    Constructor(Constructor),
    Destructor(Destructor),
    Case(Case),
    Cocase(Cocase),
    Goto(Goto),
    Label(Label),
    Paren(Paren),
}

impl Print for Term {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Term::Var(var) => var.print(cfg, alloc),
            Term::Lit(lit) => lit.print(cfg, alloc),
            Term::Op(op) => op.print(cfg, alloc),
            Term::IfZ(ifz) => ifz.print(cfg, alloc),
            Term::Let(lete) => lete.print(cfg, alloc),
            Term::Fun(fun) => fun.print(cfg, alloc),
            Term::Constructor(constructor) => constructor.print(cfg, alloc),
            Term::Destructor(destructor) => destructor.print(cfg, alloc),
            Term::Case(case) => case.print(cfg, alloc),
            Term::Cocase(cocase) => cocase.print(cfg, alloc),
            Term::Goto(goto) => goto.print(cfg, alloc),
            Term::Label(label) => label.print(cfg, alloc),
            Term::Paren(paren) => paren.print(cfg, alloc),
        }
    }
}
