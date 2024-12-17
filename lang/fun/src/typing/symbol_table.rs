use std::collections::{HashMap, HashSet};

use codespan::Span;
use miette::SourceSpan;

use crate::syntax::{
    context::TypingContext,
    declarations::{
        CodataDeclaration, CtorSig, DataDeclaration, Declaration, Definition, DtorSig, Module,
    },
    types::Ty,
    Name,
};

use super::errors::Error;
use crate::parser::util::ToMiette;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Polarity {
    Data,
    Codata,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolTable {
    pub funs: HashMap<Name, (TypingContext, Ty)>,
    pub ctors: HashMap<Name, TypingContext>,
    pub dtors: HashMap<Name, (TypingContext, Ty)>,
    pub ty_ctors: HashMap<Name, (Polarity, Vec<Name>)>,
}

impl SymbolTable {
    pub fn lookup_ty_for_dtor(&self, span: &SourceSpan, dtor: &Name) -> Result<Ty, Error> {
        for (ty_ctor, (pol, xtors)) in &self.ty_ctors {
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

    pub fn lookup_ty_for_ctor(
        &self,
        span: &SourceSpan,
        ctor: &Name,
    ) -> Result<(Ty, HashSet<String>), Error> {
        for (ty_ctor, (pol, xtors)) in &self.ty_ctors {
            if pol == &Polarity::Data && xtors.contains(ctor) {
                return Ok((
                    Ty::Decl {
                        span: Span::default(),
                        name: ty_ctor.to_string(),
                    },
                    xtors.iter().cloned().collect(),
                ));
            }
        }
        Err(Error::Undefined {
            span: *span,
            name: ctor.clone(),
        })
    }

    pub fn combine(&mut self, other: SymbolTable) {
        self.funs.extend(other.funs);
        self.ctors.extend(other.ctors);
        self.dtors.extend(other.dtors);
        self.ty_ctors.extend(other.ty_ctors)
    }
}

pub fn build_symbol_table(module: &Module) -> Result<SymbolTable, Error> {
    let mut symbol_table = SymbolTable::default();
    module.build(&mut symbol_table)?;
    Ok(symbol_table)
}

pub trait BuildSymbolTable {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error>;
}

impl BuildSymbolTable for Module {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        for decl in &self.declarations {
            decl.build(symbol_table)?;
        }
        Ok(())
    }
}

impl BuildSymbolTable for Declaration {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        match self {
            Declaration::Definition(definition) => definition.build(symbol_table),
            Declaration::DataDeclaration(data_declaration) => data_declaration.build(symbol_table),
            Declaration::CodataDeclaration(codata_declaration) => {
                codata_declaration.build(symbol_table)
            }
        }
    }
}

impl BuildSymbolTable for Definition {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.funs.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes {
                span: self.span.to_miette(),
                name: self.name.clone(),
            });
        } else {
            symbol_table.funs.insert(
                self.name.clone(),
                (self.context.clone(), self.ret_ty.clone()),
            );
        }
        Ok(())
    }
}

impl BuildSymbolTable for DataDeclaration {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.ty_ctors.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes {
                span: self.span.to_miette(),
                name: self.name.clone(),
            });
        }
        symbol_table.ty_ctors.insert(
            self.name.clone(),
            (
                Polarity::Data,
                self.ctors.iter().map(|ctor| ctor.name.clone()).collect(),
            ),
        );

        for ctor in &self.ctors {
            ctor.build(symbol_table)?;
        }
        Ok(())
    }
}

impl BuildSymbolTable for CtorSig {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.ctors.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes {
                span: self.span.to_miette(),
                name: self.name.clone(),
            });
        }
        symbol_table
            .ctors
            .insert(self.name.clone(), self.args.clone());
        Ok(())
    }
}

impl BuildSymbolTable for CodataDeclaration {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.ty_ctors.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes {
                span: self.span.to_miette(),
                name: self.name.clone(),
            });
        }
        symbol_table.ty_ctors.insert(
            self.name.clone(),
            (
                Polarity::Codata,
                self.dtors.iter().map(|ctor| ctor.name.clone()).collect(),
            ),
        );

        for dtor in &self.dtors {
            dtor.build(symbol_table)?;
        }
        Ok(())
    }
}

impl BuildSymbolTable for DtorSig {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.dtors.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes {
                span: self.span.to_miette(),
                name: self.name.clone(),
            });
        }
        symbol_table
            .dtors
            .insert(self.name.clone(), (self.args.clone(), self.cont_ty.clone()));
        Ok(())
    }
}

#[cfg(test)]
mod symbol_table_tests {
    use std::collections::HashSet;

    use super::{BuildSymbolTable, SymbolTable};
    use crate::{
        parser::util::ToMiette,
        syntax::{
            context::{ContextBinding, TypingContext},
            declarations::Module,
            types::Ty,
        },
        test_common::{
            codata_stream, data_list, def_mult, symbol_table_list, symbol_table_lpair,
            symbol_table_stream,
        },
    };
    use codespan::Span;

    #[test]
    fn build_module() {
        let mut symbol_table = SymbolTable::default();
        Module {
            declarations: vec![
                data_list().into(),
                codata_stream().into(),
                def_mult().into(),
            ],
        }
        .build(&mut symbol_table)
        .unwrap();
        let mut expected = symbol_table_list();
        expected.combine(symbol_table_stream());
        expected.funs.insert(
            "mult".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![ContextBinding::TypedVar {
                        var: "l".to_owned(),
                        ty: Ty::mk_decl("ListInt"),
                    }],
                },
                Ty::mk_i64(),
            ),
        );
        assert_eq!(symbol_table, expected)
    }
    #[test]
    fn build_data() {
        let mut symbol_table = SymbolTable::default();
        data_list().build(&mut symbol_table).unwrap();
        let expected = symbol_table_list();
        assert_eq!(symbol_table, expected)
    }

    #[test]
    fn build_codata() {
        let mut symbol_table = SymbolTable::default();
        codata_stream().build(&mut symbol_table).unwrap();
        let expected = symbol_table_stream();
        assert_eq!(symbol_table, expected)
    }

    #[test]
    fn build_def() {
        let mut symbol_table = SymbolTable::default();
        def_mult().build(&mut symbol_table).unwrap();
        let mut expected = SymbolTable::default();
        expected.funs.insert(
            "mult".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![ContextBinding::TypedVar {
                        var: "l".to_owned(),
                        ty: Ty::mk_decl("ListInt"),
                    }],
                },
                Ty::mk_i64(),
            ),
        );
        assert_eq!(symbol_table, expected)
    }

    #[test]
    fn dtor_lookup() {
        let symbol_table = symbol_table_lpair();
        let result = symbol_table
            .lookup_ty_for_dtor(&Span::default().to_miette(), &"Fst".to_owned())
            .unwrap();
        let expected = Ty::mk_decl("LPairIntInt");
        assert_eq!(result, expected)
    }
    #[test]
    fn dtor_lookup_fail() {
        let result = SymbolTable::default()
            .lookup_ty_for_dtor(&Span::default().to_miette(), &"Snd".to_owned());
        assert!(result.is_err())
    }
    #[test]
    fn ctor_lookup() {
        let symbol_table = symbol_table_list();
        let result = symbol_table
            .lookup_ty_for_ctor(&Span::default().to_miette(), &"Nil".to_owned())
            .unwrap();
        let expected = (
            Ty::mk_decl("ListInt"),
            HashSet::from(["Nil".to_owned(), "Cons".to_owned()]),
        );
        assert_eq!(result, expected)
    }
    #[test]
    fn ctor_lookup_fail() {
        let result = SymbolTable::default()
            .lookup_ty_for_ctor(&Span::default().to_miette(), &"Nil".to_owned());
        assert!(result.is_err())
    }
}
