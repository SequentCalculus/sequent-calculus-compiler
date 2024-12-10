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

    use super::{BuildSymbolTable, Polarity, SymbolTable};
    use crate::{
        parser::util::ToMiette,
        syntax::{
            context::{ContextBinding, TypingContext},
            declarations::{CodataDeclaration, Definition, DtorSig, Module},
            substitution::SubstitutionBinding,
            terms::{Constructor, Lit},
            types::Ty,
        },
        test_common::data_list,
    };
    use codespan::Span;

    fn example_codata() -> CodataDeclaration {
        CodataDeclaration {
            span: Span::default(),
            name: "StreamInt".to_owned(),
            dtors: vec![
                DtorSig {
                    span: Span::default(),
                    name: "Hd".to_owned(),
                    args: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    cont_ty: Ty::mk_int(),
                },
                DtorSig {
                    span: Span::default(),
                    name: "Tl".to_owned(),
                    args: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    cont_ty: Ty::mk_decl("StreamInt"),
                },
            ],
        }
    }
    fn example_def() -> Definition {
        Definition {
            span: Span::default(),
            name: "main".to_owned(),
            context: TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            ret_ty: Ty::mk_decl("ListInt"),
            body: Constructor {
                span: Span::default(),
                id: "Cons".to_owned(),
                args: vec![
                    SubstitutionBinding::TermBinding(
                        Lit {
                            span: Span::default(),
                            val: 1,
                        }
                        .into(),
                    ),
                    SubstitutionBinding::TermBinding(
                        Constructor {
                            span: Span::default(),
                            id: "Nil".to_owned(),
                            args: vec![],
                            ty: Some(Ty::mk_decl("ListInt")),
                        }
                        .into(),
                    ),
                ],
                ty: Some(Ty::mk_decl("ListInt")),
            }
            .into(),
        }
    }
    #[test]
    fn build_module() {
        let mut symbol_table = SymbolTable::default();
        Module {
            declarations: vec![
                data_list().into(),
                example_codata().into(),
                example_def().into(),
            ],
        }
        .build(&mut symbol_table)
        .unwrap();
        let mut expected = SymbolTable::default();
        expected.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        expected.ty_ctors.insert(
            "StreamInt".to_owned(),
            (Polarity::Codata, vec!["Hd".to_owned(), "Tl".to_owned()]),
        );
        expected.ctors.insert(
            "Nil".to_owned(),
            TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
        );
        expected.ctors.insert(
            "Cons".to_owned(),
            TypingContext {
                span: Span::default(),
                bindings: vec![
                    ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedVar {
                        var: "xs".to_owned(),
                        ty: Ty::mk_decl("ListInt"),
                    },
                ],
            },
        );
        expected.dtors.insert(
            "Hd".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                Ty::mk_int(),
            ),
        );
        expected.dtors.insert(
            "Tl".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                Ty::mk_decl("StreamInt"),
            ),
        );
        expected.funs.insert(
            "main".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                Ty::mk_decl("ListInt"),
            ),
        );
        assert_eq!(symbol_table, expected)
    }
    #[test]
    fn build_data() {
        let mut symbol_table = SymbolTable::default();
        data_list().build(&mut symbol_table).unwrap();
        let mut expected = SymbolTable::default();
        expected.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        expected.ctors.insert(
            "Nil".to_owned(),
            TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
        );
        expected.ctors.insert(
            "Cons".to_owned(),
            TypingContext {
                span: Span::default(),
                bindings: vec![
                    ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedVar {
                        var: "xs".to_owned(),
                        ty: Ty::mk_decl("ListInt"),
                    },
                ],
            },
        );
        assert_eq!(symbol_table, expected)
    }
    #[test]
    fn build_codata() {
        let mut symbol_table = SymbolTable::default();
        example_codata().build(&mut symbol_table).unwrap();
        let mut expected = SymbolTable::default();
        expected.ty_ctors.insert(
            "StreamInt".to_owned(),
            (Polarity::Codata, vec!["Hd".to_owned(), "Tl".to_owned()]),
        );
        expected.dtors.insert(
            "Hd".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                Ty::mk_int(),
            ),
        );
        expected.dtors.insert(
            "Tl".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                Ty::mk_decl("StreamInt"),
            ),
        );
        assert_eq!(symbol_table, expected)
    }
    #[test]
    fn build_def() {
        let mut symbol_table = SymbolTable::default();
        example_def().build(&mut symbol_table).unwrap();
        let mut expected = SymbolTable::default();
        expected.funs.insert(
            "main".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                Ty::mk_decl("ListInt"),
            ),
        );
        assert_eq!(symbol_table, expected)
    }

    #[test]
    fn dtor_lookup() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "LPairIntInt".to_owned(),
            (Polarity::Codata, vec!["Fst".to_owned(), "Snd".to_owned()]),
        );
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
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
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
