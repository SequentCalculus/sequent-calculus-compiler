use crate::{
    syntax::{
        context::TypingContext,
        declarations::{CodataDeclaration, DataDeclaration, Declaration, Definition, Module},
        terms::{
            Case, Cocase, Constructor, Destructor, Fun, Goto, IfZ, Label, Let, Lit, Op, Paren,
            Term, Var,
        },
        types::Ty,
    },
    typing::symbol_table::{self, build_symbol_table},
};

use super::{errors::Error, symbol_table::SymbolTable};

pub fn check_module(module: &Module) -> Result<(), Error> {
    let symbol_table = build_symbol_table(module)?;
    check_module_with_table(module, &symbol_table)
}

fn check_module_with_table(module: &Module, symbol_table: &SymbolTable) -> Result<(), Error> {
    for decl in module.declarations.iter() {
        check_declaration(decl, symbol_table)?
    }
    Ok(())
}

fn check_declaration(decl: &Declaration, symbol_table: &SymbolTable) -> Result<(), Error> {
    match decl {
        Declaration::Definition(definition) => check_definition(definition, symbol_table),
        Declaration::DataDeclaration(data_declaration) => {
            check_data_declaration(data_declaration, symbol_table)
        }
        Declaration::CodataDeclaration(codata_declaration) => {
            check_codata_declaration(codata_declaration, symbol_table)
        }
    }
}

fn check_definition(def: &Definition, symbol_table: &SymbolTable) -> Result<(), Error> {
    todo!()
}

fn check_data_declaration(decl: &DataDeclaration, symbol_table: &SymbolTable) -> Result<(), Error> {
    todo!()
}

fn check_codata_declaration(
    decl: &CodataDeclaration,
    symbol_table: &SymbolTable,
) -> Result<(), Error> {
    todo!()
}

trait Check {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error>;
}

impl Check for Term {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
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

impl Check for Var {
    fn check(
        &self,
        _symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!("lookup in context")
    }
}

impl Check for Lit {
    fn check(
        &self,
        _symbol_table: &SymbolTable,
        _context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        match expected {
            Ty::Int { .. } => Ok(()),
            ty => Err(Error::Mismatch {
                expected: ty.clone(),
                got: Ty::mk_int(),
            }),
        }
    }
}

impl Check for Op {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for IfZ {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Let {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Fun {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Constructor {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Destructor {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Case {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Cocase {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Label {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Goto {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Paren {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: Ty,
    ) -> Result<(), Error> {
        self.inner.check(symbol_table, context, expected)
    }
}
