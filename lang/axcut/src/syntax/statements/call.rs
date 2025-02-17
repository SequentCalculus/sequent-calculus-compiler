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

    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Call {
        self.args = self.args.subst_sim(subst);
        self
    }
}

impl Linearizing for Call {
    type Target = Statement;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let args = std::mem::take(&mut self.args);

        if context == args {
            self.into()
        } else {
            let freshened_context = freshen(&args, HashSet::new(), used_vars);
            let rearrange = freshened_context.into_iter().zip(args).collect();
            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}
