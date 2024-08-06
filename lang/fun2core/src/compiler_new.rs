use super::definition::CompileState;
use std::rc::Rc;

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

impl Compile for fun::syntax::Dtor {
    type Target = core::syntax::Dtor;
    type TargetInner = core::syntax::Dtor;
    type Continuation = ();

    fn compile(self, _: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::Dtor::Hd => core::syntax::Dtor::Hd,
            fun::syntax::Dtor::Tl => core::syntax::Dtor::Tl,
            fun::syntax::Dtor::Fst => core::syntax::Dtor::Fst,
            fun::syntax::Dtor::Snd => core::syntax::Dtor::Snd,
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
            fun::syntax::Term::Destructor(dest) => dest.compile(st).into(),
            fun::syntax::Term::Case(case) => case.compile(st).into(),
            fun::syntax::Term::Cocase(cocase) => cocase.compile(st).into(),
            fun::syntax::Term::Lam(lam) => lam.compile(st).into(),
            fun::syntax::Term::App(ap) => ap.compile(st).into(),
            fun::syntax::Term::Goto(goto) => goto.compile(st).into(),
            fun::syntax::Term::Label(label) => label.compile(st).into(),
            fun::syntax::Term::Paren(p) => (*p.inner.compile(st)).clone(),
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
            fun::syntax::Term::Constructor(cons) => cons.compile_inner(cont, st).into(),
            fun::syntax::Term::Destructor(dest) => dest.compile_inner(cont, st),
            fun::syntax::Term::Case(case) => case.compile_inner(cont, st),
            fun::syntax::Term::Cocase(cocase) => cocase.compile_inner(cont, st).into(),
            fun::syntax::Term::Lam(lam) => lam.compile_inner(cont, st).into(),
            fun::syntax::Term::App(ap) => ap.compile_inner(cont, st),
            fun::syntax::Term::Goto(goto) => goto.compile_inner(cont, st),
            fun::syntax::Term::Label(label) => label.compile_inner(cont, st).into(),
            fun::syntax::Term::Paren(p) => (*p.inner.compile_inner(cont, st)).clone(),
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
        Rc::unwrap_or_clone(self.in_term).compile_inner(new_cont.into(), st)
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
    type TargetInner = core::syntax::Cut;
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
    }
}

impl Compile for fun::syntax::Case {
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
        let clauses_compiled = self
            .cases
            .iter()
            .cloned()
            .map(|x| x.compile_inner(cont.clone(), st))
            .collect();
        let new_cont = core::syntax::Consumer::Case(clauses_compiled);
        Rc::unwrap_or_clone(self.destructee).compile_inner(new_cont, st)
    }
}

impl Compile for fun::syntax::Clause<fun::syntax::Ctor> {
    type Target = core::syntax::Clause<core::syntax::Ctor>;
    type TargetInner = core::syntax::Clause<core::syntax::Ctor>;
    type Continuation = core::syntax::Consumer;

    fn compile(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        self.compile_inner(core::syntax::Consumer::Covar(new_cv), st)
    }
    fn compile_inner(self, cont: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        core::syntax::Clause {
            xtor: self.xtor.compile(st),
            vars: self.vars,
            covars: vec![],
            rhs: Rc::new(self.rhs.compile_inner(cont, st)),
        }
    }
}

impl Compile for fun::syntax::Destructor {
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
        let new_cont = core::syntax::Destructor {
            id: self.id.compile(st),
            producers: self.args.iter().cloned().map(|p| p.compile(st)).collect(),
            consumers: vec![cont],
        }
        .into();
        Rc::unwrap_or_clone(self.destructee).compile_inner(new_cont, st)
    }
}

impl Compile for fun::syntax::Cocase {
    type Target = core::syntax::Cocase;
    type TargetInner = core::syntax::Cut;
    type Continuation = core::syntax::Consumer;

    fn compile(self, st: &mut CompileState) -> Self::Target {
        core::syntax::Cocase {
            cocases: self
                .cocases
                .iter()
                .cloned()
                .map(|cc| cc.compile(st))
                .collect(),
        }
    }

    fn compile_inner(self, cont: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        let new_cocase = self.compile(st).into();
        core::syntax::Cut {
            producer: Rc::new(new_cocase),
            consumer: Rc::new(cont),
        }
    }
}

impl Compile for fun::syntax::Clause<fun::syntax::Dtor> {
    type Target = core::syntax::Clause<core::syntax::Dtor>;
    type TargetInner = core::syntax::Clause<core::syntax::Dtor>;
    type Continuation = ();

    fn compile(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        core::syntax::Clause {
            xtor: self.xtor.compile(st),
            vars: self.vars,
            covars: vec![new_cv.clone()],
            rhs: Rc::new(
                self.rhs
                    .compile_inner(core::syntax::Consumer::Covar(new_cv), st),
            ),
        }
    }
    fn compile_inner(self, _: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        self.compile(st)
    }
}

impl Compile for fun::syntax::Lam {
    type Target = core::syntax::Cocase;
    type TargetInner = core::syntax::Cut;
    type Continuation = core::syntax::Consumer;

    fn compile(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        core::syntax::Cocase {
            cocases: vec![core::syntax::Clause {
                xtor: core::syntax::Dtor::Ap,
                vars: vec![self.variable],
                covars: vec![new_cv.clone()],
                rhs: self
                    .body
                    .compile_inner(core::syntax::Consumer::Covar(new_cv), st),
            }],
        }
    }
    fn compile_inner(self, cont: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        let new_prod = self.compile(st).into();
        core::syntax::Cut {
            producer: Rc::new(new_prod),
            consumer: Rc::new(cont),
        }
    }
}

impl Compile for fun::syntax::App {
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
        let new_cont = core::syntax::Destructor {
            id: core::syntax::Dtor::Ap,
            producers: vec![Rc::unwrap_or_clone(self.argument).compile(st)],
            consumers: vec![cont],
        }
        .into();
        Rc::unwrap_or_clone(self.function).compile_inner(new_cont, st)
    }
}

impl Compile for fun::syntax::Goto {
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

    fn compile_inner(self, _: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        Rc::unwrap_or_clone(self.term).compile_inner(core::syntax::Consumer::Covar(self.target), st)
    }
}

impl Compile for fun::syntax::Label {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Cut;
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
        let new_cont = core::syntax::Consumer::Covar(self.label.clone());
        let new_st = self.term.compile_inner(new_cont, st);
        let new_mu = core::syntax::Mu {
            covariable: self.label,
            statement: new_st,
        };
        core::syntax::Cut {
            producer: Rc::new(new_mu.into()),
            consumer: Rc::new(cont),
        }
    }
}
