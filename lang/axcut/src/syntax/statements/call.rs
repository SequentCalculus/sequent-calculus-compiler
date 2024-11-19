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
    type Target = Substitute;
    fn linearize(self, _context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Substitute {
        let freshened_context = freshen(&self.args, HashSet::new(), used_vars);
        let rearrange = freshened_context.into_iter().zip(self.args).collect();
        Substitute {
            rearrange,
            next: Rc::new(
                Call {
                    label: self.label,
                    args: vec![],
                }
                .into(),
            ),
        }
    }
}
