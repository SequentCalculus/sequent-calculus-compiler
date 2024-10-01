use crate::{
    parser::util::ToMiette,
    syntax::{
        context::{ContextBinding, TypingContext},
        declarations::{
            CodataDeclaration, CtorSig, DataDeclaration, Declaration, Definition, DtorSig, Module
        },
        substitution::Substitution,
        terms::{
            Case, Cocase, Constructor, Destructor, Fun, Goto, IfZ, Label, Let, Lit, Op, Paren,
            Term, Var,
        },
        types::Ty,
    },
    typing::symbol_table::build_symbol_table,
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

// Checking types and typing contexts
//
//

fn check_type(ty: &Ty, symbol_table: &SymbolTable) -> Result<(), Error> {
    match ty {
        Ty::Int { .. } => Ok(()),
        Ty::Decl { span, name } => match symbol_table.ty_ctors.get(name) {
            None => Err(Error::Undefined {
                span: span.to_miette(),
                name: name.clone(),
            }),
            Some(_) => Ok(())
        },
    }
}

fn check_typing_context(ctx: &TypingContext, symbol_table: &SymbolTable) -> Result<(), Error> {
    for binding in ctx.iter() {
        match binding {
            ContextBinding::TypedVar { ty, .. } => check_type(ty, symbol_table)?,
            ContextBinding::TypedCovar { ty, .. } => check_type(ty, symbol_table)?,
        }
    }
    todo!()
}

// Checking toplevel declarations
//
//

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
    check_typing_context(&def.context, symbol_table)?;
    check_type(&def.ret_ty, symbol_table)?;
    def.body.check(symbol_table, &def.context, &def.ret_ty)
}

fn check_data_declaration(decl: &DataDeclaration, symbol_table: &SymbolTable) -> Result<(), Error> {
    for ctor in decl.ctors.iter() {
        check_ctor_sig(ctor, symbol_table)?;
    }
    Ok(())
}

fn check_ctor_sig(ctor: &CtorSig, symbol_table: &SymbolTable) -> Result<(), Error> {
    check_typing_context(&ctor.args, symbol_table)?;
    Ok(())
}

fn check_codata_declaration(
    decl: &CodataDeclaration,
    symbol_table: &SymbolTable,
) -> Result<(), Error> {
    for dtor in decl.dtors.iter() {
        check_dtor_sig(dtor, symbol_table)?;
    }
    Ok(())
}

fn check_dtor_sig(dtor: &DtorSig, symbol_table: &SymbolTable) -> Result<(), Error> {
    check_typing_context(&dtor.args, symbol_table)?;
    check_type(&dtor.cont_ty, symbol_table)?;
    Ok(())
}

// Checking terms
//
//

fn check_args(
    symbol_table: &SymbolTable,
    context: &TypingContext,
    args: &Substitution,
    types: &TypingContext,
) -> Result<(), Error> {
    todo!()
}

trait Check {
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

impl Check for Var {
    fn check(
        &self,
        _symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        // Due to variable shadowing we have to traverse from
        // right to left.
        for binding in context.iter().rev() {
            match binding {
                crate::syntax::context::ContextBinding::TypedVar { var, ty } => {
                    if var == &self.var {
                        if ty == expected {
                            return Ok(());
                        }
                        return Err(Error::Mismatch {
                            span: self.span.to_miette(),
                            expected: expected.clone(),
                            got: ty.clone(),
                        });
                    }
                    continue;
                }
                crate::syntax::context::ContextBinding::TypedCovar { .. } => continue,
            }
        }
        Err(Error::UnboundVariable {
            span: self.span.to_miette(),
            var: self.var.clone(),
        })
    }
}

impl Check for Lit {
    fn check(
        &self,
        _symbol_table: &SymbolTable,
        _context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        match expected {
            Ty::Int { .. } => Ok(()),
            ty => Err(Error::Mismatch {
                span: self.span.to_miette(),
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
        expected: &Ty,
    ) -> Result<(), Error> {
        match expected {
            Ty::Int { .. } => {
                self.fst.check(symbol_table, context, expected)?;
                self.snd.check(symbol_table, context, expected)?
            }
            ty => {
                return Err(Error::Mismatch {
                    span: self.span.to_miette(),
                    expected: expected.clone(),
                    got: ty.clone(),
                })
            }
        }
        Ok(())
    }
}

impl Check for IfZ {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        self.ifc.check(symbol_table, context, &Ty::mk_int())?;
        self.thenc.check(symbol_table, context, expected)?;
        self.elsec.check(symbol_table, context, expected)
    }
}

impl Check for Let {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Fun {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        match symbol_table.funs.get(&self.name) {
            Some((types, ret_ty)) => {
                if ret_ty == expected {
                    check_args(symbol_table, context, &self.args, types)
                } else {
                    Err(Error::Mismatch {
                        span: self.span.to_miette(),
                        expected: expected.clone(),
                        got: ret_ty.clone(),
                    })
                }
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.name.clone(),
            }),
        }
    }
}

impl Check for Constructor {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        match symbol_table.ctors.get(&self.id) {
            Some(types) => todo!(),
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.id.clone(),
            }),
        }
    }
}

impl Check for Destructor {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Case {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Cocase {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Label {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Goto {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl Check for Paren {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        self.inner.check(symbol_table, context, expected)
    }
}
