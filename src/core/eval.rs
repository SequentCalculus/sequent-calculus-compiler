use crate::core::substitution::Subst;
use crate::core::syntax::{Consumer, Def, Pattern, Producer, Prog, Statement};
use crate::fun::syntax::{BinOp, Covariable, Ctor, Dtor, Variable};
use std::rc::Rc;

use super::syntax::{Cocase, Constructor, Literal, Mu};

fn eval<T>(st: Statement, p: &Prog<T>, tr: &mut Vec<Statement>) -> Vec<Statement> {
    let st_eval: Option<Statement> = eval_once(st, p);
    match st_eval {
        None => tr.clone(),
        Some(st_new) => {
            tr.push(st_new.clone());
            eval(st_new, p, tr)
        }
    }
}

macro_rules! eval {
    ($st: expr,$p:expr,$tr:expr) => {
        eval($st, $p, $tr)
    };
    ($st:expr,$p:expr) => {
        eval($st, $p, &mut vec![])
    };
}
fn eval_once<T>(st: Statement, p: &Prog<T>) -> Option<Statement> {
    match st {
        Statement::Cut(p, c) => match (Rc::unwrap_or_clone(p), Rc::unwrap_or_clone(c)) {
            (
                Producer::Mu(Mu {
                    covariable,
                    statement,
                }),
                cons,
            ) => {
                let st_subst: Rc<Statement> = Subst::subst_covar(&statement, cons, covariable);
                Some(Rc::unwrap_or_clone(st_subst))
            }
            (prod, Consumer::MuTilde(v, mu_st)) => {
                let st_subst: Rc<Statement> = Subst::subst_var(&mu_st, prod, v);
                Some(Rc::unwrap_or_clone(st_subst))
            }
            (
                Producer::Constructor(Constructor {
                    id,
                    producers,
                    consumers,
                }),
                Consumer::Case(pts),
            ) => {
                let ct_pt: &Pattern<Ctor> = pts.iter().find(|pt| pt.xtor == id)?;
                let prod_subst: Vec<(Producer, Variable)> = producers
                    .iter()
                    .cloned()
                    .map(Rc::unwrap_or_clone)
                    .zip(ct_pt.vars.clone())
                    .collect();
                let cons_subst: Vec<(Consumer, Covariable)> = consumers
                    .iter()
                    .cloned()
                    .map(Rc::unwrap_or_clone)
                    .zip(ct_pt.covars.clone())
                    .collect();
                let new_st: Rc<Statement> =
                    Subst::subst_sim(Rc::as_ref(&ct_pt.rhs), &prod_subst, &cons_subst);
                Some(Rc::unwrap_or_clone(new_st))
            }
            (Producer::Cocase(Cocase { cocases }), Consumer::Destructor(dtor, pargs, cargs)) => {
                let dt_pt: &Pattern<Dtor> = cocases.iter().find(|pt| pt.xtor == dtor)?;
                let prod_subst: Vec<(Producer, Variable)> = pargs
                    .iter()
                    .cloned()
                    .map(Rc::unwrap_or_clone)
                    .zip(dt_pt.vars.clone())
                    .collect();
                let cons_subst: Vec<(Consumer, Covariable)> = cargs
                    .iter()
                    .cloned()
                    .map(Rc::unwrap_or_clone)
                    .zip(dt_pt.covars.clone())
                    .collect();
                let new_st: Rc<Statement> = Subst::subst_sim(&dt_pt.rhs, &prod_subst, &cons_subst);
                Some(Rc::unwrap_or_clone(new_st))
            }
            (_, _) => None,
        },
        Statement::Op(p1, op, p2, c) => match (Rc::unwrap_or_clone(p1), Rc::unwrap_or_clone(p2)) {
            (Producer::Literal(Literal { lit: n }), Producer::Literal(Literal { lit: m })) => {
                let new_int: i64 = match op {
                    BinOp::Prod => n * m,
                    BinOp::Sum => n + m,
                    BinOp::Sub => n - m,
                };
                let new_lit: Rc<Producer> = Rc::new(Literal { lit: new_int }.into());
                Some(Statement::Cut(new_lit, c))
            }
            (_, _) => None,
        },
        Statement::IfZ(p, st1, st2) => match Rc::unwrap_or_clone(p) {
            Producer::Literal(Literal { lit: 0 }) => Some(Rc::unwrap_or_clone(st1)),
            Producer::Literal(Literal { lit: n }) if n != 0 => Some(Rc::unwrap_or_clone(st2)),
            _ => None,
        },
        Statement::Fun(nm, pargs, cargs) => {
            let nm_def: &Def<T> = p.prog_defs.iter().find(|df| df.name == nm)?;
            let prod_vars: Vec<Variable> =
                nm_def.pargs.iter().map(|(var, _)| var.clone()).collect();
            let prod_subst: Vec<(Producer, Variable)> = pargs
                .iter()
                .cloned()
                .map(Rc::unwrap_or_clone)
                .zip(prod_vars)
                .collect();
            let cons_covars: Vec<Covariable> = nm_def
                .cargs
                .iter()
                .map(|(covar, _)| covar.clone())
                .collect();
            let cons_subst: Vec<(Consumer, Covariable)> = cargs
                .iter()
                .cloned()
                .map(Rc::unwrap_or_clone)
                .zip(cons_covars)
                .collect();
            let new_st: Rc<Statement> = Subst::subst_sim(&nm_def.body, &prod_subst, &cons_subst);
            Some(Rc::unwrap_or_clone(new_st))
        }
        Statement::Done() => Some(Statement::Done()),
    }
}

pub fn eval_main<T>(prog: Prog<T>) -> Option<Vec<Statement>> {
    let main_def: &Def<T> = prog.prog_defs.iter().find(|df| df.name == "main")?;
    let main_cont: &(String, T) = main_def.cargs.first()?;
    let main_body: Rc<Statement> = Subst::subst_covar(
        &main_def.body,
        Consumer::Covar(String::from("*")),
        main_cont.0.clone(),
    );
    Some(eval!(Rc::unwrap_or_clone(main_body), &prog))
}
