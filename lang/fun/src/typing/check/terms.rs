use crate::{
    syntax::{context::TypingContext, terms::Term, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

pub trait Check {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error>;
}

impl Check for Term {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        match self {
            Term::Var(var) => var.check(symbol_table, context, expected),
            Term::Lit(lit) => lit.check(symbol_table, context, expected),
            Term::Op(op) => op.check(symbol_table, context, expected),
            Term::IfZ(if_z) => if_z.check(symbol_table, context, expected),
            Term::Let(letexp) => letexp.check(symbol_table, context, expected),
            Term::Fun(fun) => fun.check(symbol_table, context, expected),
            Term::Constructor(constructor) => constructor.check(symbol_table, context, expected),
            Term::Destructor(destructor) => destructor.check(symbol_table, context, expected),
            Term::Case(case) => case.check(symbol_table, context, expected),
            Term::Cocase(cocase) => cocase.check(symbol_table, context, expected),
            Term::Goto(goto) => goto.check(symbol_table, context, expected),
            Term::Label(label) => label.check(symbol_table, context, expected),
            Term::Paren(paren) => paren.check(symbol_table, context, expected),
        }
    }
}
