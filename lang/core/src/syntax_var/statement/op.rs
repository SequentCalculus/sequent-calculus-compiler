use crate::{
    syntax_var::{
        cont_int,
        term::{Mu, Term, XVar},
        BinOp,
        Chirality::Cns,
        Statement, TypeDeclaration, Var,
    },
    traits::{
        shrink::{fresh_var, Shrinking, UsedBinders},
        substitution::SubstVar,
    },
};
use std::{collections::HashSet, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Var,
    pub op: BinOp,
    pub snd: Var,
    pub continuation: Rc<Term>,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({}, {}; {})",
            self.op, self.fst, self.snd, self.continuation
        )
    }
}

impl From<Op> for Statement {
    fn from(value: Op) -> Self {
        Statement::Op(value)
    }
}

impl SubstVar for Op {
    type Target = Op;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        Op {
            fst: self.fst.subst_sim(subst),
            op: self.op,
            snd: self.snd.subst_sim(subst),
            continuation: self.continuation.subst_sim(subst),
        }
    }
}

impl UsedBinders for Op {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.continuation.used_binders(used);
    }
}

impl Shrinking for Op {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        match Rc::unwrap_or_clone(self.continuation) {
            Term::Mu(Mu {
                chi: Cns,
                variable,
                statement,
            }) => axcut::syntax::Statement::Op(axcut::syntax::Op {
                fst: self.fst,
                op: self.op.shrink(used_vars, types),
                snd: self.snd,
                var: variable,
                case: statement.shrink(used_vars, types),
            }),
            Term::XVar(XVar { chi: Cns, var }) => {
                let fresh_var = fresh_var(used_vars, "x");
                axcut::syntax::Statement::Op(axcut::syntax::Op {
                    fst: self.fst,
                    op: self.op.shrink(used_vars, types),
                    snd: self.snd,
                    var: fresh_var.clone(),
                    case: Rc::new(axcut::syntax::Statement::Invoke(axcut::syntax::Invoke {
                        var,
                        tag: cont_int().xtors[0].name.clone(),
                        ty: axcut::syntax::Ty::Decl(cont_int().name),
                        args: vec![fresh_var],
                    })),
                })
            }
            _ => panic!("cannot happen"),
        }
    }
}
