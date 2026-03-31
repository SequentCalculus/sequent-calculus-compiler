//! This module defines arithmetic binary operations in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{DIVIDE, MINUS, MODULO, PLUS, TIMES};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::Inference;
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
impl Check for Op {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        check_equality(&self.span, symbol_table, &Ty::mk_i64(), expected)?;
        self.fst = self.fst.check(symbol_table, context, &Ty::mk_i64())?;
        self.snd = self.snd.check(symbol_table, context, &Ty::mk_i64())?;

        Ok(self)
    }
}

impl Inference for Op {
    fn constraint_equations(
            &mut self,
            symbol_table: &mut SymbolTable,
            context: &TypingContext,
            var_name_generator: &mut inference::VarNameGenerator,
            ty_var: Ty
        ) -> Result<Vec<(Ty,Ty)>, Error> {
        let mut constraints: Vec<(Ty, Ty)> = vec![(ty_var, Ty::mk_i64())];

        constraints.append(&mut self.fst.constraint_equations(symbol_table, context, var_name_generator, Ty::mk_i64())?);
        constraints.append(&mut self.snd.constraint_equations(symbol_table, context, var_name_generator, Ty::mk_i64())?);

        Ok(constraints)
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable
    ) -> Result<(), Error> {
        self.fst.insert_inferred_type(mappings, symbol_table)?;
        self.snd.insert_inferred_type(mappings, symbol_table)
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
    use crate::typing::inference::{Inference, VarNameGenerator};
    use crate::typing::*;

    use std::rc::Rc;

    #[test]
    fn check_op() {
        let result = Op {
            span: dummy_span(),
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
            span: dummy_span(),
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
            span: dummy_span(),
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

    #[test]
    fn inference_op() {
        let mut term = Op {
            span: dummy_span(),
            fst: Rc::new(Lit::mk(2).into()),
            op: BinOp::Sum,
            snd: Rc::new(Lit::mk(3).into()),
        };

        let result = term.constraint_equations(&mut SymbolTable::default(), &TypingContext::default(), &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let expected = vec![(Ty::mk_ty_var("x"), Ty::mk_i64()), (Ty::mk_i64(), Ty::mk_i64()),(Ty::mk_i64(), Ty::mk_i64())];

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
