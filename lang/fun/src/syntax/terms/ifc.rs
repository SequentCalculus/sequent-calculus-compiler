use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{ELSE, EQQ, IF, LT, LTE, NEQ},
    util::BracesExt,
    DocAllocator, Print,
};

use super::Term;
use crate::{
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
        used_binders::UsedBinders,
        Variable,
    },
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfSort {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
}

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct IfC {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub sort: IfSort,
    pub fst: Rc<Term>,
    pub snd: Rc<Term>,
    pub thenc: Rc<Term>,
    pub elsec: Rc<Term>,
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
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
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
        self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let fst_checked = self.fst.check(symbol_table, context, &Ty::mk_i64())?;
        let snd_checked = self.snd.check(symbol_table, context, &Ty::mk_i64())?;
        let thenc_checked = self.thenc.check(symbol_table, context, expected)?;
        let elsec_checked = self.elsec.check(symbol_table, context, expected)?;
        Ok(IfC {
            fst: fst_checked,
            snd: snd_checked,
            thenc: thenc_checked,
            elsec: elsec_checked,
            ty: Some(expected.clone()),
            ..self
        })
    }
}

impl UsedBinders for IfC {
    fn used_binders(&self, used: &mut HashSet<Variable>) {
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
            snd: Rc::new(Lit::mk(1).into()),
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
            snd: Rc::new(Lit::mk(1).into()),
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
            snd: Rc::new(XVar::mk("x").into()),
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
            snd: Rc::new(Term::Lit(Lit::mk(1))),
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
}
