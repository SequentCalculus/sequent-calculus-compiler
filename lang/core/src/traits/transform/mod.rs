pub mod clause;
pub mod cut;
pub mod fun;
pub mod ifz;
pub mod lit;
pub mod mu;
pub mod op;
pub mod subst;
pub mod term;
pub mod xcase;
pub mod xtor;

use crate::{
    syntax::{
        context::{context_covars, context_vars, ContextBinding, TypingContext},
        program::Declaration,
        substitution::SubstitutionBinding,
        term::{Cns, Prd, XVar},
        Covar, Def, Name, Prog, Statement, Var,
    },
    traits::free_vars::{fresh_covar, fresh_var},
};
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

#[derive(Default)]
pub struct TransformState {
    pub used_vars: HashSet<Var>,
    pub used_covars: HashSet<Covar>,
}

impl TransformState {
    pub fn fresh_var(&mut self) -> Var {
        let new_var = fresh_var(&self.used_vars);
        self.used_vars.insert(new_var.clone());
        new_var
    }

    pub fn fresh_covar(&mut self) -> Covar {
        let new_covar = fresh_covar(&self.used_covars);
        self.used_covars.insert(new_covar.clone());
        new_covar
    }

    pub fn add_context(&mut self, ctx: &TypingContext) {
        for bnd in ctx.iter() {
            match bnd {
                ContextBinding::VarBinding { var, ty: _ } => {
                    self.used_vars.insert(var.clone());
                }
                ContextBinding::CovarBinding { covar, ty: _ } => {
                    self.used_covars.insert(covar.clone());
                }
            }
        }
    }
}

pub trait NamingTransformation {
    type Target;
    fn transform(self, state: &mut TransformState) -> Self::Target;
}

impl<T: NamingTransformation + Clone> NamingTransformation for Rc<T> {
    type Target = Rc<T::Target>;
    fn transform(self, state: &mut TransformState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).transform(state))
    }
}

impl<T: NamingTransformation> NamingTransformation for Vec<T> {
    type Target = Vec<T::Target>;
    fn transform(self, state: &mut TransformState) -> Self::Target {
        self.into_iter().map(|x| x.transform(state)).collect()
    }
}

pub type Continuation = Box<dyn FnOnce(Name, &mut TransformState) -> Statement>;
pub type ContinuationVec =
    Box<dyn FnOnce(VecDeque<SubstitutionBinding>, &mut TransformState) -> Statement>;

pub trait Bind: Sized {
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement;
}

pub fn bind_many(
    mut args: VecDeque<SubstitutionBinding>,
    k: ContinuationVec,
    state: &mut TransformState,
) -> Statement {
    match args.pop_front() {
        None => k(VecDeque::new(), state),
        Some(SubstitutionBinding::ProducerBinding(p)) => p.bind(
            Box::new(|name, state| {
                bind_many(
                    args,
                    Box::new(|mut names, state| {
                        names.push_front(SubstitutionBinding::ProducerBinding(
                            XVar {
                                prdcns: Prd,
                                var: name,
                            }
                            .into(),
                        ));
                        k(names, state)
                    }),
                    state,
                )
            }),
            state,
        ),
        Some(SubstitutionBinding::ConsumerBinding(c)) => c.bind(
            Box::new(|name, state| {
                bind_many(
                    args,
                    Box::new(|mut names, state| {
                        names.push_front(SubstitutionBinding::ConsumerBinding(
                            XVar {
                                prdcns: Cns,
                                var: name,
                            }
                            .into(),
                        ));
                        k(names, state)
                    }),
                    state,
                )
            }),
            state,
        ),
    }
}

pub fn transform_def(def: Def) -> Def {
    let mut initial_state = TransformState {
        used_vars: context_vars(&def.context),
        used_covars: context_covars(&def.context),
    };

    Def {
        name: def.name,
        context: def.context,
        body: def.body.transform(&mut initial_state),
    }
}

pub fn transform_decl(decl: Declaration) -> Declaration {
    match decl {
        Declaration::Definition(def) => transform_def(def).into(),
        _ => decl,
    }
}

pub fn transform_prog(prog: Prog) -> Prog {
    Prog {
        prog_decls: prog.prog_decls.into_iter().map(transform_decl).collect(),
    }
}

impl NamingTransformation for Statement {
    type Target = Statement;
    fn transform(self: Statement, state: &mut TransformState) -> Statement {
        match self {
            Statement::Cut(cut) => cut.transform(state),
            Statement::Op(op) => op.transform(state),
            Statement::IfZ(ifz) => ifz.transform(state),
            Statement::Fun(fun) => fun.transform(state),
            Statement::Done() => Statement::Done(),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::{transform_def, transform_prog, NamingTransformation};
    use crate::syntax::{
        context::ContextBinding,
        program::Declaration,
        statement::{Cut, Fun, IfZ, Op},
        substitution::SubstitutionBinding,
        term::{Cns, Literal, Prd, XVar},
        types::Ty,
        BinOp, Def, Prog, Statement,
    };
    use std::rc::Rc;

    fn example_cut() -> Cut {
        Cut {
            producer: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            consumer: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                }
                .into(),
            ),
        }
    }
    fn example_op() -> Op {
        Op {
            fst: Rc::new(Literal { lit: 1 }.into()),
            op: BinOp::Prod,
            snd: Rc::new(Literal { lit: 2 }.into()),
            continuation: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                }
                .into(),
            ),
        }
    }

    fn example_ifz() -> IfZ {
        IfZ {
            ifc: Rc::new(Literal { lit: 0 }.into()),
            thenc: Rc::new(Statement::Done()),
            elsec: Rc::new(Statement::Done()),
        }
    }

    fn example_fun() -> Fun {
        Fun {
            name: "multFast".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    XVar {
                        prdcns: Cns,
                        var: "a".to_owned(),
                    }
                    .into(),
                ),
            ],
        }
    }

    fn example_done() -> Statement {
        Statement::Done()
    }

    fn example_def1() -> Def {
        Def {
            name: "done".to_owned(),
            context: vec![],
            body: Statement::Done(),
        }
    }
    fn example_def2() -> Def {
        Def {
            name: "cut".to_owned(),
            context: vec![
                ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: Ty::Int(),
                },
            ],
            body: Cut {
                producer: Rc::new(
                    XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                consumer: Rc::new(
                    XVar {
                        prdcns: Cns,
                        var: "a".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        }
    }

    fn example_prog1() -> Prog {
        Prog { prog_decls: vec![] }
    }
    fn example_prog2() -> Prog {
        Prog {
            prog_decls: vec![example_def1().into()],
        }
    }

    #[test]
    fn transform_cut() {
        let result =
            <Cut as Into<Statement>>::into(example_cut()).transform(&mut Default::default());
        let expected = example_cut().transform(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_op() {
        let result = <Op as Into<Statement>>::into(example_op()).transform(&mut Default::default());
        let expected = example_op().transform(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_ifz() {
        let result =
            <IfZ as Into<Statement>>::into(example_ifz()).transform(&mut Default::default());
        let expected = example_ifz().transform(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_fun() {
        let result =
            <Fun as Into<Statement>>::into(example_fun()).transform(&mut Default::default());
        let expected = example_fun().transform(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_done() {
        let result = example_done().transform(&mut Default::default());
        let expected = Statement::Done();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_def1() {
        let result = transform_def(example_def1());
        let expected = example_def1();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.context, expected.context);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_def2() {
        let result = transform_def(example_def2());
        let expected = example_def2();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.context, expected.context);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_prog1() {
        let result = transform_prog(example_prog1());
        assert!(result.prog_decls.is_empty())
    }

    #[test]
    fn transform_prog2() {
        let result = transform_prog(example_prog2());
        assert_eq!(result.prog_decls.len(), 1);
        let def1 = result.prog_decls.get(0);
        assert!(def1.is_some());
        let def1un = def1.unwrap();
        let def = if let Declaration::Definition(def) = def1un {
            Some(def)
        } else {
            None
        }
        .unwrap();
        let ex = example_def1();
        assert_eq!(def.name, ex.name);
        assert_eq!(def.context, ex.context);
        assert_eq!(def.body, ex.body);
    }
}
