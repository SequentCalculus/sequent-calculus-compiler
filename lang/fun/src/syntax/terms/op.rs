//! This module defines arithmetic binary operations in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{DIVIDE, MINUS, MODULO, PLUS, TIMES};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::{TypeInference, TypeInferenceState, VariationalConstraint};
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

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

impl TypeInference for Op {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        state
            .constraints
            .push(VariationalConstraint::equal(expected_ty, Ty::mk_i64()));

        self.fst.collect_constraints(state, context, Ty::mk_i64())?;
        self.snd.collect_constraints(state, context, Ty::mk_i64())?;

        Ok(())
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        self.fst
            .insert_inferred_types(type_vars, symbol_table, world)?;
        self.snd
            .insert_inferred_types(type_vars, symbol_table, world)
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
    use crate::typing::inference::{TypeInference, TypeInferenceState, VariationalConstraint};

    use std::rc::Rc;

    #[test]
    fn inference_op() {
        let mut term = Op {
            span: dummy_span(),
            fst: Rc::new(Lit::mk(2).into()),
            op: BinOp::Sum,
            snd: Rc::new(Lit::mk(3).into()),
        };

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_i64()),
            VariationalConstraint::equal(Ty::mk_i64(), Ty::mk_i64()),
            VariationalConstraint::equal(Ty::mk_i64(), Ty::mk_i64()),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

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
