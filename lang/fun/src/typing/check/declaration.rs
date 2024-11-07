use super::{check_type, context::check_typing_context, terms::Check};
use crate::{
    syntax::declarations::{
        CodataDeclaration, CtorSig, DataDeclaration, Declaration, Definition, DtorSig, Module,
    },
    typing::{
        errors::Error,
        symbol_table::{build_symbol_table, SymbolTable},
    },
};

pub fn check_module(module: Module) -> Result<Module, Error> {
    let symbol_table = build_symbol_table(&module)?;
    check_module_with_table(module, &symbol_table)
}

fn check_module_with_table(module: Module, symbol_table: &SymbolTable) -> Result<Module, Error> {
    let mut new_decls = vec![];
    for decl in module.declarations.into_iter() {
        let new_decl = check_declaration(decl, symbol_table)?;
        new_decls.push(new_decl);
    }
    Ok(Module {
        declarations: new_decls,
    })
}

// Checking toplevel declarations
//
//

pub fn check_declaration(
    decl: Declaration,
    symbol_table: &SymbolTable,
) -> Result<Declaration, Error> {
    match decl {
        Declaration::Definition(definition) => {
            Ok(check_definition(definition, symbol_table)?.into())
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
        span: def.span,
        name: def.name,
        context: def.context,
        ret_ty: def.ret_ty,
    })
}

fn check_data_declaration(decl: &DataDeclaration, symbol_table: &SymbolTable) -> Result<(), Error> {
    for ctor in decl.ctors.iter() {
        check_ctor_sig(&ctor, symbol_table)?;
    }
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

fn check_ctor_sig(ctor: &CtorSig, symbol_table: &SymbolTable) -> Result<(), Error> {
    check_typing_context(&ctor.args, symbol_table)?;
    Ok(())
}

fn check_dtor_sig(dtor: &DtorSig, symbol_table: &SymbolTable) -> Result<(), Error> {
    check_typing_context(&dtor.args, symbol_table)?;
    check_type(&dtor.cont_ty, symbol_table)?;
    Ok(())
}

#[cfg(test)]
mod decl_tests {
    use super::{check_codata_declaration, check_data_declaration, check_definition, check_module};
    use crate::{
        syntax::{
            context::ContextBinding,
            declarations::{
                CodataDeclaration, CtorSig, DataDeclaration, Definition, DtorSig, Module,
            },
            substitution::SubstitutionBinding,
            terms::{Constructor, Lit},
            types::Ty,
        },
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
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
                    SubstitutionBinding::TermBinding {
                        term: Lit {
                            span: Span::default(),
                            val: 1,
                        }
                        .into(),
                        ty: None,
                    },
                    SubstitutionBinding::TermBinding {
                        term: Constructor {
                            span: Span::default(),
                            id: "Nil".to_owned(),
                            args: vec![],
                            ty: None,
                        }
                        .into(),
                        ty: None,
                    },
                ],
                ty: None,
            }
            .into(),
        }
    }

    #[test]
    fn module_check() {
        let result = check_module(Module {
            declarations: vec![
                example_data().into(),
                example_codata().into(),
                example_def().into(),
            ],
        })
        .unwrap();
        let expected = Module {
            declarations: vec![
                example_data().into(),
                example_codata().into(),
                Definition {
                    span: Span::default(),
                    name: "main".to_owned(),
                    context: vec![],
                    ret_ty: Ty::mk_decl("ListInt"),
                    body: Constructor {
                        span: Span::default(),
                        id: "Cons".to_owned(),
                        args: vec![
                            SubstitutionBinding::TermBinding {
                                term: Lit {
                                    span: Span::default(),
                                    val: 1,
                                }
                                .into(),
                                ty: Some(Ty::mk_int()),
                            },
                            SubstitutionBinding::TermBinding {
                                term: Constructor {
                                    span: Span::default(),
                                    id: "Nil".to_owned(),
                                    args: vec![],
                                    ty: Some(Ty::mk_decl("ListInt")),
                                }
                                .into(),
                                ty: Some(Ty::mk_decl("ListInt")),
                            },
                        ],
                        ty: Some(Ty::mk_decl("ListInt")),
                    }
                    .into(),
                }
                .into(),
            ],
        };
        assert_eq!(result, expected)
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
                    SubstitutionBinding::TermBinding {
                        term: Lit {
                            span: Span::default(),
                            val: 1,
                        }
                        .into(),
                        ty: Some(Ty::mk_int()),
                    },
                    SubstitutionBinding::TermBinding {
                        term: Constructor {
                            span: Span::default(),
                            id: "Nil".to_owned(),
                            args: vec![],
                            ty: Some(Ty::mk_decl("ListInt")),
                        }
                        .into(),
                        ty: Some(Ty::mk_decl("ListInt")),
                    },
                ],
                ty: Some(Ty::mk_decl("ListInt")),
            }
            .into(),
        };
        assert_eq!(result, expected)
    }
}
