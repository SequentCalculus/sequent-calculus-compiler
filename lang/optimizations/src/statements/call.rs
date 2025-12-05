use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::{
        TypingContext,
        statements::{Call, Clause, Statement},
    },
    traits::{substitution::Subst, typed_free_vars::TypedFreeVars},
};
use std::{collections::BTreeSet, rc::Rc};

impl Rewrite for Call {
    type Target = Statement;
    fn rewrite(mut self, ctx: &mut RewriteState) -> Self::Target {
        let called_ind = match ctx
            .lifted_statements
            .iter()
            .position(|df| df.name == self.label)
        {
            None => return self.into(),
            Some(ind) => ind,
        };

        let mut called_def = ctx.lifted_statements.remove(called_ind);
        let mut switch = match called_def.body {
            Statement::Switch(sw) => sw,
            _ => {
                ctx.add_def(called_def);
                return self.into();
            }
        };

        let switch_arg_ind = called_def
            .context
            .bindings
            .iter()
            .position(|bind| bind.var == switch.var)
            .expect("Could not find switch variable");

        let call_arg = self.args.bindings.remove(switch_arg_ind);
        let (let_xtor, mut let_args) = match ctx.get_let(&call_arg.var) {
            None => {
                called_def.body = switch.into();
                ctx.add_def(called_def);
                self.args.bindings.insert(switch_arg_ind, call_arg);
                return self.into();
            }
            Some(lt) => lt,
        };

        let switch_clause_ind = switch
            .clauses
            .iter()
            .position(|clause| clause.xtor == let_xtor)
            .expect("Could not find clause for xtor");
        let switch_clause = switch.clauses.remove(switch_clause_ind);
        let (lifted_name, lifted_args) = match &*switch_clause.body {
            Statement::Call(_) | Statement::Invoke(_) | Statement::Exit(_) => {
                let mut subst = vec![];
                let mut free = BTreeSet::new();
                switch_clause.typed_free_vars(&mut free);
                self.args.bindings.insert(switch_arg_ind, call_arg);
                for binding in free {
                    let call_pos = called_def
                        .context
                        .bindings
                        .iter()
                        .position(|bind| bind.var == binding.var)
                        .expect("Could not find variable in definition");
                    subst.push((
                        called_def.context.bindings[call_pos].var.clone(),
                        self.args.bindings[call_pos].var.clone(),
                    ));
                }
                for (ind, bind) in switch_clause.context.bindings.iter().enumerate() {
                    subst.push((bind.var.clone(), let_args.bindings[ind].var.clone()));
                }

                switch
                    .clauses
                    .insert(switch_clause_ind, switch_clause.clone());
                called_def.body = switch.into();
                ctx.add_def(called_def);

                return Rc::unwrap_or_clone(switch_clause.body.subst_sim(&subst));
            }
            _ => {
                let_args.bindings.extend(self.args.bindings);
                self.args = let_args;

                ctx.lift_switch_call(
                    &called_def.name,
                    &switch.var,
                    &called_def.context,
                    &called_def.used_vars,
                    &switch_clause,
                )
            }
        };

        let new_clause = Clause {
            xtor: switch_clause.xtor,
            context: switch_clause.context,
            body: Rc::new(
                Call {
                    label: lifted_name.clone(),
                    args: TypingContext {
                        bindings: lifted_args,
                    },
                }
                .into(),
            ),
        };
        switch.clauses.insert(switch_clause_ind, new_clause);
        called_def.body = switch.into();
        ctx.add_def(called_def);
        Call {
            label: lifted_name,
            args: self.args,
        }
        .into()
    }
}
