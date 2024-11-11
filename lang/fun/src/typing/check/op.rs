use super::{check_equality, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Op, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Op {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        check_equality(&self.span.to_miette(), &Ty::mk_int(), expected)?;
        // In the following two cases we know that "expected = Int".
        let fst_checked = self.fst.check(symbol_table, context, expected)?;
        let snd_checked = self.snd.check(symbol_table, context, expected)?;
        Ok(Op {
            span: self.span,
            fst: fst_checked,
            op: self.op,
            snd: snd_checked,
        })
    }
}

#[cfg(test)]
mod op_test {
    use super::Check;
    use crate::{
        syntax::{
            terms::{BinOp, Lit, Op},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use std::rc::Rc;
    #[test]
    fn check_op() {
        let result = Op {
            span: Span::default(),
            fst: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            op: BinOp::Sum,
            snd: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_int())
        .unwrap();
        let expected = Op {
            span: Span::default(),
            fst: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            op: BinOp::Sum,
            snd: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn check_op_fail() {
        let result = Op {
            span: Span::default(),
            fst: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
            op: BinOp::Sub,
            snd: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_decl("ListInt"));
        assert!(result.is_err())
    }
}
