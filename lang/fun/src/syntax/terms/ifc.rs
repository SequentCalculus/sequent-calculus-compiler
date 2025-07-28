//! Defines [IfC]
use codespan::Span;
use derivative::Derivative;
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{ELSE, EQQ, GT, GTE, IF, LT, LTE, NEQ, ZERO},
    util::BracesExt,
};

use super::Term;
use crate::{
    syntax::{
        Var,
        context::TypingContext,
        types::{OptTyped, Ty},
    },
    traits::used_binders::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

/// The conditional operator in an [if][IfC] expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfSort {
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// <
    Less,
    /// <=
    LessOrEqual,
    /// >
    Greater,
    /// >=
    GreaterOrEqual,
}

/// An if expression
/// Example: `if n == 0 {1} else {n * fac(n - 1 )}`
/// when `n` is `0` return `1` else calculate `n * fac(n-1)`
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct IfC {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The conditional operator
    pub sort: IfSort,
    /// The left-hand side of the comparison
    pub fst: Rc<Term>,
    /// The right-hand side of the comparison
    pub snd: Option<Rc<Term>>,
    /// The then-branch of the condition
    pub thenc: Rc<Term>,
    /// The else-branch of the condition
    pub elsec: Rc<Term>,
    /// The type of the term (inferred)
    pub ty: Option<Ty>,
}

impl OptTyped for IfC {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for IfC {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let comparison = match self.sort {
            IfSort::Equal => EQQ,
            IfSort::NotEqual => NEQ,
            IfSort::Less => LT,
            IfSort::LessOrEqual => LTE,
            IfSort::Greater => GT,
            IfSort::GreaterOrEqual => GTE,
        };
        let snd = match self.snd {
            None => alloc.text(ZERO),
            Some(ref snd) => snd.print(cfg, alloc),
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(snd)
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

impl From<IfC> for Term {
    fn from(value: IfC) -> Self {
        Term::IfC(value)
    }
}

impl Check for IfC {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        self.fst = self.fst.check(symbol_table, context, &Ty::mk_i64())?;
        self.snd = self.snd.check(symbol_table, context, &Ty::mk_i64())?;
        self.thenc = self.thenc.check(symbol_table, context, expected)?;
        self.elsec = self.elsec.check(symbol_table, context, expected)?;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for IfC {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.fst.used_binders(used);
        self.snd.used_binders(used);
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
    use crate::syntax::terms::IfSort;
    use crate::{
        syntax::{
            terms::{IfC, Lit, XVar},
            types::{Ty, TypeArgs},
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_ife() {
        let result = IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(Lit::mk(2).into()),
            snd: Some(Rc::new(Lit::mk(1).into())),
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
        let expected = IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(Lit::mk(2).into()),
            snd: Some(Rc::new(Lit::mk(1).into())),
            thenc: Rc::new(Lit::mk(2).into()),
            elsec: Rc::new(Lit::mk(3).into()),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_ife_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let result = IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(XVar::mk("x").into()),
            snd: Some(Rc::new(XVar::mk("x").into())),
            thenc: Rc::new(Lit::mk(1).into()),
            elsec: Rc::new(Lit::mk(2).into()),
            ty: None,
        }
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64());
        assert!(result.is_err())
    }

    fn example() -> IfC {
        IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(Term::Lit(Lit::mk(1))),
            snd: Some(Rc::new(Term::Lit(Lit::mk(1)))),
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "if 1 == 1 {\n    2\n} else {\n    4\n}"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("if 1 == 1 {2 } else { 4}"),
            Ok(example().into())
        );
    }

    #[test]
    fn check_ifz() {
        let result = IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(Lit::mk(1).into()),
            snd: None,
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
        let expected = IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(Lit::mk(1).into()),
            snd: None,
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
        let result = IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(XVar::mk("x").into()),
            snd: None,
            thenc: Rc::new(Lit::mk(1).into()),
            elsec: Rc::new(Lit::mk(2).into()),
            ty: None,
        }
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64());
        assert!(result.is_err())
    }

    fn example_zero() -> IfC {
        IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(Term::Lit(Lit::mk(0))),
            snd: None,
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    fn example_zero_not() -> IfC {
        IfC {
            span: Span::default(),
            sort: IfSort::NotEqual,
            fst: Rc::new(Term::Lit(Lit::mk(1))),
            snd: None,
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display_zero() {
        assert_eq!(
            example_zero().print_to_string(Default::default()),
            "if 0 == 0 {\n    2\n} else {\n    4\n}"
        )
    }

    #[test]
    fn display_zero_not() {
        assert_eq!(
            example_zero_not().print_to_string(Default::default()),
            "if 1 != 0 {\n    2\n} else {\n    4\n}"
        )
    }

    #[test]
    fn parse_zero() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("if 0 == 0 { 2} else {4 }"),
            Ok(example_zero().into())
        );
    }

    #[test]
    fn parse_zero_not() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("if 1 != 0 { 2} else {4 }"),
            Ok(example_zero_not().into())
        );
    }
}
