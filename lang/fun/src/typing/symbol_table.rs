use std::collections::HashMap;

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
    use super::{BuildSymbolTable, Polarity, SymbolTable};
    use crate::syntax::{
        context::ContextBinding,
        declarations::{CodataDeclaration, CtorSig, DataDeclaration, Definition, DtorSig, Module},
        substitution::SubstitutionBinding,
        terms::{Constructor, Lit},
        types::Ty,
    };
    use codespan::Span;
    fn example_data() -> DataDeclaration {
        DataDeclaration {
            span: Span::default(),
            name: "ListInt".to_owned(),
            ctors: vec![
                CtorSig {
                    span: Span::default(),
                    name: "Nil".to_owned(),
                    args: vec![],
                },
                CtorSig {
                    span: Span::default(),
                    name: "Cons".to_owned(),
                    args: vec![
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
            ],
        }
    }
    fn example_codata() -> CodataDeclaration {
        CodataDeclaration {
            span: Span::default(),
            name: "StreamInt".to_owned(),
            dtors: vec![
                DtorSig {
                    span: Span::default(),
                    name: "Hd".to_owned(),
                    args: vec![],
                    cont_ty: Ty::mk_int(),
                },
                DtorSig {
                    span: Span::default(),
                    name: "Tl".to_owned(),
                    args: vec![],
                    cont_ty: Ty::mk_decl("StreamInt"),
                },
            ],
        }
    }
    fn example_def() -> Definition {
        Definition {
            span: Span::default(),
            name: "main".to_owned(),
            context: vec![],
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
                example_data().into(),
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
        expected.ctors.insert("Nil".to_owned(), vec![]);
        expected.ctors.insert(
            "Cons".to_owned(),
            vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedVar {
                    var: "xs".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                },
            ],
        );
        expected
            .dtors
            .insert("Hd".to_owned(), (vec![], Ty::mk_int()));
        expected
            .dtors
            .insert("Tl".to_owned(), (vec![], Ty::mk_decl("StreamInt")));
        expected
            .funs
            .insert("main".to_owned(), (vec![], Ty::mk_decl("ListInt")));
        assert_eq!(symbol_table, expected)
    }
    #[test]
    fn build_data() {
        let mut symbol_table = SymbolTable::default();
        example_data().build(&mut symbol_table).unwrap();
        let mut expected = SymbolTable::default();
        expected.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        expected.ctors.insert("Nil".to_owned(), vec![]);
        expected.ctors.insert(
            "Cons".to_owned(),
            vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedVar {
                    var: "xs".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                },
            ],
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
        expected
            .dtors
            .insert("Hd".to_owned(), (vec![], Ty::mk_int()));
        expected
            .dtors
            .insert("Tl".to_owned(), (vec![], Ty::mk_decl("StreamInt")));
        assert_eq!(symbol_table, expected)
    }
    #[test]
    fn build_def() {
        let mut symbol_table = SymbolTable::default();
        example_def().build(&mut symbol_table).unwrap();
        let mut expected = SymbolTable::default();
        expected
            .funs
            .insert("main".to_owned(), (vec![], Ty::mk_decl("ListInt")));
        assert_eq!(symbol_table, expected)
    }
}
