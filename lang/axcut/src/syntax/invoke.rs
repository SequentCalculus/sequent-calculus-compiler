use super::{names::freshen, stringify_and_join, Name, Statement, Ty, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use printer::Print;


use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invoke {
    pub var: Var,
    pub tag: Name,
    pub ty: Ty,
    pub args: Vec<Var>,
}

impl std::fmt::Display for Invoke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = if self.args.is_empty() {
            String::new()
        } else {
            "(".to_string() + &stringify_and_join(&self.args, ", ") + ")"
        };
        write!(f, "invoke {} {}{args}", self.var, self.tag)
    }
}

impl Print for Invoke {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        todo!()
    }
}

impl From<Invoke> for Statement {
    fn from(value: Invoke) -> Self {
        Statement::Invoke(value)
    }
}

impl FreeVars for Invoke {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.args.free_vars(vars);
        vars.insert(self.var.clone());
    }
}

impl Subst for Invoke {
    type Target = Invoke;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Invoke {
        Invoke {
            var: self.var.subst_sim(subst),
            args: self.args.subst_sim(subst),
            ..self
        }
    }
}

impl Linearizing for Invoke {
    type Target = crate::syntax::Substitute;
    fn linearize(
        self,
        _context: Vec<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> crate::syntax::Substitute {
        let freshened_context = freshen(&self.args, HashSet::new(), used_vars);

        let mut rearrange: Vec<(Var, Var)> = freshened_context.into_iter().zip(self.args).collect();

        rearrange.push((self.var.clone(), self.var.clone()));

        crate::syntax::Substitute {
            rearrange,
            next: Rc::new(
                crate::syntax::Invoke {
                    var: self.var,
                    tag: self.tag,
                    ty: self.ty,
                    args: vec![],
                }
                .into(),
            ),
        }
    }
}
