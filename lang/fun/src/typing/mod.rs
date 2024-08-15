use std::collections::HashMap;
use std::rc::Rc;

use crate::program::{Def, Prog};
use crate::syntax::{
    App, Case, Clause, Cocase, Constructor, Covariable, Ctor, Destructor, Dtor, Fun, Goto, Label,
    Lam, Name, Term, Variable,
};

use super::syntax::{IfZ, Let, Op};

pub mod solver;
pub mod types;

pub use solver::*;
pub use types::*;

//---------------------------------------------------------------
//---------------------- Zonking --------------------------------
//---------------------------------------------------------------
trait Zonk {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> Self;
}

impl Zonk for Ty {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> Ty {
        match self {
            Ty::Tyvar(v) => match varmap.get(v) {
                None => Ty::Tyvar(v.clone()),
                Some(ty) => ty.clone(),
            },
            Ty::Int() => Ty::Int(),
            Ty::List(ty) => {
                let ty_zonked: Ty = Zonk::zonk(ty, varmap);
                Ty::List(Rc::new(ty_zonked))
            }
            Ty::Stream(ty) => {
                let ty_zonked: Ty = Zonk::zonk(ty, varmap);
                Ty::Stream(Rc::new(ty_zonked))
            }
            Ty::Pair(ty1, ty2) => {
                let ty1_zonked: Ty = Zonk::zonk(ty1, varmap);
                let ty2_zonked: Ty = Zonk::zonk(ty2, varmap);
                Ty::Pair(Rc::new(ty1_zonked), Rc::new(ty2_zonked))
            }
            Ty::LPair(ty1, ty2) => {
                let ty1_zonked: Ty = Zonk::zonk(ty1, varmap);
                let ty2_zonked: Ty = Zonk::zonk(ty2, varmap);
                Ty::LPair(Rc::new(ty1_zonked), Rc::new(ty2_zonked))
            }
            Ty::Fun(ty1, ty2) => {
                let ty1_zonked: Ty = Zonk::zonk(ty1, varmap);
                let ty2_zonked: Ty = Zonk::zonk(ty2, varmap);
                Ty::Fun(Rc::new(ty1_zonked), Rc::new(ty2_zonked))
            }
        }
    }
}

impl Zonk for Def<Ty> {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> Def<Ty> {
        Def {
            name: self.name.clone(),
            args: self
                .args
                .iter()
                .map(|(v, ty)| (v.clone(), Zonk::zonk(ty, varmap)))
                .collect(),
            cont: self
                .cont
                .iter()
                .map(|(cv, ty)| (cv.clone(), Zonk::zonk(ty, varmap)))
                .collect(),
            body: self.body.clone(),
            ret_ty: Zonk::zonk(&self.ret_ty, varmap),
        }
    }
}

impl Zonk for Prog<Ty> {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> Prog<Ty> {
        Prog {
            prog_defs: self
                .prog_defs
                .iter()
                .map(|def| Zonk::zonk(def, varmap))
                .collect(),
        }
    }
}

impl Zonk for HashMap<Typevar, Ty> {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> HashMap<Typevar, Ty> {
        self.iter()
            .map(|(var, ty)| (var.clone(), Zonk::zonk(ty, varmap)))
            .collect::<HashMap<Typevar, Ty>>()
    }
}

//---------------------------------------------------------------
//--------------- Constraint Generation -------------------------
//---------------------------------------------------------------

pub type Error = String;

struct GenReader<'a> {
    gen_vars: HashMap<Variable, Ty>,
    gen_covars: HashMap<Covariable, Ty>,
    gen_prog: &'a Prog<Ty>,
}

impl<'a> GenReader<'a> {
    fn add_var_bindings(&self, new_bindings: Vec<(Variable, Ty)>) -> GenReader {
        let new_map: HashMap<Variable, Ty> = new_bindings.into_iter().collect();
        let new_gen_vars: HashMap<Variable, Ty> =
            self.gen_vars.clone().into_iter().chain(new_map).collect();
        GenReader {
            gen_vars: new_gen_vars,
            gen_covars: self.gen_covars.clone(),
            gen_prog: self.gen_prog,
        }
    }

    fn add_covar_bindings(&self, new_bindings: Vec<(Covariable, Ty)>) -> GenReader {
        let new_map: HashMap<Covariable, Ty> = new_bindings.into_iter().collect();
        let new_gen_covars: HashMap<Covariable, Ty> =
            self.gen_covars.clone().into_iter().chain(new_map).collect();
        GenReader {
            gen_vars: self.gen_vars.clone(),
            gen_covars: new_gen_covars,
            gen_prog: self.gen_prog,
        }
    }

    fn lookup_definition(&self, name: &Name) -> Result<(Vec<Ty>, Vec<Ty>, Ty), Error> {
        match &self.gen_prog.prog_defs.iter().find(|def| def.name == *name) {
            None => Err(format!(
                "A top-level function named {} is not contained in the program.",
                name
            )),
            Some(def) => {
                let arg_tys = def.args.iter().map(|(_, x)| x.clone()).collect();
                let cont_tys = def.cont.iter().map(|(_, x)| x.clone()).collect();
                Ok((arg_tys, cont_tys, def.ret_ty.clone()))
            }
        }
    }
}

struct GenState {
    varcnt: i64,
    constraints: Vec<Constraint>,
}

impl GenState {
    fn fresh_var(&mut self) -> Ty {
        let new_var: String = self.varcnt.to_string();
        self.varcnt += self.varcnt;
        Ty::Tyvar(new_var)
    }

    fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint)
    }
}

trait GenConstraint {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error>;
}

impl GenConstraint for Op {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        let ty1 = self.fst.gen_constraints(env, st)?;
        let ty2 = self.snd.gen_constraints(env, st)?;
        st.add_constraint((ty1, Ty::Int()));
        st.add_constraint((ty2, Ty::Int()));
        Ok(Ty::Int())
    }
}

impl GenConstraint for IfZ {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        let ty1 = self.ifc.gen_constraints(env, st)?;
        let ty2 = self.thenc.gen_constraints(env, st)?;
        let ty3 = self.elsec.gen_constraints(env, st)?;
        st.add_constraint((ty1, Ty::Int()));
        st.add_constraint((ty2.clone(), ty3));
        Ok(ty2)
    }
}

impl GenConstraint for Let {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        let ty = self.bound_term.gen_constraints(env, st)?;
        let new_reader: GenReader = env.add_var_bindings(vec![(self.variable.clone(), ty)]);
        self.in_term.gen_constraints(&new_reader, st)
    }
}

impl GenConstraint for Fun {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        let (arg_tys, coarg_tys, ret_ty) = env.lookup_definition(&self.name)?;
        if self.args.len() != arg_tys.len() || self.coargs.len() != coarg_tys.len() {
            Err(format!(
                "{} called with wrong number of arguments. Expected: {} + {} Got: {} + {}",
                self.name,
                arg_tys.len(),
                coarg_tys.len(),
                self.args.len(),
                self.coargs.len()
            ))
        } else {
            let arg_tys1: Vec<Ty> = self
                .args
                .iter()
                .map(|x| x.gen_constraints(env, st))
                .collect::<Result<Vec<Ty>, Error>>()?;
            let args_zipped = arg_tys1.into_iter().zip(arg_tys);
            for constraint in args_zipped {
                st.add_constraint(constraint);
            }
            let coargs_zipped = self.coargs.iter().cloned().zip(coarg_tys);
            for (coarg, coarg_ty) in coargs_zipped {
                match env.gen_covars.iter().find(|(cv, _)| **cv == coarg) {
                    None => Err(format!("Variable {} not found in environment", coarg)),
                    Some((_, ty)) => {
                        st.add_constraint((ty.clone(), coarg_ty));
                        Ok(())
                    }
                }?;
            }
            Ok(ret_ty.clone())
        }
    }
}

impl GenConstraint for Constructor {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        match &self.id {
            Ctor::Nil if self.args.is_empty() => Ok(Ty::List(Rc::new(st.fresh_var()))),
            Ctor::Cons => {
                let arg1: &Term = self
                    .args
                    .first()
                    .ok_or(format!("Wrong number of arguments for {}", Ctor::Cons))?;
                let arg2: &Term = self
                    .args
                    .get(1)
                    .ok_or(format!("Wrong number of arguments for {}", Ctor::Cons))?;
                if self.args.len() > 2 {
                    Err(format!("Wrong number of arguments for {}", Ctor::Cons))
                } else {
                    let ty1: Ty = arg1.gen_constraints(env, st)?;
                    let ty2: Ty = arg2.gen_constraints(env, st)?;
                    st.add_constraint((Ty::List(Rc::new(ty1)), ty2.clone()));
                    Ok(ty2)
                }
            }
            Ctor::Tup => {
                let arg1: &Term = self
                    .args
                    .first()
                    .ok_or(format!("Wrong number of arguments for {}", Ctor::Tup))?;
                let arg2: &Term = self
                    .args
                    .get(1)
                    .ok_or(format!("Wrong number of arguments for {}", Ctor::Tup))?;
                if self.args.len() > 2 {
                    Err(format!("Wrong number of arguments for {}", Ctor::Tup))
                } else {
                    let ty1: Ty = arg1.gen_constraints(env, st)?;
                    let ty2: Ty = arg2.gen_constraints(env, st)?;
                    Ok(Ty::Pair(Rc::new(ty1), Rc::new(ty2)))
                }
            }
            ctor => Err(format!(
                "Constructor {} applied to wrong number of arguments",
                ctor
            )),
        }
    }
}

impl GenConstraint for Destructor {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        match &self.id {
            Dtor::Hd if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_a: Ty = st.fresh_var();
                st.add_constraint((ty_bound, Ty::Stream(Rc::new(ty_a.clone()))));
                Ok(ty_a)
            }
            Dtor::Tl if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_str: Ty = Ty::Stream(Rc::new(st.fresh_var()));
                st.add_constraint((ty_bound, ty_str.clone()));
                Ok(ty_str)
            }
            Dtor::Fst if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_a: Ty = st.fresh_var();
                let ty_b: Ty = st.fresh_var();
                st.add_constraint((ty_bound, Ty::LPair(Rc::new(ty_a.clone()), Rc::new(ty_b))));
                Ok(ty_a)
            }
            Dtor::Snd if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_a: Ty = st.fresh_var();
                let ty_b: Ty = st.fresh_var();
                st.add_constraint((ty_bound, Ty::LPair(Rc::new(ty_a), Rc::new(ty_b.clone()))));
                Ok(ty_b)
            }
            dtor => Err(format!(
                "Destructor {} applied to wrong number of arguments",
                dtor
            )),
        }
    }
}

impl GenConstraint for Case {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        if self.cases.len() == 1 {
            let clause_tup: &Clause<Ctor> = self
                .cases
                .first()
                .ok_or(format!("Invalid case expression: {}", self))?;
            let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
            let ty_a: Ty = st.fresh_var();
            let ty_b: Ty = st.fresh_var();
            st.add_constraint((
                ty_bound,
                Ty::Pair(Rc::new(ty_a.clone()), Rc::new(ty_b.clone())),
            ));
            let var_first: &Variable = clause_tup.vars.first().ok_or(format!(
                "Wrong number of bound variables for {}",
                clause_tup.xtor
            ))?;
            let var_second: &Variable = clause_tup.vars.get(1).ok_or(format!(
                "Wrong number of bound variables for {}",
                clause_tup.xtor
            ))?;
            let new_env: GenReader =
                env.add_var_bindings(vec![(var_first.clone(), ty_a), (var_second.clone(), ty_b)]);
            clause_tup.rhs.gen_constraints(&new_env, st)
        } else if self.cases.len() == 2 {
            let clause_nil: &Clause<Ctor> = self
                .cases
                .iter()
                .find(|clauses| clauses.xtor == Ctor::Nil)
                .ok_or(format!("Invalid case expression: {}", self))?;
            let clause_cons: &Clause<Ctor> = self
                .cases
                .iter()
                .find(|clauses| clauses.xtor == Ctor::Cons)
                .ok_or(format!("Invalid case expression: {}", self))?;
            let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
            let list_arg: Rc<Ty> = Rc::new(st.fresh_var());
            let list_ty: Ty = Ty::List(list_arg.clone());
            st.add_constraint((ty_bound, list_ty.clone()));
            let ty_nil: Ty = clause_nil.rhs.gen_constraints(env, st)?;
            let var_head: &Variable = clause_cons.vars.first().ok_or(format!(
                "Wrong number of bound variables for {}",
                clause_cons.xtor
            ))?;
            let var_tail: &Variable = clause_cons.vars.get(1).ok_or(format!(
                "Wrong number of bound variables for {}",
                clause_cons.xtor
            ))?;
            let new_env: GenReader = env.add_var_bindings(vec![
                (var_head.clone(), Rc::unwrap_or_clone(list_arg)),
                (var_tail.clone(), list_ty),
            ]);
            let ty_cons: Ty = clause_cons.rhs.gen_constraints(&new_env, st)?;
            st.add_constraint((ty_nil.clone(), ty_cons));
            Ok(ty_nil)
        } else {
            Err(format!("Invalid case expression: {}", self))
        }
    }
}

impl GenConstraint for Cocase {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        let err_str = format!("Invalid cocase expression {}", self);
        if self.cocases.len() == 2 {
            let clause1: &Clause<Dtor> = self.cocases.first().ok_or(err_str.clone())?;
            let _ = if clause1.vars.is_empty() {
                Ok("")
            } else {
                Err(err_str.clone())
            }?;
            let clause2: &Clause<Dtor> = self.cocases.get(1).ok_or(err_str.clone())?;
            let _ = if clause1.vars.is_empty() {
                Ok("")
            } else {
                Err(err_str.clone())
            }?;
            let ty1: Ty = clause1.rhs.gen_constraints(env, st)?;
            let ty2: Ty = clause2.rhs.gen_constraints(env, st)?;
            if clause1.xtor == Dtor::Hd && clause2.xtor == Dtor::Tl {
                let str_ty: Ty = Ty::Stream(Rc::new(ty1));
                st.add_constraint((str_ty.clone(), ty2));
                Ok(str_ty)
            } else if clause1.xtor == Dtor::Fst && clause2.xtor == Dtor::Snd {
                let pair_ty: Ty = Ty::LPair(Rc::new(ty1), Rc::new(ty2));
                Ok(pair_ty)
            } else {
                Err(err_str)
            }
        } else {
            Err(err_str)
        }
    }
}

impl GenConstraint for Lam {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        let ty_a: Ty = st.fresh_var();
        let new_env: GenReader = env.add_var_bindings(vec![(self.variable.clone(), ty_a.clone())]);
        let ty_body = self.body.gen_constraints(&new_env, st)?;
        Ok(Ty::Fun(Rc::new(ty_a), Rc::new(ty_body)))
    }
}

impl GenConstraint for App {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        let ty1: Ty = self.function.gen_constraints(env, st)?;
        let ty2: Ty = self.argument.gen_constraints(env, st)?;
        let ret_ty: Ty = st.fresh_var();
        st.add_constraint((ty1, Ty::Fun(Rc::new(ty2), Rc::new(ret_ty.clone()))));
        Ok(ret_ty)
    }
}

impl GenConstraint for Goto {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        let ty1: Ty = self.term.gen_constraints(env, st)?;
        let (_, ty2): (&String, &Ty) = env
            .gen_covars
            .iter()
            .find(|(cv1, _)| *self.target == **cv1)
            .ok_or(format!(
                "Covariable {} not bound in environment",
                self.target
            ))?;
        st.add_constraint((ty1, ty2.clone()));
        Ok(st.fresh_var())
    }
}

impl GenConstraint for Label {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        let ty_a: Ty = st.fresh_var();
        env.add_covar_bindings(vec![(self.label.clone(), ty_a.clone())]);
        let ty: Ty = self.term.gen_constraints(env, st)?;
        st.add_constraint((ty.clone(), ty_a));
        Ok(ty)
    }
}

impl GenConstraint for Term {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
        match self {
            Term::Var(v) => match env.gen_vars.get(v) {
                None => Err(format!("Variable {} not bound in environment", v)),
                Some(ty) => Ok(ty.clone()),
            },
            Term::Lit(_) => Ok(Ty::Int()),
            Term::Op(o) => o.gen_constraints(env, st),
            Term::IfZ(i) => i.gen_constraints(env, st),
            Term::Let(l) => l.gen_constraints(env, st),
            Term::Fun(f) => f.gen_constraints(env, st),
            Term::Constructor(c) => c.gen_constraints(env, st),
            Term::Case(c) => c.gen_constraints(env, st),
            Term::Destructor(d) => d.gen_constraints(env, st),
            Term::Cocase(c) => c.gen_constraints(env, st),
            Term::Lam(l) => l.gen_constraints(env, st),
            Term::App(a) => a.gen_constraints(env, st),
            Term::Goto(g) => g.gen_constraints(env, st),
            Term::Label(l) => l.gen_constraints(env, st),
            Term::Paren(t) => t.inner.gen_constraints(env, st),
        }
    }
}

fn gen_constraints_def(def: &Def<Ty>, env: &GenReader, st: &mut GenState) -> Result<(), Error> {
    let env_with_vars: GenReader = env.add_var_bindings(def.args.clone());
    let env_with_covars: GenReader = env_with_vars.add_covar_bindings(def.cont.clone());
    let ty: Ty = def.body.gen_constraints(&env_with_covars, st)?;
    st.add_constraint((ty, def.ret_ty.clone()));
    Ok(())
}

fn annotate_program(prog: Prog<()>) -> Prog<Ty> {
    let mut var_cnt = 0;
    let mut fresh = || {
        var_cnt += 1;
        Ty::Tyvar(format!("b{}", var_cnt))
    };
    Prog {
        prog_defs: prog
            .prog_defs
            .into_iter()
            .map(|def| Def {
                name: def.name,
                args: def.args.into_iter().map(|(v, _)| (v, fresh())).collect(),
                cont: def.cont.into_iter().map(|(cv, _)| (cv, fresh())).collect(),
                body: def.body,
                ret_ty: fresh(),
            })
            .collect(),
    }
}

fn generate_constraints(prog: Prog<()>) -> Result<(Prog<Ty>, Vec<Constraint>), Error> {
    let prog_annot: Prog<Ty> = annotate_program(prog);
    let initial_reader: GenReader = GenReader {
        gen_vars: HashMap::new(),
        gen_covars: HashMap::new(),
        gen_prog: &prog_annot,
    };
    let mut initial_state: GenState = GenState {
        varcnt: 0,
        constraints: vec![],
    };

    let _: Vec<()> = prog_annot
        .prog_defs
        .iter()
        .map(|def| gen_constraints_def(def, &initial_reader, &mut initial_state))
        .collect::<Result<Vec<()>, Error>>()?;
    Ok((prog_annot, initial_state.constraints))
}

//---------------------------------------------------------------
//---------------- Type Inference -------------------------------
//---------------------------------------------------------------

pub fn infer_types(prog: Prog<()>) -> Result<Prog<Ty>, Error> {
    let (prog_typed, constraints): (Prog<Ty>, Vec<Constraint>) = generate_constraints(prog)?;
    let subst: HashMap<Typevar, Ty> = solve_constraints(constraints)?;
    let prog_zonked: Prog<Ty> = Zonk::zonk(&prog_typed, &subst);
    Ok(prog_zonked)
}
