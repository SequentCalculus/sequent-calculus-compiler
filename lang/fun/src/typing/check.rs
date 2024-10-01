use codespan::Span;
use miette::SourceSpan;

use crate::{
    parser::util::ToMiette,
    syntax::{
        context::{ContextBinding, TypingContext},
        declarations::{
            CodataDeclaration, CtorSig, DataDeclaration, Declaration, Definition, DtorSig, Module,
        },
        substitution::{Substitution, SubstitutionBinding},
        terms::{
            Case, Cocase, Constructor, Destructor, Fun, Goto, IfZ, Label, Let, Lit, Op, Paren,
            Term, Var,
        },
        types::Ty,
        Covariable, Name, Variable,
    },
    typing::symbol_table::{build_symbol_table, Polarity},
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

// Lookup functions
//
//

fn lookup_var(
    span: &SourceSpan,
    ctx: &TypingContext,
    searched_var: &Variable,
) -> Result<Ty, Error> {
    // Due to variable shadowing we have to traverse from
    // right to left.
    for binding in ctx.iter().rev() {
        match binding {
            ContextBinding::TypedVar { var, ty } => {
                if var == searched_var {
                    return Ok(ty.clone());
                }
                continue;
            }
            ContextBinding::TypedCovar { .. } => continue,
        }
    }
    Err(Error::UnboundVariable {
        span: *span,
        var: searched_var.clone(),
    })
}

fn lookup_covar(
    span: &SourceSpan,
    ctx: &TypingContext,
    searched_covar: &Covariable,
) -> Result<Ty, Error> {
    // Due to variable shadowing we have to traverse from
    // right to left.
    for binding in ctx.iter().rev() {
        match binding {
            ContextBinding::TypedVar { .. } => continue,
            ContextBinding::TypedCovar { covar, ty } => {
                if covar == searched_covar {
                    return Ok(ty.clone());
                }
                continue;
            }
        }
    }
    Err(Error::UnboundCovariable {
        span: *span,
        covar: searched_covar.clone(),
    })
}

fn lookup_ty_for_dtor(
    span: &SourceSpan,
    dtor: &Name,
    symbol_table: &SymbolTable,
) -> Result<Ty, Error> {
    for (ty_ctor, (pol, xtors)) in symbol_table.ty_ctors.iter() {
        if pol == &Polarity::Codata && xtors.contains(dtor) {
            return Ok(Ty::Decl {
                span: Span::default(),
                name: ty_ctor.to_string(),
            });
        }
    }
    Err(Error::Undefined {
        span: *span,
        name: dtor.clone(),
    })
}

fn lookup_ty_for_ctor(
    span: &SourceSpan,
    ctor: &Name,
    symbol_table: &SymbolTable,
) -> Result<Ty, Error> {
    for (ty_ctor, (pol, xtors)) in symbol_table.ty_ctors.iter() {
        if pol == &Polarity::Data && xtors.contains(ctor) {
            return Ok(Ty::Decl {
                span: Span::default(),
                name: ty_ctor.to_string(),
            });
        }
    }
    Err(Error::Undefined {
        span: *span,
        name: ctor.clone(),
    })
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
            Some(_) => Ok(()),
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
    Ok(())
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
    span: &SourceSpan,
    symbol_table: &SymbolTable,
    context: &TypingContext,
    args: &Substitution,
    types: &TypingContext,
) -> Result<(), Error> {
    if types.len() != args.len() {
        return Err(Error::WrongNumberOfArguments {
            span: *span,
            expected: types.len(),
            got: args.len(),
        });
    }
    for c in types.iter().zip(args.iter()) {
        match c {
            (ContextBinding::TypedVar { ty, .. }, SubstitutionBinding::TermBinding(term)) => {
                term.check(symbol_table, context, ty)?
            }
            (ContextBinding::TypedCovar { ty, .. }, SubstitutionBinding::CovarBinding(cov)) => {
                let found_ty = lookup_covar(span, context, cov)?;
                if &found_ty == ty {
                    continue;
                } else {
                    return Err(Error::Mismatch {
                        span: *span,
                        expected: ty.clone(),
                        got: found_ty,
                    });
                }
            }
            (ContextBinding::TypedVar { .. }, SubstitutionBinding::CovarBinding(_)) => {
                return Err(Error::ExpectedTermGotCovariable { span: *span })
            }
            (ContextBinding::TypedCovar { .. }, SubstitutionBinding::TermBinding(..)) => {
                return Err(Error::ExpectedCovariableGotTerm { span: *span })
            }
        }
    }
    Ok(())
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
        let found_ty = lookup_var(&self.span.to_miette(), context, &self.var)?;
        if &found_ty == expected {
            return Ok(());
        } else {
            return Err(Error::Mismatch {
                span: self.span.to_miette(),
                expected: expected.clone(),
                got: found_ty,
            });
        }
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
        self.bound_term.check(symbol_table, context, &self.var_ty)?;
        let mut new_context = context.clone();
        new_context.push(ContextBinding::TypedVar {
            var: self.variable.clone(),
            ty: self.var_ty.clone(),
        });
        self.in_term.check(symbol_table, &new_context, expected)?;
        Ok(())
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
                    check_args(
                        &self.span.to_miette(),
                        symbol_table,
                        context,
                        &self.args,
                        types,
                    )
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
        let ty = lookup_ty_for_dtor(&self.span.to_miette(), &self.id, symbol_table)?;
        self.destructee.check(symbol_table, context, &ty)?;
        match symbol_table.dtors.get(&self.id) {
            Some((types, ret_ty)) => {
                check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    &self.args,
                    types,
                )?;
                if ret_ty != expected {
                    Err(Error::Mismatch {
                        span: self.span.to_miette(),
                        expected: expected.clone(),
                        got: ret_ty.clone(),
                    })
                } else {
                    Ok(())
                }
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.id.clone(),
            }),
        }
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
        let mut new_context = context.clone();
        new_context.push(ContextBinding::TypedCovar {
            covar: self.label.clone(),
            ty: expected.clone(),
        });
        self.term.check(symbol_table, &new_context, expected)
    }
}

impl Check for Goto {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        _expected: &Ty,
    ) -> Result<(), Error> {
        let cont_type = lookup_covar(&self.span.to_miette(), context, &self.target)?;
        self.term.check(symbol_table, context, &cont_type)
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
