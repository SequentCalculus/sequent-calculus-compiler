use printer::{theme::ThemeExt, tokens::JUMP, DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::freshen, Name, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub label: Name,
    pub args: Vec<Var>,
}

impl Print for Call {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        alloc
            .keyword(JUMP)
            .append(alloc.space())
            .append(&self.label)
            .append(args)
    }
}

impl From<Call> for Statement {
    fn from(value: Call) -> Self {
        Statement::Call(value)
    }
}

impl FreeVars for Call {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.args.free_vars(vars);
    }
}

impl Subst for Call {
    type Target = Call;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Call {
        Call {
            label: self.label,
            args: self.args.subst_sim(subst),
        }
    }
}

impl Linearizing for Call {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let call = Call {
            label: self.label,
            args: vec![],
        }
        .into();

        if context == self.args {
            call
        } else {
            let freshened_context = freshen(&self.args, HashSet::new(), used_vars);
            let rearrange = freshened_context.into_iter().zip(self.args).collect();
            Substitute {
                rearrange,
                next: Rc::new(call),
            }
            .into()
        }
    }
}

#[cfg(test)]
mod call_tests {
    use super::{Call, FreeVars, Linearizing, Subst, Substitute};
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_exit() -> Call {
        Call {
            label: "exit".to_owned(),
            args: vec![],
        }
    }
    fn example_mult() -> Call {
        Call {
            label: "mult".to_owned(),
            args: vec!["x".to_owned(), "y".to_owned()],
        }
    }

    #[test]
    fn print_exit() {
        let result = example_exit().print_to_string(Default::default());
        let expected = "jump exit";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_mult() {
        let result = example_mult().print_to_string(Default::default());
        let expected = "jump mult(x, y)";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_exit() {
        let mut result = HashSet::new();
        example_exit().free_vars(&mut result);
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_mult() {
        let mut result = HashSet::new();
        example_mult().free_vars(&mut result);
        let expected = HashSet::from(["x".to_owned(), "y".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_exit() {
        let result = example_exit().subst_sim(&vec![("x".to_owned(), "y".to_owned())]);
        let expected = example_exit();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_mult() {
        let result = example_mult().subst_sim(&vec![("x".to_owned(), "y".to_owned())]);
        let expected = Call {
            label: "mult".to_owned(),
            args: vec!["y".to_owned(), "y".to_owned()],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn linearize_exit() {
        let result = example_exit().linearize(vec![], &mut HashSet::new());
        let expected = Call {
            label: "exit".to_owned(),
            args: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn linearize_mult() {
        let result = example_mult().linearize(vec![], &mut HashSet::new());
        let expected = Substitute {
            rearrange: vec![
                ("x".to_owned(), "x".to_owned()),
                ("y".to_owned(), "y".to_owned()),
            ],
            next: Rc::new(
                Call {
                    label: "mult".to_owned(),
                    args: vec![],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
