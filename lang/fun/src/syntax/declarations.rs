use std::collections::HashSet;

use codespan::Span;
use derivative::Derivative;
use printer::theme::ThemeExt;
use printer::tokens::{CODATA, COLON, COLONEQ, COMMA, DATA, DEF, SEMI};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use crate::syntax::terms::Term;
use crate::syntax::{context::TypingContext, Name};

use super::empty_braces;
use super::types::Ty;

// Definition
//
//

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

impl Print for Definition {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(self.name.clone())
            .append(self.context.print(cfg, alloc).parens())
            .append(alloc.space())
            .append(COLON)
            .append(alloc.space())
            .append(self.ret_ty.print(cfg, alloc))
            .append(alloc.space())
            .append(COLONEQ)
            .append(alloc.space())
            .append(self.body.print(cfg, alloc))
            .append(SEMI)
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
            declarations::Module,
            terms::{Lit, Term},
            types::Ty,
        },
    };

    use super::Definition;

    /// A definition with no arguments:
    fn simple_definition() -> Definition {
        Definition {
            span: Span::default(),
            name: "x".to_string(),
            context: vec![],
            body: Term::Lit(Lit::mk(4)),
            ret_ty: Ty::mk_int(),
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            simple_definition().print_to_string(Default::default()),
            "def x() : Int := 4;".to_string()
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
}

// DataDeclaration
//
//

#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct CtorSig {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub args: TypingContext,
}

#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct DataDeclaration {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub ctors: Vec<CtorSig>,
}

impl From<DataDeclaration> for Declaration {
    fn from(data: DataDeclaration) -> Declaration {
        Declaration::DataDeclaration(data)
    }
}

impl Print for DataDeclaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = alloc
            .keyword(DATA)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(alloc.space());

        let sep = alloc.text(COMMA).append(alloc.line());

        let body = if self.ctors.is_empty() {
            empty_braces(alloc)
        } else {
            alloc
                .line()
                .append(
                    alloc.intersperse(self.ctors.iter().map(|ctor| ctor.print(cfg, alloc)), sep),
                )
                .nest(cfg.indent)
                .append(alloc.line())
                .braces_anno()
        };

        head.append(body.group())
    }
}

impl Print for CtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .ctor(&self.name)
            .append(self.args.print(cfg, alloc).parens())
    }
}

#[cfg(test)]
mod data_declaration_tests {
    use codespan::Span;
    use printer::Print;

    use crate::syntax::{context::ContextBinding, types::Ty};

    use super::{CtorSig, DataDeclaration};

    /// Lists containing Int
    fn example_list() -> DataDeclaration {
        let nil = CtorSig {
            span: Span::default(),
            name: "Nil".to_owned(),
            args: vec![],
        };
        let cons = CtorSig {
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
        };

        DataDeclaration {
            span: Span::default(),
            name: "ListInt".to_owned(),
            ctors: vec![nil, cons],
        }
    }

    #[test]
    fn display_list() {
        let result = example_list().print_to_string(Default::default());
        let expected = "data ListInt { Nil(), Cons(x : Int, xs : ListInt) }";
        assert_eq!(result, expected)
    }
}

// CodataDeclaration
//
//

#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct DtorSig {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub args: TypingContext,
    pub cont_ty: Ty,
}

#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct CodataDeclaration {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub dtors: Vec<DtorSig>,
}

impl From<CodataDeclaration> for Declaration {
    fn from(codata: CodataDeclaration) -> Declaration {
        Declaration::CodataDeclaration(codata)
    }
}

impl Print for CodataDeclaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = alloc
            .keyword(CODATA)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(alloc.space());

        let sep = alloc.text(COMMA).append(alloc.line());

        let body = if self.dtors.is_empty() {
            empty_braces(alloc)
        } else {
            alloc
                .line()
                .append(
                    alloc.intersperse(self.dtors.iter().map(|dtor| dtor.print(cfg, alloc)), sep),
                )
                .nest(cfg.indent)
                .append(alloc.line())
                .braces_anno()
        };

        head.append(body.group())
    }
}

impl Print for DtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .dtor(&self.name)
            .append(self.args.print(cfg, alloc).parens())
            .append(alloc.space())
            .append(COLON)
            .append(alloc.space())
            .append(self.cont_ty.print(cfg, alloc))
    }
}

#[cfg(test)]
mod codata_declaration_tests {
    use codespan::Span;
    use printer::Print;

    use crate::syntax::{context::ContextBinding, types::Ty};

    use super::{CodataDeclaration, DtorSig};

    // Streams
    fn example_stream() -> CodataDeclaration {
        let hd = DtorSig {
            span: Span::default(),
            name: "hd".to_owned(),
            args: vec![],
            cont_ty: Ty::mk_int(),
        };
        let tl = DtorSig {
            span: Span::default(),
            name: "tl".to_owned(),
            args: vec![],
            cont_ty: Ty::mk_decl("IntStream"),
        };

        CodataDeclaration {
            span: Span::default(),
            name: "IntStream".to_owned(),
            dtors: vec![hd, tl],
        }
    }

    #[test]
    fn display_stream() {
        let result = example_stream().print_to_string(Default::default());
        let expected = "codata IntStream { hd() : Int, tl() : IntStream }";
        assert_eq!(result, expected)
    }

    // Functions from Int to Int
    fn example_fun() -> CodataDeclaration {
        let ap = DtorSig {
            span: Span::default(),
            name: "ap".to_owned(),
            args: vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
            cont_ty: Ty::mk_int(),
        };

        CodataDeclaration {
            span: Span::default(),
            name: "Fun".to_owned(),
            dtors: vec![ap],
        }
    }

    #[test]
    fn display_fun() {
        let result = example_fun().print_to_string(Default::default());
        let expected = "codata Fun { ap(x : Int) : Int }";
        assert_eq!(result, expected)
    }
}

// Declaration
//
//

/// A toplevel declaration in a module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Definition(Definition),
    DataDeclaration(DataDeclaration),
    CodataDeclaration(CodataDeclaration),
}

impl Print for Declaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Declaration::Definition(definition) => definition.print(cfg, alloc),
            Declaration::DataDeclaration(data_declaration) => data_declaration.print(cfg, alloc),
            Declaration::CodataDeclaration(codata_declaration) => {
                codata_declaration.print(cfg, alloc)
            }
        }
    }
}

// Module
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub declarations: Vec<Declaration>,
}

impl Module {
    pub fn data_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for decl in &self.declarations {
            if let Declaration::DataDeclaration(data) = decl {
                names.insert(data.name.clone());
            }
        }

        names
    }

    pub fn codata_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for decl in &self.declarations {
            if let Declaration::CodataDeclaration(codata) = decl {
                names.insert(codata.name.clone());
            }
        }
        names
    }
}

impl Print for Module {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        // We usually separate declarations with an empty line, except when the `omit_decl_sep` option is set.
        // This is useful for typesetting examples in papers which have to make economic use of vertical space.
        let sep = if cfg.omit_decl_sep {
            alloc.line()
        } else {
            alloc.line().append(alloc.line())
        };

        let decls = self.declarations.iter().map(|decl| decl.print(cfg, alloc));

        alloc.intersperse(decls, sep)
    }
}

#[cfg(test)]
mod module_tests {
    use codespan::Span;
    use printer::Print;

    use super::{Definition, Module, Term};
    use crate::{
        parser::fun,
        syntax::{context::ContextBinding, terms::Lit, types::Ty},
    };
    use std::collections::HashSet;

    // Program with one definition without arguments
    //
    //

    fn example_simple() -> Module {
        Module {
            declarations: vec![Definition {
                span: Span::default(),
                name: "x".to_string(),
                context: vec![],
                body: Term::Lit(Lit::mk(4)),
                ret_ty: Ty::mk_int(),
            }
            .into()],
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            example_simple().print_to_string(Default::default()),
            "def x() : Int := 4;".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def x() : Int := 4;"),
            Ok(example_simple().into())
        );
    }

    #[test]
    fn data_simple() {
        let result = example_simple().data_types();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn codata_simple() {
        let result = example_simple().codata_types();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    // Program with one definition which takes arguments
    //
    //

    fn example_args() -> Module {
        Module {
            declarations: vec![Definition {
                span: Span::default(),
                name: "f".to_string(),
                context: vec![
                    ContextBinding::TypedVar {
                        var: "x".to_string(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedCovar {
                        covar: "a".to_owned(),
                        ty: Ty::mk_int(),
                    },
                ],
                body: Term::Lit(Lit::mk(4)),
                ret_ty: Ty::mk_int(),
            }
            .into()],
        }
    }

    #[test]
    fn display_args() {
        assert_eq!(
            example_args().print_to_string(Default::default()),
            "def f(x : Int, 'a :cnt Int) : Int := 4;".to_string(),
        )
    }

    #[test]
    fn parse_args() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(x : Int, 'a :cnt Int) : Int := 4;"),
            Ok(example_args().into())
        )
    }

    // Program with two definitions
    //
    //

    fn example_two() -> Module {
        let d1 = Definition {
            span: Span::default(),
            name: "f".to_string(),
            context: vec![],
            body: Term::Lit(Lit::mk(2)),
            ret_ty: Ty::mk_int(),
        };

        let d2 = Definition {
            span: Span::default(),
            name: "g".to_string(),
            context: vec![],
            body: Term::Lit(Lit::mk(4)),
            ret_ty: Ty::mk_int(),
        };
        Module {
            declarations: vec![d1.into(), d2.into()],
        }
    }

    #[test]
    fn display_two() {
        assert_eq!(
            example_two().print_to_string(Default::default()),
            "def f() : Int := 2;\n\ndef g() : Int := 4;".to_string(),
        )
    }

    #[test]
    fn parse_two() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f() : Int := 2;\n def g() : Int := 4;"),
            Ok(example_two().into())
        )
    }
}
