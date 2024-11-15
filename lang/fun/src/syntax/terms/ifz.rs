use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFZ},
    DocAllocator, Print,
};

use super::Term;
use crate::{
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
    },
    typing::{check::terms::Check, errors::Error, symbol_table::SymbolTable},
};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct IfZ {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub ifc: Rc<Term>,
    pub thenc: Rc<Term>,
    pub elsec: Rc<Term>,
    pub ty: Option<Ty>,
}

impl OptTyped for IfZ {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for IfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(IFZ).append(
            self.ifc
                .print(cfg, alloc)
                .append(COMMA)
                .append(alloc.space())
                .append(self.thenc.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(self.elsec.print(cfg, alloc))
                .parens(),
        )
    }
}

impl From<IfZ> for Term {
    fn from(value: IfZ) -> Self {
        Term::IfZ(value)
    }
}

impl Check for IfZ {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let ifc_checked = self.ifc.check(symbol_table, context, &Ty::mk_int())?;
        let thenc_checked = self.thenc.check(symbol_table, context, expected)?;
        let elsec_checked = self.elsec.check(symbol_table, context, expected)?;
        Ok(IfZ {
            ifc: ifc_checked,
            thenc: thenc_checked,
            elsec: elsec_checked,
            ty: Some(expected.clone()),
            ..self
        })
    }
}

#[cfg(test)]
mod test {
    use super::Check;
    use super::Term;
    use crate::parser::fun;
    use crate::{
        syntax::{
            context::ContextBinding,
            terms::{IfZ, Lit, Var},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_ifz() {
        let result = IfZ {
            span: Span::default(),
            ifc: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            thenc: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
            elsec: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 3,
                }
                .into(),
            ),
            ty: None,
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_int())
        .unwrap();
        let expected = IfZ {
            span: Span::default(),
            ifc: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            thenc: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
            elsec: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 3,
                }
                .into(),
            ),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_ifz_fail() {
        let result = IfZ {
            span: Span::default(),
            ifc: Rc::new(
                Var {
                    span: Span::default(),
                    ty: None,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            thenc: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            elsec: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
            ty: None,
        }
        .check(
            &SymbolTable::default(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_decl("ListInt"),
            }],
            &Ty::mk_int(),
        );
        assert!(result.is_err())
    }

    fn example() -> IfZ {
        IfZ {
            span: Span::default(),
            ifc: Rc::new(Term::Lit(Lit::mk(0))),
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "ifz(0, 2, 4)"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("ifz(0, 2, 4)"), Ok(example().into()));
    }
}
