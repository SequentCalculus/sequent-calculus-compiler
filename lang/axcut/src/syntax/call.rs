use std::{collections::HashSet, rc::Rc};

use super::{names::freshen, Name, Statement, Substitute, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use printer::{theme::ThemeExt, tokens::JUMP, DocAllocator, Print};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub label: Name,
    pub args: Vec<Var>,
}

impl Print for Call {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(JUMP)
            .append(alloc.space())
            .append(&self.label)
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
    fn linearize(
        self,
        _context: Vec<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> crate::syntax::Substitute {
        let freshened_context = freshen(&self.args, HashSet::new(), used_vars);
        let rearrange = freshened_context.into_iter().zip(self.args).collect();
        crate::syntax::Substitute {
            rearrange,
            next: Rc::new(
                crate::syntax::Call {
                    label: self.label,
                    args: vec![],
                }
                .into(),
            ),
        }
    }
}
