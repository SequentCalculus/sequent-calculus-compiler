use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COLON, COLONEQ, DEF, SEMI},
    DocAllocator, Print,
};

use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Term, types::Ty, Name},
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use super::Declaration;

/// A toplevel function definition in a module.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Definition {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub context: TypingContext,
    pub body: Term,
    pub ret_ty: Ty,
}

impl Definition {
    pub fn check(self, symbol_table: &SymbolTable) -> Result<Definition, Error> {
        self.context.check(symbol_table)?;
        self.context
            .no_dups(&self.span.to_miette(), self.name.clone())?;
        self.ret_ty.check(symbol_table)?;
        let body_checked = self.body.check(symbol_table, &self.context, &self.ret_ty)?;
        Ok(Definition {
            body: body_checked,
            ..self
        })
    }
}
impl Print for Definition {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(self.name.clone())
            .append(self.context.print(cfg, alloc))
            .append(COLON)
            .append(alloc.space())
            .append(self.ret_ty.print(cfg, alloc))
            .append(alloc.space())
            .append(COLONEQ);

        let body = alloc
            .line()
            .append(self.body.print(cfg, alloc))
            .append(SEMI)
            .nest(cfg.indent);

        head.append(body).group()
    }
}

impl From<Definition> for Declaration {
    fn from(value: Definition) -> Self {
        Declaration::Definition(value)
    }
}

#[cfg(test)]
mod definition_tests {
    use codespan::Span;
    use printer::Print;

    use crate::{
        parser::fun,
        syntax::{
            context::{ContextBinding, TypingContext},
            declarations::{CtorSig, DataDeclaration, Module},
            substitution::SubstitutionBinding,
            terms::{Constructor, Lit, Term},
            types::Ty,
        },
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
    };

    use super::Definition;

    /// A definition with no arguments:
    fn simple_definition() -> Definition {
        Definition {
            span: Span::default(),
            name: "x".to_string(),
            context: TypingContext { bindings: vec![] },
            body: Term::Lit(Lit::mk(4)),
            ret_ty: Ty::mk_int(),
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            simple_definition().print_to_string(Default::default()),
            "def x: Int := 4;".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        let module = Module {
            declarations: vec![simple_definition().into()],
        };
        assert_eq!(parser.parse("def x() : Int := 4;"), Ok(module));
    }

    fn example_def() -> Definition {
        Definition {
            span: Span::default(),
            name: "main".to_owned(),
            context: TypingContext { bindings: vec![] },
            ret_ty: Ty::mk_decl("ListInt"),
            body: Constructor {
                span: Span::default(),
                id: "Cons".to_owned(),
                args: vec![
                    SubstitutionBinding::TermBinding(Lit::mk(1).into()),
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

    fn example_data() -> DataDeclaration {
        DataDeclaration {
            span: Span::default(),
            name: "ListInt".to_owned(),
            ctors: vec![
                CtorSig {
                    span: Span::default(),
                    name: "Nil".to_owned(),
                    args: TypingContext { bindings: vec![] },
                },
                CtorSig {
                    span: Span::default(),
                    name: "Cons".to_owned(),
                    args: TypingContext {
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
                },
            ],
        }
    }
    #[test]
    fn def_check() {
        let mut symbol_table = SymbolTable::default();
        example_def().build(&mut symbol_table).unwrap();
        example_data().build(&mut symbol_table).unwrap();
        let result = example_def().check(&symbol_table).unwrap();
        let expected = Definition {
            span: Span::default(),
            name: "main".to_owned(),
            context: TypingContext { bindings: vec![] },
            ret_ty: Ty::mk_decl("ListInt"),
            body: Constructor {
                span: Span::default(),
                id: "Cons".to_owned(),
                args: vec![
                    SubstitutionBinding::TermBinding(Lit::mk(1).into()),
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
}
