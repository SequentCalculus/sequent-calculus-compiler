use printer::tokens::{LEFT_ARROW, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::filter_by_set, BinOp, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Var,
    pub op: BinOp,
    pub snd: Var,
    pub var: Var,
    pub case: Rc<Statement>,
}

impl Print for Op {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.var)
            .append(alloc.space())
            .append(LEFT_ARROW)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(self.op.print(cfg, alloc))
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
            .append(SEMI)
            .append(alloc.space())
            .append(self.case.print(cfg, alloc))
    }
}

impl From<Op> for Statement {
    fn from(value: Op) -> Self {
        Statement::Op(value)
    }
}

impl FreeVars for Op {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        vars.remove(&self.var);
        vars.insert(self.fst.clone());
        vars.insert(self.snd.clone());
    }
}

impl Subst for Op {
    type Target = Op;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Op {
        Op {
            fst: self.fst.subst_sim(subst),
            snd: self.snd.subst_sim(subst),
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for Op {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.var.clone());
        self.case.used_binders(used);
    }
}

impl Linearizing for Op {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.case.free_vars(&mut free_vars);
        free_vars.insert(self.fst.clone());
        free_vars.insert(self.snd.clone());

        let mut new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        new_context.push(self.var.clone());
        let op = Op {
            fst: self.fst,
            op: self.op,
            snd: self.snd,
            var: self.var,
            case: self.case.linearize(new_context, used_vars),
        }
        .into();

        if context == context_rearrange {
            op
        } else {
            let rearrange = context_rearrange
                .clone()
                .into_iter()
                .zip(context_rearrange.clone())
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(op),
            }
            .into()
        }
    }
}

#[cfg(test)]
mod op_tests {
    use super::{FreeVars, Linearizing, Op, Subst, UsedBinders};
    use crate::syntax::{
        statements::{Return, Substitute},
        BinOp,
    };
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_op() -> Op {
        Op {
            fst: "x".to_owned(),
            op: BinOp::Prod,
            snd: "y".to_owned(),
            var: "z".to_owned(),
            case: Rc::new(
                Return {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn print_op() {
        let result = example_op().print_to_string(Default::default());
        let expected = "z <- x * y; return x";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_op() {
        let mut result = HashSet::new();
        example_op().free_vars(&mut result);
        let expected = HashSet::from(["x".to_owned(), "y".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_op() {
        let result = example_op().subst_sim(&vec![
            ("x".to_owned(), "a".to_owned()),
            ("y".to_owned(), "b".to_owned()),
            ("z".to_owned(), "c".to_owned()),
        ]);
        let expected = Op {
            fst: "a".to_owned(),
            op: BinOp::Prod,
            snd: "b".to_owned(),
            var: "z".to_owned(),
            case: Rc::new(
                Return {
                    var: "a".to_owned(),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn used_binders_op() {
        let mut result = HashSet::new();
        example_op().used_binders(&mut result);
        let expected = HashSet::from(["z".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn linearize_op() {
        let result = example_op().linearize(vec![], &mut HashSet::new());
        let expected = Substitute {
            rearrange: vec![],
            next: Rc::new(example_op().into()),
        }
        .into();
        assert_eq!(result, expected)
    }
}
