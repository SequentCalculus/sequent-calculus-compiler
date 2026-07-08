//! This module defines arithmetic binary operations in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{DIVIDE, MINUS, MODULO, PLUS, TIMES};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::{Constraint, ConstraintBank, Inference};
use crate::typing::*;

use std::collections::HashMap;
use std::{collections::HashSet, rc::Rc};

/// This enum encodes the different kinds of arithmetic binary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    /// Division `/`
    Div,
    /// Multiplication `*`
    Prod,
    /// Remainder `%`
    Rem,
    /// Addition `+`
    Sum,
    /// Subtraction `-`
    Sub,
}

impl Print for BinOp {
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            BinOp::Div => alloc.text(DIVIDE),
            BinOp::Prod => alloc.text(TIMES),
            BinOp::Rem => alloc.text(MODULO),
            BinOp::Sum => alloc.text(PLUS),
            BinOp::Sub => alloc.text(MINUS),
        }
    }
}

/// This struct defines arithmetic binary operations in Fun. It consists of the input terms and the
/// kind of the binary operator.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Op {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The first operand
    pub fst: Rc<Term>,
    /// The kind of operation
    pub op: BinOp,
    /// The second operand
    pub snd: Rc<Term>,
}

impl OptTyped for Op {
    fn get_type(&self) -> Option<Ty> {
        Some(Ty::mk_i64())
    }
}

impl Print for Op {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.fst
            .print(cfg, alloc)
            .group()
            .append(alloc.space())
            .append(self.op.print(cfg, alloc))
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc).group())
    }
}

impl From<Op> for Term {
    fn from(value: Op) -> Self {
        Term::Op(value)
    }
}

impl Inference for Op {
    fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
        context: &TypingContext,
        ty_var: Ty,
    ) -> Result<(), Error> {
        constraint_bank
            .constraints
            .push(Constraint::mk_only_ty(ty_var, Ty::mk_i64()));

        self.fst
            .gather_constraints(constraint_bank, context, Ty::mk_i64())?;
        self.snd
            .gather_constraints(constraint_bank, context, Ty::mk_i64())?;

        Ok(())
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>,
    ) -> Result<(), Error> {
        self.fst
            .insert_inferred_type(mappings, symbol_table, choices)?;
        self.snd
            .insert_inferred_type(mappings, symbol_table, choices)
    }
}

impl UsedBinders for Op {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.fst.used_binders(used);
        self.snd.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::typing::inference::{Constraint, ConstraintBank, Inference};

    use std::rc::Rc;

    #[test]
    fn inference_op() {
        let mut term = Op {
            span: dummy_span(),
            fst: Rc::new(Lit::mk(2).into()),
            op: BinOp::Sum,
            snd: Rc::new(Lit::mk(3).into()),
        };

        let mut constraint_bank = ConstraintBank {
            symbol_table: Default::default(),
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        term.gather_constraints(
            &mut constraint_bank,
            &TypingContext::default(),
            Ty::mk_ty_var("x"),
        )
        .unwrap();

        let expected = vec![
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64()),
            Constraint::mk_only_ty(Ty::mk_i64(), Ty::mk_i64()),
            Constraint::mk_only_ty(Ty::mk_i64(), Ty::mk_i64()),
        ];

        let ConstraintBank {
            constraints: result,
            ..
        } = constraint_bank;

        assert_eq!(result, expected);
    }

    fn example_prod() -> Op {
        Op {
            span: dummy_span(),
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
            span: dummy_span(),
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
            span: dummy_span(),
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
            span: dummy_span(),
            fst: Rc::new(
                Paren::mk(Op {
                    span: dummy_span(),
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
