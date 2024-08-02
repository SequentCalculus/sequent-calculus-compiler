use core::traits::free_vars::{fresh_covar, FreeV};
use fun::syntax::Covariable;
use std::collections::HashSet;
use std::rc::Rc;

struct CompileState {
    covars: HashSet<Covariable>,
}

impl CompileState {
    fn add_covars<T: FreeV>(&mut self, new_cv: &T) {
        let fr_cv = FreeV::free_covars(new_cv);
        self.covars.extend(fr_cv);
    }

    fn free_covar_from_state(&mut self) -> Covariable {
        let new_cv: Covariable = fresh_covar(&self.covars);
        self.covars.insert(new_cv.clone());
        new_cv
    }
}

trait Compile {
    type Target;
    type TargetInner;
    type Continuation;

    fn compile(self, _: &mut CompileState) -> Self::Target;
    fn compile_inner(self, _: Self::Continuation, st: &mut CompileState) -> Self::TargetInner;
}

impl<T: Compile + Clone> Compile for Rc<T> {
    type Target = Rc<T::Target>;
    type TargetInner = Rc<T::TargetInner>;
    type Continuation = T::Continuation;

    fn compile(self, st: &mut CompileState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).compile(st))
    }

    fn compile_inner(self, cont: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        Rc::new(Rc::unwrap_or_clone(self).compile_inner(cont, st))
    }
}

impl Compile for fun::syntax::BinOp {
    type Target = core::syntax::BinOp;
    type TargetInner = core::syntax::BinOp;
    type Continuation = ();

    fn compile(self, _: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::BinOp::Prod => core::syntax::BinOp::Prod,
            fun::syntax::BinOp::Sum => core::syntax::BinOp::Sum,
            fun::syntax::BinOp::Sub => core::syntax::BinOp::Sub,
        }
    }

    fn compile_inner(self, _: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        Compile::compile(self, st)
    }
}

impl Compile for fun::syntax::Ctor {
    type Target = core::syntax::Ctor;
    type TargetInner = core::syntax::Ctor;
    type Continuation = ();

    fn compile(self, _: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::Ctor::Nil => core::syntax::Ctor::Nil,
            fun::syntax::Ctor::Cons => core::syntax::Ctor::Cons,
            fun::syntax::Ctor::Tup => core::syntax::Ctor::Tup,
        }
    }

    fn compile_inner(self, _: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        self.compile(st)
    }
}

impl Compile for fun::syntax::Term {
    type Target = core::syntax::Producer;
    type TargetInner = core::syntax::Statement;
    type Continuation = core::syntax::Consumer;

    fn compile(self, st: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::Term::Var(v) => core::syntax::Variable { var: v }.into(),
            fun::syntax::Term::Lit(n) => core::syntax::Literal { lit: n }.into(),
            fun::syntax::Term::Op(op) => op.compile(st).into(),
            fun::syntax::Term::IfZ(ifz) => ifz.compile(st).into(),
            fun::syntax::Term::Let(lt) => lt.compile(st).into(),
            fun::syntax::Term::Fun(fun) => fun.compile(st).into(),
            fun::syntax::Term::Constructor(cons) => cons.compile(st).into(),
            fun::syntax::Term::Destructor(dest) => panic!(""),
            fun::syntax::Term::Case(case) => panic!(""),
            fun::syntax::Term::Cocase(cocase) => panic!(""),
            fun::syntax::Term::Lam(lam) => panic!(""),
            fun::syntax::Term::App(ap) => panic!(""),
            fun::syntax::Term::Goto(goto) => panic!(""),
            fun::syntax::Term::Label(label) => panic!(""),
        }
    }

    fn compile_inner(self, cont: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        match self {
            fun::syntax::Term::Var(v) => {
                let new_var: core::syntax::Producer = core::syntax::Variable { var: v }.into();
                core::syntax::Cut {
                    producer: Rc::new(new_var),
                    consumer: Rc::new(cont),
                }
                .into()
            }
            fun::syntax::Term::Lit(n) => {
                let new_lit: core::syntax::Producer = core::syntax::Literal { lit: n }.into();
                core::syntax::Cut {
                    producer: Rc::new(new_lit),
                    consumer: Rc::new(cont),
                }
                .into()
            }
            fun::syntax::Term::Op(op) => op.compile_inner(cont, st).into(),
            fun::syntax::Term::IfZ(ifz) => ifz.compile_inner(cont, st).into(),
            fun::syntax::Term::Let(lt) => lt.compile_inner(cont, st),
            fun::syntax::Term::Fun(fun) => fun.compile_inner(cont, st).into(),
            fun::syntax::Term::Constructor(cons) => cons.compile_inner(cont, st),
            fun::syntax::Term::Destructor(dest) => panic!(""),
            fun::syntax::Term::Case(case) => panic!(""),
            fun::syntax::Term::Cocase(cocase) => panic!(""),
            fun::syntax::Term::Lam(lam) => panic!(""),
            fun::syntax::Term::App(ap) => panic!(""),
            fun::syntax::Term::Goto(goto) => panic!(""),
            fun::syntax::Term::Label(label) => panic!(""),
        }
    }
}

impl Compile for fun::syntax::Op {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Op;
    type Continuation = core::syntax::Consumer;

    fn compile(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_inner(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st.into()),
        }
    }

    fn compile_inner(self, cont: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        core::syntax::Op {
            fst: self.fst.compile(st),
            op: self.op.compile(st),
            snd: self.snd.compile(st),
            continuation: Rc::new(cont),
        }
    }
}

impl Compile for fun::syntax::IfZ {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::IfZ;
    type Continuation = core::syntax::Consumer;

    fn compile(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_cont = core::syntax::Consumer::Covar(new_cv.clone());
        let new_st = self.compile_inner(new_cont, st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st.into()),
        }
    }

    fn compile_inner(self, cont: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        core::syntax::IfZ {
            ifc: self.ifc.compile(st),
            thenc: self.thenc.compile_inner(cont.clone(), st),
            elsec: self.elsec.compile_inner(cont, st),
        }
    }
}

impl Compile for fun::syntax::Let {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Statement;
    type Continuation = core::syntax::Consumer;

    fn compile(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_inner(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
    }

    fn compile_inner(self, cont: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        let new_st = self.bound_term.compile_inner(cont, st);
        let new_cont = core::syntax::MuTilde {
            variable: self.variable,
            statement: new_st,
        };
        Rc::unwrap_or_clone(self.in_term.compile_inner(new_cont.into(), st))
    }
}

impl Compile for fun::syntax::Fun {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Fun;
    type Continuation = core::syntax::Consumer;

    fn compile(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_inner(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st.into()),
        }
    }

    fn compile_inner(self, cont: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        let mut new_coargs: Vec<core::syntax::Consumer> = self
            .coargs
            .iter()
            .cloned()
            .map(core::syntax::Consumer::Covar)
            .collect();
        new_coargs.push(cont);
        let new_args = self.args.iter().cloned().map(|p| p.compile(st)).collect();
        core::syntax::Fun {
            name: self.name,
            producers: new_args,
            consumers: new_coargs,
        }
    }
}

impl Compile for fun::syntax::Constructor {
    type Target = core::syntax::Constructor;
    type TargetInner = core::syntax::Statement;
    type Continuation = core::syntax::Consumer;

    fn compile(self, st: &mut CompileState) -> Self::Target {
        let new_prods = self.args.iter().cloned().map(|t| t.compile(st)).collect();
        core::syntax::Constructor {
            id: self.id.compile(st),
            producers: new_prods,
            consumers: vec![],
        }
    }

    fn compile_inner(self, cont: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        let new_cons = self.compile(st);
        core::syntax::Cut {
            producer: Rc::new(new_cons.into()),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
