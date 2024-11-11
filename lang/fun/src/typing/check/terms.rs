use crate::{
    syntax::{context::TypingContext, terms::Term, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};
use std::rc::Rc;

pub trait Check: Sized {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error>;
}

impl Check for Term {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        match self {
            Term::Var(var) => var.check(symbol_table, context, expected).map(|v| v.into()),
            Term::Lit(lit) => lit
                .check(symbol_table, context, expected)
                .map(|lit| lit.into()),
            Term::Op(op) => op
                .check(symbol_table, context, expected)
                .map(|op| op.into()),
            Term::IfZ(if_z) => if_z
                .check(symbol_table, context, expected)
                .map(|ifz| ifz.into()),
            Term::Let(letexp) => letexp
                .check(symbol_table, context, expected)
                .map(|letexp| letexp.into()),
            Term::Fun(fun) => fun
                .check(symbol_table, context, expected)
                .map(|fun| fun.into()),
            Term::Constructor(constructor) => constructor
                .check(symbol_table, context, expected)
                .map(|ctor| ctor.into()),
            Term::Destructor(destructor) => destructor
                .check(symbol_table, context, expected)
                .map(|dtor| dtor.into()),
            Term::Case(case) => case
                .check(symbol_table, context, expected)
                .map(|case| case.into()),
            Term::Cocase(cocase) => cocase
                .check(symbol_table, context, expected)
                .map(|cocase| cocase.into()),
            Term::Goto(goto) => goto
                .check(symbol_table, context, expected)
                .map(|goto| goto.into()),
            Term::Label(label) => label
                .check(symbol_table, context, expected)
                .map(|label| label.into()),
            Term::Paren(paren) => paren
                .check(symbol_table, context, expected)
                .map(|paren| paren.into()),
        }
    }
}

impl<T: Check + Clone> Check for Rc<T> {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let self_checked = Rc::unwrap_or_clone(self).check(symbol_table, context, expected)?;
        Ok(Rc::new(self_checked))
    }
}
