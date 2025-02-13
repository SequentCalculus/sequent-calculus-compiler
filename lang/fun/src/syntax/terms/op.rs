use codespan::Span;
use derivative::Derivative;
use printer::{
    tokens::{DIVIDE, MINUS, MODULO, PLUS, TIMES},
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
    typing::{
        check::{check_equality, Check},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Div,
    Prod,
    Rem,
    Sum,
    Sub,
}

impl Print for BinOp {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            BinOp::Div => alloc.text(DIVIDE),
            BinOp::Prod => alloc.text(TIMES),
            BinOp::Rem => alloc.text(MODULO),
            BinOp::Sum => alloc.text(PLUS),
            BinOp::Sub => alloc.text(MINUS),
        }
    }
}

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Op {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub fst: Rc<Term>,
    pub op: BinOp,
    pub snd: Rc<Term>,
}

impl OptTyped for Op {
    fn get_type(&self) -> Option<Ty> {
        Some(Ty::mk_i64())
    }
}

impl Print for Op {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.fst
            .print(cfg, alloc)
            .append(alloc.space())
            .append(self.op.print(cfg, alloc))
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
    }
}

impl From<Op> for Term {
    fn from(value: Op) -> Self {
        Term::Op(value)
    }
}
impl Check for Op {
    fn check(
        self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        check_equality(&self.span, symbol_table, &Ty::mk_i64(), expected)?;
        // In the following two cases we know that "expected = i64".
        let fst_checked = self.fst.check(symbol_table, context, expected)?;
        let snd_checked = self.snd.check(symbol_table, context, expected)?;
        Ok(Op {
            fst: fst_checked,
            snd: snd_checked,
            ..self
        })
    }
}

impl UsedBinders for Op {
    fn used_binders(&self, used: &mut HashSet<Variable>) {
        self.fst.used_binders(used);
        self.snd.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use super::Check;
    use super::Term;
    use crate::syntax::context::TypingContext;
    use crate::{parser::fun, syntax::terms::Paren};
    use crate::{
        syntax::{
            terms::{BinOp, Lit, Op},
            types::{Ty, TypeArgs},
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_op() {
        let result = Op {
            span: Span::default(),
            fst: Rc::new(Lit::mk(1).into()),
            op: BinOp::Sum,
            snd: Rc::new(Lit::mk(2).into()),
        }
        .check(
            &mut SymbolTable::default(),
            &TypingContext::default(),
            &Ty::mk_i64(),
        )
        .unwrap();
        let expected = Op {
            span: Span::default(),
            fst: Rc::new(Lit::mk(1).into()),
            op: BinOp::Sum,
            snd: Rc::new(Lit::mk(2).into()),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn check_op_fail() {
        let result = Op {
            span: Span::default(),
            fst: Rc::new(Lit::mk(2).into()),
            op: BinOp::Sub,
            snd: Rc::new(Lit::mk(2).into()),
        }
        .check(
            &mut SymbolTable::default(),
            &TypingContext::default(),
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        assert!(result.is_err())
    }

    fn example_prod() -> Op {
        Op {
            span: Span::default(),
            fst: Rc::new(Term::Lit(Lit::mk(2))),
            op: BinOp::Prod,
            snd: Rc::new(Term::Lit(Lit::mk(4))),
        }
    }

    #[test]
    fn display_prod() {
        assert_eq!(example_prod().print_to_string(Default::default()), "2 * 4")
    }

    #[test]
    fn parse_prod() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("2 * 4"), Ok(example_prod().into()));
    }

    fn example_sum() -> Op {
        Op {
            span: Span::default(),
            fst: Rc::new(Term::Lit(Lit::mk(2))),
            op: BinOp::Sum,
            snd: Rc::new(Term::Lit(Lit::mk(4))),
        }
    }

    #[test]
    fn display_sum() {
        assert_eq!(example_sum().print_to_string(Default::default()), "2 + 4")
    }

    #[test]
    fn parse_sum() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("2 + 4"), Ok(example_sum().into()));
    }

    fn example_sub() -> Op {
        Op {
            span: Span::default(),
            fst: Rc::new(Term::Lit(Lit::mk(2))),
            op: BinOp::Sub,
            snd: Rc::new(Term::Lit(Lit::mk(4))),
        }
    }

    #[test]
    fn display_sub() {
        assert_eq!(example_sub().print_to_string(Default::default()), "2 - 4")
    }

    #[test]
    fn parse_sub() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("2 - 4"), Ok(example_sub().into()));
    }

    /// (2 * 3) * 4
    fn example_parens() -> Op {
        Op {
            span: Span::default(),
            fst: Rc::new(
                Paren::mk(Op {
                    span: Span::default(),
                    fst: Rc::new(Term::Lit(Lit::mk(2))),
                    op: BinOp::Prod,
                    snd: Rc::new(Term::Lit(Lit::mk(3))),
                })
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(Term::Lit(Lit::mk(4))),
        }
    }

    #[test]
    fn display_parens() {
        assert_eq!(
            example_parens().print_to_string(Default::default()),
            "(2 * 3) * 4"
        )
    }

    #[test]
    fn parse_parens() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("(2 * 3) * 4"), Ok(example_parens().into()));
    }
}
