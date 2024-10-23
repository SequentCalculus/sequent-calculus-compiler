use super::Statement;
use crate::syntax::context::freshen;
use crate::syntax::{stringify_and_join, ContextBinding, Name, Polarity, Ty, TypingContext, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invoke {
    pub var: Var,
    pub tag: Name,
    pub ty: Ty,
    pub args: TypingContext,
}

impl std::fmt::Display for Invoke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = stringify_and_join(&self.args, ", ");
        writeln!(f, "invoke {} {}({})", self.var, self.tag, args)
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
        _context: TypingContext,
        used_vars: &mut HashSet<Var>,
    ) -> crate::syntax::Substitute {
        let freshened_context = freshen(self.args.clone(), used_vars);

        let mut rearrange: Vec<(ContextBinding, ContextBinding)> =
            freshened_context.into_iter().zip(self.args).collect();

        let object_binding = ContextBinding {
            var: self.var.clone(),
            pol: Polarity::Cns,
            ty: self.ty.clone(),
        };

        rearrange.push((object_binding.clone(), object_binding));

        crate::syntax::Substitute {
            rearrange,
            next: Rc::new(
                crate::syntax::Invoke {
                    var: self.var,
                    tag: self.tag,
                    ty: self.ty,
                }
                .into(),
            ),
        }
    }
}
