use std::collections::HashMap;

use crate::program::{Def, Prog};
use crate::syntax::{
    App, Case, Clause, Cocase, Constructor, Covariable, Ctor, Destructor, Dtor, Fun, Goto, Label,
    Lam, Name, Term, Variable,
};

use super::syntax::{IfZ, Let, Op};

pub mod result;
pub mod solver;
pub mod types;

use result::TypeError;
pub use solver::*;
pub use types::*;

//---------------------------------------------------------------
//---------------------- Zonking --------------------------------
//---------------------------------------------------------------
trait Zonk {
    fn zonk(&mut self, varmap: &HashMap<Typevar, Ty>);
}

impl Zonk for Ty {
    fn zonk(&mut self, varmap: &HashMap<Typevar, Ty>) {
        match self {
            Ty::Var(v) => match varmap.get(v) {
                None => {}
                Some(ty) => *self = ty.clone(),
            },
            Ty::Int() => {}
            Ty::List(ty) => ty.zonk(varmap),
            Ty::Stream(ty) => ty.zonk(varmap),
            Ty::Pair(ty1, ty2) => {
                ty1.zonk(varmap);
                ty2.zonk(varmap)
            }
            Ty::LPair(ty1, ty2) => {
                ty1.zonk(varmap);
                ty2.zonk(varmap)
            }
            Ty::Fun(ty1, ty2) => {
                ty1.zonk(varmap);
                ty2.zonk(varmap)
            }
        }
    }
}

impl Zonk for Def<Ty> {
    fn zonk(&mut self, varmap: &HashMap<Typevar, Ty>) {
        for e in self.args.iter_mut() {
            e.1.zonk(varmap)
        }
        for e in self.cont.iter_mut() {
            e.1.zonk(varmap)
        }
        self.ret_ty.zonk(varmap)
    }
}

impl Zonk for Prog<Ty> {
    fn zonk(&mut self, varmap: &HashMap<Typevar, Ty>) {
        for def in self.prog_defs.iter_mut() {
            def.zonk(varmap)
        }
    }
}

impl Zonk for HashMap<Typevar, Ty> {
    fn zonk(&mut self, varmap: &HashMap<Typevar, Ty>) {
        for ty in self.values_mut() {
            ty.zonk(varmap)
        }
    }
}

#[cfg(test)]
mod zonk_tests {

    use super::{Def, Term, Ty, Zonk};
    use std::collections::HashMap;

    #[test]
    fn zonk_int() {
        let mut ty = Ty::Int();
        ty.zonk(&HashMap::new());
        assert_eq!(ty, Ty::Int())
    }

    #[test]
    fn zonk_var() {
        let mut ty = Ty::Var("X".to_owned());
        let mut varmap = HashMap::new();
        varmap.insert("X".to_owned(), Ty::Int());
        ty.zonk(&varmap);
        assert_eq!(ty, Ty::Int())
    }

    #[test]
    fn zonk_var2() {
        let mut ty = Ty::Var("X".to_owned());
        let mut varmap = HashMap::new();
        varmap.insert("Y".to_owned(), Ty::Int());
        ty.zonk(&varmap);
        assert_eq!(ty, Ty::Var("X".to_owned()))
    }

    #[test]
    fn zonk_list() {
        let mut ty = Ty::List(Box::new(Ty::Var("X".to_owned())));
        let mut varmap = HashMap::new();
        varmap.insert("X".to_owned(), Ty::Int());
        ty.zonk(&varmap);
        assert_eq!(ty, Ty::List(Box::new(Ty::Int())));
    }

    #[test]
    fn zonk_pair() {
        let mut ty = Ty::Pair(Box::new(Ty::Int()), Box::new(Ty::Var("X".to_owned())));
        let mut varmap = HashMap::new();
        varmap.insert("Y".to_owned(), Ty::Int());
        ty.zonk(&varmap);
        assert_eq!(
            ty,
            Ty::Pair(Box::new(Ty::Int()), Box::new(Ty::Var("X".to_owned())))
        )
    }

    #[test]
    fn zonk_stream() {
        let mut ty = Ty::Stream(Box::new(Ty::Int()));
        let mut varmap = HashMap::new();
        varmap.insert("X".to_owned(), Ty::Int());
        ty.zonk(&varmap);
        assert_eq!(ty, Ty::Stream(Box::new(Ty::Int())));
    }

    #[test]
    fn zonk_lpair() {
        let mut ty = Ty::LPair(Box::new(Ty::Int()), Box::new(Ty::Var("X".to_owned())));
        let mut varmap = HashMap::new();
        varmap.insert("X".to_owned(), Ty::Var("Y".to_owned()));
        ty.zonk(&varmap);
        assert_eq!(
            ty,
            Ty::LPair(Box::new(Ty::Int()), Box::new(Ty::Var("Y".to_owned())))
        );
    }

    #[test]
    fn zonk_fun() {
        let mut ty = Ty::Fun(Box::new(Ty::Int()), Box::new(Ty::Int()));
        let varmap = HashMap::new();
        ty.zonk(&varmap);
        assert_eq!(ty, Ty::Fun(Box::new(Ty::Int()), Box::new(Ty::Int())));
    }

    #[test]
    fn zonk_def() {
        let mut def = Def {
            name: "main".to_owned(),
            args: vec![("x".to_owned(), Ty::Var("X".to_owned()))],
            cont: vec![("a".to_owned(), Ty::Var("Y".to_owned()))],
            body: Term::Var("x".to_owned()),
            ret_ty: Ty::Var("X".to_owned()),
        };
        let mut varmap = HashMap::new();
        varmap.insert("X".to_owned(), Ty::Int());
        varmap.insert("Y".to_owned(), Ty::Int());
        def.zonk(&varmap);
        assert_eq!(def.name, "main".to_owned());
        assert_eq!(def.args, vec![("x".to_owned(), Ty::Int())]);
        assert_eq!(def.cont, vec![("a".to_owned(), Ty::Int())]);
        assert_eq!(def.body, Term::Var("x".to_owned()));
        assert_eq!(def.ret_ty, Ty::Int())
    }
}

//---------------------------------------------------------------
//--------------- Constraint Generation -------------------------
//---------------------------------------------------------------

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

    fn lookup_definition(&self, name: &Name) -> Result<(Vec<Ty>, Vec<Ty>, Ty), TypeError> {
        match &self.gen_prog.prog_defs.iter().find(|def| def.name == *name) {
            None => Err(TypeError::FunNotFound { name: name.clone() }),
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
        self.varcnt += 1;
        Ty::Var(new_var)
    }

    fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint)
    }
}

trait GenConstraint {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError>;
}

impl GenConstraint for Op {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty1 = self.fst.gen_constraints(env, st)?;
        let ty2 = self.snd.gen_constraints(env, st)?;
        st.add_constraint((ty1, Ty::Int()));
        st.add_constraint((ty2, Ty::Int()));
        Ok(Ty::Int())
    }
}

impl GenConstraint for IfZ {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty1 = self.ifc.gen_constraints(env, st)?;
        let ty2 = self.thenc.gen_constraints(env, st)?;
        let ty3 = self.elsec.gen_constraints(env, st)?;
        st.add_constraint((ty1, Ty::Int()));
        st.add_constraint((ty2.clone(), ty3));
        Ok(ty2)
    }
}

impl GenConstraint for Let {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty = self.bound_term.gen_constraints(env, st)?;
        let new_reader: GenReader = env.add_var_bindings(vec![(self.variable.clone(), ty)]);
        self.in_term.gen_constraints(&new_reader, st)
    }
}

impl GenConstraint for Fun {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let (arg_tys, coarg_tys, ret_ty) = env.lookup_definition(&self.name)?;
        if self.args.len() != arg_tys.len() || self.coargs.len() != coarg_tys.len() {
            Err(TypeError::FunWrongNumOfArgs {
                name: self.name.clone(),
                expected_vars: arg_tys.len(),
                actual_vars: self.args.len(),
                expected_covars: coarg_tys.len(),
                actual_covars: self.coargs.len(),
            })
        } else {
            let arg_tys1: Vec<Ty> = self
                .args
                .iter()
                .map(|x| x.gen_constraints(env, st))
                .collect::<Result<Vec<Ty>, TypeError>>()?;
            let args_zipped = arg_tys1.into_iter().zip(arg_tys);
            for constraint in args_zipped {
                st.add_constraint(constraint);
            }
            let coargs_zipped = self.coargs.iter().cloned().zip(coarg_tys);
            for (coarg, coarg_ty) in coargs_zipped {
                match env.gen_covars.iter().find(|(cv, _)| **cv == coarg) {
                    None => Err(TypeError::VarNotFound { name: coarg }),
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
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        match &self.id {
            Ctor::Nil if self.args.is_empty() => Ok(Ty::List(Box::new(st.fresh_var()))),
            Ctor::Cons => {
                let arg1: &Term = self
                    .args
                    .first()
                    .ok_or(TypeError::CtorWrongNumOfArgs { ctor: Ctor::Cons })?;
                let arg2: &Term = self
                    .args
                    .get(1)
                    .ok_or(TypeError::CtorWrongNumOfArgs { ctor: Ctor::Cons })?;
                if self.args.len() > 2 {
                    Err(TypeError::CtorWrongNumOfArgs { ctor: Ctor::Cons })
                } else {
                    let ty1: Ty = arg1.gen_constraints(env, st)?;
                    let ty2: Ty = arg2.gen_constraints(env, st)?;
                    st.add_constraint((Ty::List(Box::new(ty1)), ty2.clone()));
                    Ok(ty2)
                }
            }
            Ctor::Tup => {
                let arg1: &Term = self
                    .args
                    .first()
                    .ok_or(TypeError::CtorWrongNumOfArgs { ctor: Ctor::Tup })?;
                let arg2: &Term = self
                    .args
                    .get(1)
                    .ok_or(TypeError::CtorWrongNumOfArgs { ctor: Ctor::Tup })?;
                if self.args.len() > 2 {
                    Err(TypeError::CtorWrongNumOfArgs { ctor: Ctor::Tup })
                } else {
                    let ty1: Ty = arg1.gen_constraints(env, st)?;
                    let ty2: Ty = arg2.gen_constraints(env, st)?;
                    Ok(Ty::Pair(Box::new(ty1), Box::new(ty2)))
                }
            }
            ctor => Err(TypeError::CtorWrongNumOfArgs { ctor: ctor.clone() }),
        }
    }
}

impl GenConstraint for Destructor {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        match &self.id {
            Dtor::Hd if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_a: Ty = st.fresh_var();
                st.add_constraint((ty_bound, Ty::Stream(Box::new(ty_a.clone()))));
                Ok(ty_a)
            }
            Dtor::Tl if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_str: Ty = Ty::Stream(Box::new(st.fresh_var()));
                st.add_constraint((ty_bound, ty_str.clone()));
                Ok(ty_str)
            }
            Dtor::Fst if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_a: Ty = st.fresh_var();
                let ty_b: Ty = st.fresh_var();
                st.add_constraint((ty_bound, Ty::LPair(Box::new(ty_a.clone()), Box::new(ty_b))));
                Ok(ty_a)
            }
            Dtor::Snd if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_a: Ty = st.fresh_var();
                let ty_b: Ty = st.fresh_var();
                st.add_constraint((ty_bound, Ty::LPair(Box::new(ty_a), Box::new(ty_b.clone()))));
                Ok(ty_b)
            }
            dtor => Err(TypeError::DtorWrongNumOfArgs { dtor: dtor.clone() }),
        }
    }
}

impl GenConstraint for Case {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        if self.cases.len() == 1 {
            let clause_tup: &Clause<Ctor> = self.cases.first().ok_or(TypeError::InvalidCase)?;
            let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
            let ty_a: Ty = st.fresh_var();
            let ty_b: Ty = st.fresh_var();
            st.add_constraint((
                ty_bound,
                Ty::Pair(Box::new(ty_a.clone()), Box::new(ty_b.clone())),
            ));
            let var_first: &Variable =
                clause_tup
                    .vars
                    .first()
                    .ok_or(TypeError::PatternWrongNumOfArgs {
                        ctor: clause_tup.xtor.clone(),
                    })?;
            let var_second: &Variable =
                clause_tup
                    .vars
                    .get(1)
                    .ok_or(TypeError::PatternWrongNumOfArgs {
                        ctor: clause_tup.xtor.clone(),
                    })?;
            let new_env: GenReader =
                env.add_var_bindings(vec![(var_first.clone(), ty_a), (var_second.clone(), ty_b)]);
            clause_tup.rhs.gen_constraints(&new_env, st)
        } else if self.cases.len() == 2 {
            let clause_nil: &Clause<Ctor> = self
                .cases
                .iter()
                .find(|clauses| clauses.xtor == Ctor::Nil)
                .ok_or(TypeError::InvalidCase)?;
            let clause_cons: &Clause<Ctor> = self
                .cases
                .iter()
                .find(|clauses| clauses.xtor == Ctor::Cons)
                .ok_or(TypeError::InvalidCase)?;
            let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
            let list_arg: Box<Ty> = Box::new(st.fresh_var());
            let list_ty: Ty = Ty::List(list_arg.clone());
            st.add_constraint((ty_bound, list_ty.clone()));
            let ty_nil: Ty = clause_nil.rhs.gen_constraints(env, st)?;
            let var_head: &Variable =
                clause_cons
                    .vars
                    .first()
                    .ok_or(TypeError::PatternWrongNumOfArgs {
                        ctor: clause_cons.xtor.clone(),
                    })?;
            let var_tail: &Variable =
                clause_cons
                    .vars
                    .get(1)
                    .ok_or(TypeError::PatternWrongNumOfArgs {
                        ctor: clause_cons.xtor.clone(),
                    })?;
            let new_env: GenReader = env.add_var_bindings(vec![
                (var_head.clone(), *list_arg),
                (var_tail.clone(), list_ty),
            ]);
            let ty_cons: Ty = clause_cons.rhs.gen_constraints(&new_env, st)?;
            st.add_constraint((ty_nil.clone(), ty_cons));
            Ok(ty_nil)
        } else {
            Err(TypeError::InvalidCase)
        }
    }
}

impl GenConstraint for Cocase {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        if self.cocases.len() == 2 {
            let clause1: &Clause<Dtor> = self.cocases.first().ok_or(TypeError::InvalidCocase)?;
            let _ = if clause1.vars.is_empty() {
                Ok("")
            } else {
                Err(TypeError::InvalidCocase)
            }?;
            let clause2: &Clause<Dtor> = self.cocases.get(1).ok_or(TypeError::InvalidCocase)?;
            let _ = if clause1.vars.is_empty() {
                Ok("")
            } else {
                Err(TypeError::InvalidCocase)
            }?;
            let ty1: Ty = clause1.rhs.gen_constraints(env, st)?;
            let ty2: Ty = clause2.rhs.gen_constraints(env, st)?;
            if clause1.xtor == Dtor::Hd && clause2.xtor == Dtor::Tl {
                let str_ty: Ty = Ty::Stream(Box::new(ty1));
                st.add_constraint((str_ty.clone(), ty2));
                Ok(str_ty)
            } else if clause1.xtor == Dtor::Fst && clause2.xtor == Dtor::Snd {
                let pair_ty: Ty = Ty::LPair(Box::new(ty1), Box::new(ty2));
                Ok(pair_ty)
            } else {
                Err(TypeError::InvalidCocase)
            }
        } else {
            Err(TypeError::InvalidCocase)
        }
    }
}

impl GenConstraint for Lam {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty_a: Ty = st.fresh_var();
        let new_env: GenReader = env.add_var_bindings(vec![(self.variable.clone(), ty_a.clone())]);
        let ty_body = self.body.gen_constraints(&new_env, st)?;
        Ok(Ty::Fun(Box::new(ty_a), Box::new(ty_body)))
    }
}

impl GenConstraint for App {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty1: Ty = self.function.gen_constraints(env, st)?;
        let ty2: Ty = self.argument.gen_constraints(env, st)?;
        let ret_ty: Ty = st.fresh_var();
        st.add_constraint((ty1, Ty::Fun(Box::new(ty2), Box::new(ret_ty.clone()))));
        Ok(ret_ty)
    }
}

impl GenConstraint for Goto {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty1: Ty = self.term.gen_constraints(env, st)?;
        let (_, ty2): (&String, &Ty) = env
            .gen_covars
            .iter()
            .find(|(cv1, _)| *self.target == **cv1)
            .ok_or(TypeError::CovarNotFound {
                name: self.target.clone(),
            })?;
        st.add_constraint((ty1, ty2.clone()));
        Ok(st.fresh_var())
    }
}

impl GenConstraint for Label {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty_a: Ty = st.fresh_var();
        let new_env = env.add_covar_bindings(vec![(self.label.clone(), ty_a.clone())]);
        let ty: Ty = self.term.gen_constraints(&new_env, st)?;
        st.add_constraint((ty.clone(), ty_a));
        Ok(ty)
    }
}

impl GenConstraint for Term {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        match self {
            Term::Var(v) => match env.gen_vars.get(v) {
                None => Err(TypeError::VarNotFound { name: v.clone() }),
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

fn gen_constraints_def(def: &Def<Ty>, env: &GenReader, st: &mut GenState) -> Result<(), TypeError> {
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
        Ty::Var(format!("b{}", var_cnt))
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

fn generate_constraints(prog: Prog<()>) -> Result<(Prog<Ty>, Vec<Constraint>), TypeError> {
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
        .collect::<Result<Vec<()>, TypeError>>()?;
    Ok((prog_annot, initial_state.constraints))
}

//---------------------------------------------------------------
//---------------- Type Inference -------------------------------
//---------------------------------------------------------------

pub fn infer_types(prog: Prog<()>) -> Result<Prog<Ty>, TypeError> {
    let (mut prog, constraints): (Prog<Ty>, Vec<Constraint>) = generate_constraints(prog)?;
    let subst = solve_constraints(constraints)?;
    prog.zonk(&subst);
    Ok(prog)
}
