use super::{check_type, context::check_typing_context, terms::Check};
use crate::{
    syntax::{
        declarations::{
            CodataDeclaration, CtorSig, DataDeclaration, Declaration, Definition, DtorSig,
        },
        types::Ty,
        Name,
    },
    typing::{
        errors::Error,
        symbol_table::{Polarity, SymbolTable},
    },
};
use codespan::Span;
use miette::SourceSpan;
use std::collections::HashSet;

// Checking top-level declarations
//
//

pub fn check_declaration(
    decl: Declaration,
    symbol_table: &SymbolTable,
) -> Result<Declaration, Error> {
    match decl {
        Declaration::Definition(definition) => {
            let new_def = check_definition(definition, symbol_table)?;
            Ok(new_def.into())
        }
        Declaration::DataDeclaration(data_declaration) => {
            check_data_declaration(&data_declaration, symbol_table)?;
            Ok(data_declaration.into())
        }
        Declaration::CodataDeclaration(codata_declaration) => {
            check_codata_declaration(&codata_declaration, symbol_table)?;
            Ok(codata_declaration.into())
        }
    }
}

pub fn check_definition(def: Definition, symbol_table: &SymbolTable) -> Result<Definition, Error> {
    check_typing_context(&def.context, symbol_table)?;
    check_type(&def.ret_ty, symbol_table)?;
    let body_checked = def.body.check(symbol_table, &def.context, &def.ret_ty)?;
    Ok(Definition {
        body: body_checked,
        ..def
    })
}

fn check_data_declaration(decl: &DataDeclaration, symbol_table: &SymbolTable) -> Result<(), Error> {
    for ctor in &decl.ctors {
        check_ctor_sig(ctor, symbol_table)?;
    }
    Ok(())
}
fn check_codata_declaration(
    decl: &CodataDeclaration,
    symbol_table: &SymbolTable,
) -> Result<(), Error> {
    for dtor in &decl.dtors {
        check_dtor_sig(dtor, symbol_table)?;
    }
    Ok(())
}

fn check_ctor_sig(ctor: &CtorSig, symbol_table: &SymbolTable) -> Result<(), Error> {
    check_typing_context(&ctor.args, symbol_table)?;
    Ok(())
}

fn check_dtor_sig(dtor: &DtorSig, symbol_table: &SymbolTable) -> Result<(), Error> {
    check_typing_context(&dtor.args, symbol_table)?;
    check_type(&dtor.cont_ty, symbol_table)?;
    Ok(())
}

pub fn lookup_ty_for_dtor(
    span: &SourceSpan,
    dtor: &Name,
    symbol_table: &SymbolTable,
) -> Result<Ty, Error> {
    for (ty_ctor, (pol, xtors)) in &symbol_table.ty_ctors {
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
    span: &SourceSpan,
    ctor: &Name,
    symbol_table: &SymbolTable,
) -> Result<(Ty, HashSet<String>), Error> {
    for (ty_ctor, (pol, xtors)) in &symbol_table.ty_ctors {
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

#[cfg(test)]
mod decl_tests {
    use super::{
        check_codata_declaration, check_data_declaration, check_definition, lookup_ty_for_ctor,
        lookup_ty_for_dtor,
    };
    use crate::{
        parser::util::ToMiette,
        syntax::{
            context::ContextBinding,
            declarations::{CodataDeclaration, CtorSig, DataDeclaration, Definition, DtorSig},
            substitution::SubstitutionBinding,
            terms::{Constructor, Lit},
            types::Ty,
        },
        typing::symbol_table::{BuildSymbolTable, Polarity, SymbolTable},
    };
    use codespan::Span;
    use std::collections::HashSet;
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
                            ty: None,
                        }
                        .into(),
                    ),
                ],
                ty: None,
            }
            .into(),
        }
    }

    #[test]
    fn data_check() {
        let mut symbol_table = SymbolTable::default();
        example_data().build(&mut symbol_table).unwrap();
        let result = check_data_declaration(&example_data(), &symbol_table);
        assert!(result.is_ok())
    }
    #[test]
    fn codata_check() {
        let mut symbol_table = SymbolTable::default();
        example_codata().build(&mut symbol_table).unwrap();
        let result = check_codata_declaration(&example_codata(), &symbol_table);
        assert!(result.is_ok())
    }
    #[test]
    fn def_check() {
        let mut symbol_table = SymbolTable::default();
        example_def().build(&mut symbol_table).unwrap();
        example_data().build(&mut symbol_table).unwrap();
        let result = check_definition(example_def(), &symbol_table).unwrap();
        let expected = Definition {
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
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn dtor_lookup() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "LPairIntInt".to_owned(),
            (Polarity::Codata, vec!["Fst".to_owned(), "Snd".to_owned()]),
        );
        let result = lookup_ty_for_dtor(
            &Span::default().to_miette(),
            &"Fst".to_owned(),
            &symbol_table,
        )
        .unwrap();
        let expected = Ty::mk_decl("LPairIntInt");
        assert_eq!(result, expected)
    }
    #[test]
    fn dtor_lookup_fail() {
        let result = lookup_ty_for_dtor(
            &Span::default().to_miette(),
            &"Snd".to_owned(),
            &SymbolTable::default(),
        );
        assert!(result.is_err())
    }
    #[test]
    fn ctor_lookup() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        let result = lookup_ty_for_ctor(
            &Span::default().to_miette(),
            &"Nil".to_owned(),
            &symbol_table,
        )
        .unwrap();
        let expected = (
            Ty::mk_decl("ListInt"),
            HashSet::from(["Nil".to_owned(), "Cons".to_owned()]),
        );
        assert_eq!(result, expected)
    }
    #[test]
    fn ctor_lookup_fail() {
        let result = lookup_ty_for_ctor(
            &Span::default().to_miette(),
            &"Nil".to_owned(),
            &SymbolTable::default(),
        );
        assert!(result.is_err())
    }
}
