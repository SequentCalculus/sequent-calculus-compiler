use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::{
    TypingContext,
    statements::{Call, Clause, Statement},
};
use std::rc::Rc;

impl Rewrite for Call {
    type Target = Self;
    fn rewrite(mut self, ctx: &mut RewriteState) -> Self::Target {
        let called_ind = match ctx
            .lifted_statements
            .iter()
            .position(|df| df.name == self.label)
        {
            None => return self,
            Some(ind) => ind,
        };

        let mut called_def = ctx.lifted_statements.remove(called_ind);
        let mut switch = match called_def.body {
            Statement::Switch(sw) => sw,
            _ => {
                ctx.add_def(called_def);
                return self;
            }
        };

        let switch_arg_ind = called_def
            .context
            .bindings
            .iter()
            .position(|bind| bind.var == switch.var)
            .expect("Could not find switch variable");

        let call_arg = self.args.bindings.remove(switch_arg_ind);
        let (let_xtor, let_args) = match ctx.get_let(&call_arg.var) {
            None => {
                called_def.body = switch.into();
                ctx.add_def(called_def);
                return self;
            }
            Some(lt) => lt,
        };
        self.args.bindings.extend(let_args.bindings);

        let switch_clause_ind = switch
            .clauses
            .iter()
            .position(|clause| clause.xtor == let_xtor)
            .expect("Could not find clause for xtor");
        let switch_clause = switch.clauses.remove(switch_clause_ind);
        let (lifted_name, lifted_args) = match &*switch_clause.body {
            Statement::Call(call) => (call.label.clone(), call.args.bindings.clone()),
            _ => ctx.lift_switch_call(&called_def.name, &switch.var, &switch_clause),
        };

        let new_clause = Clause {
            xtor: switch_clause.xtor,
            context: switch_clause.context,
            body: Rc::new(
                Call {
                    label: lifted_name.clone(),
                    args: TypingContext {
                        bindings: lifted_args.into_iter().collect(),
                    },
                }
                .into(),
            ),
        };
        switch.clauses.insert(switch_arg_ind, new_clause);
        called_def.body = switch.into();
        ctx.add_def(called_def);
        Call {
            label: lifted_name,
            args: self.args,
        }
    }
}
