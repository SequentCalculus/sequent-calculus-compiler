use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{ELSE, EQQ, IF, NEQ, ZERO},
    util::BracesExt,
    DocAllocator, Print,
};

use super::Term;
use crate::{
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
        Variable,
    },
    traits::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfZSort {
    Equal,
    NotEqual,
}

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct IfZ {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub sort: IfZSort,
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
        let comparison = match self.sort {
            IfZSort::Equal => EQQ,
            IfZSort::NotEqual => NEQ,
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.ifc.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(ZERO)
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.thenc.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
            .append(alloc.space())
            .append(alloc.keyword(ELSE))
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.elsec.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
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
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let ifc_checked = self.ifc.check(symbol_table, context, &Ty::mk_i64())?;
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

impl UsedBinders for IfZ {
    fn used_binders(&self, used: &mut HashSet<Variable>) {
        self.ifc.used_binders(used);
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use super::Check;
    use super::Term;
    use crate::parser::fun;
    use crate::syntax::context::TypingContext;
    use crate::{
        syntax::{
            terms::{IfZ, IfZSort, Lit, XVar},
            types::{Ty, TypeArgs},
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
            sort: IfZSort::Equal,
            ifc: Rc::new(Lit::mk(1).into()),
            thenc: Rc::new(Lit::mk(2).into()),
            elsec: Rc::new(Lit::mk(3).into()),
            ty: None,
        }
        .check(
            &mut SymbolTable::default(),
            &TypingContext::default(),
            &Ty::mk_i64(),
        )
        .unwrap();
        let expected = IfZ {
            span: Span::default(),
            sort: IfZSort::Equal,
            ifc: Rc::new(Lit::mk(1).into()),
            thenc: Rc::new(Lit::mk(2).into()),
            elsec: Rc::new(Lit::mk(3).into()),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_ifz_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let result = IfZ {
            span: Span::default(),
            sort: IfZSort::Equal,
            ifc: Rc::new(XVar::mk("x").into()),
            thenc: Rc::new(Lit::mk(1).into()),
            elsec: Rc::new(Lit::mk(2).into()),
            ty: None,
        }
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64());
        assert!(result.is_err())
    }

    fn example() -> IfZ {
        IfZ {
            span: Span::default(),
            sort: IfZSort::Equal,
            ifc: Rc::new(Term::Lit(Lit::mk(0))),
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    fn example_not() -> IfZ {
        IfZ {
            span: Span::default(),
            sort: IfZSort::NotEqual,
            ifc: Rc::new(Term::Lit(Lit::mk(1))),
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "if 0 == 0 {\n    2\n} else {\n    4\n}"
        )
    }

    #[test]
    fn display_not() {
        assert_eq!(
            example_not().print_to_string(Default::default()),
            "if 1 != 0 {\n    2\n} else {\n    4\n}"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("if 0 == 0 { 2} else {4 }"),
            Ok(example().into())
        );
    }

    #[test]
    fn parse_not() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("if 1 != 0 { 2} else {4 }"),
            Ok(example_not().into())
        );
    }
}
