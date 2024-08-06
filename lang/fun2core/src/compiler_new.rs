use super::definition::{Compile, CompileState, CompileWithCont};
use std::rc::Rc;

impl<T: CompileWithCont + Clone> CompileWithCont for Rc<T> {
    type Target = Rc<T::Target>;
    type TargetInner = Rc<T::TargetInner>;
    type Continuation = T::Continuation;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).compile_opt(st))
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        Rc::new(Rc::unwrap_or_clone(self).compile_with_cont(cont, st))
    }
}

impl CompileWithCont for fun::syntax::Term {
    type Target = core::syntax::Producer;
    type TargetInner = core::syntax::Statement;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::Term::Var(v) => core::syntax::Variable { var: v }.into(),
            fun::syntax::Term::Lit(n) => core::syntax::Literal { lit: n }.into(),
            fun::syntax::Term::Op(op) => op.compile_opt(st).into(),
            fun::syntax::Term::IfZ(ifz) => ifz.compile_opt(st).into(),
            fun::syntax::Term::Let(lt) => lt.compile_opt(st).into(),
            fun::syntax::Term::Fun(fun) => fun.compile_opt(st).into(),
            fun::syntax::Term::Constructor(cons) => cons.compile_opt(st).into(),
            fun::syntax::Term::Destructor(dest) => dest.compile_opt(st).into(),
            fun::syntax::Term::Case(case) => case.compile_opt(st).into(),
            fun::syntax::Term::Cocase(cocase) => cocase.compile_opt(st).into(),
            fun::syntax::Term::Lam(lam) => lam.compile_opt(st).into(),
            fun::syntax::Term::App(ap) => ap.compile_opt(st).into(),
            fun::syntax::Term::Goto(goto) => goto.compile_opt(st).into(),
            fun::syntax::Term::Label(label) => label.compile_opt(st).into(),
            fun::syntax::Term::Paren(p) => (*p.inner.compile_opt(st)).clone(),
        }
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
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
            fun::syntax::Term::Op(op) => op.compile_with_cont(cont, st).into(),
            fun::syntax::Term::IfZ(ifz) => ifz.compile_with_cont(cont, st).into(),
            fun::syntax::Term::Let(lt) => lt.compile_with_cont(cont, st),
            fun::syntax::Term::Fun(fun) => fun.compile_with_cont(cont, st).into(),
            fun::syntax::Term::Constructor(cons) => cons.compile_with_cont(cont, st).into(),
            fun::syntax::Term::Destructor(dest) => dest.compile_with_cont(cont, st),
            fun::syntax::Term::Case(case) => case.compile_with_cont(cont, st),
            fun::syntax::Term::Cocase(cocase) => cocase.compile_with_cont(cont, st).into(),
            fun::syntax::Term::Lam(lam) => lam.compile_with_cont(cont, st).into(),
            fun::syntax::Term::App(ap) => ap.compile_with_cont(cont, st),
            fun::syntax::Term::Goto(goto) => goto.compile_with_cont(cont, st),
            fun::syntax::Term::Label(label) => label.compile_with_cont(cont, st).into(),
            fun::syntax::Term::Paren(p) => (*p.inner.compile_with_cont(cont, st)).clone(),
        }
    }
}

impl CompileWithCont for fun::syntax::Op {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Op;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st.into()),
        }
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        core::syntax::Op {
            fst: self.fst.compile_opt(st),
            op: self.op.compile(st),
            snd: self.snd.compile_opt(st),
            continuation: Rc::new(cont),
        }
    }
}

impl CompileWithCont for fun::syntax::IfZ {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::IfZ;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_cont = core::syntax::Consumer::Covar(new_cv.clone());
        let new_st = self.compile_with_cont(new_cont, st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st.into()),
        }
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        core::syntax::IfZ {
            ifc: self.ifc.compile_opt(st),
            thenc: self.thenc.compile_with_cont(cont.clone(), st),
            elsec: self.elsec.compile_with_cont(cont, st),
        }
    }
}

impl CompileWithCont for fun::syntax::Let {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Statement;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_st = self.bound_term.compile_with_cont(cont, st);
        let new_cont = core::syntax::MuTilde {
            variable: self.variable,
            statement: new_st,
        };
        Rc::unwrap_or_clone(self.in_term).compile_with_cont(new_cont.into(), st)
    }
}

impl CompileWithCont for fun::syntax::Fun {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Fun;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st.into()),
        }
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let mut new_coargs: Vec<core::syntax::Consumer> = self
            .coargs
            .iter()
            .cloned()
            .map(core::syntax::Consumer::Covar)
            .collect();
        new_coargs.push(cont);
        let new_args = self
            .args
            .iter()
            .cloned()
            .map(|p| p.compile_opt(st))
            .collect();
        core::syntax::Fun {
            name: self.name,
            producers: new_args,
            consumers: new_coargs,
        }
    }
}

impl CompileWithCont for fun::syntax::Constructor {
    type Target = core::syntax::Constructor;
    type TargetInner = core::syntax::Cut;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_prods = self
            .args
            .iter()
            .cloned()
            .map(|t| t.compile_opt(st))
            .collect();
        core::syntax::Constructor {
            id: self.id.compile(st),
            producers: new_prods,
            consumers: vec![],
        }
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_cons = self.compile_opt(st);
        core::syntax::Cut {
            producer: Rc::new(new_cons.into()),
            consumer: Rc::new(cont),
        }
    }
}

impl CompileWithCont for fun::syntax::Case {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Statement;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let clauses_compiled = self
            .cases
            .iter()
            .cloned()
            .map(|x| x.compile_with_cont(cont.clone(), st))
            .collect();
        let new_cont = core::syntax::Consumer::Case(clauses_compiled);
        Rc::unwrap_or_clone(self.destructee).compile_with_cont(new_cont, st)
    }
}

impl CompileWithCont for fun::syntax::Clause<fun::syntax::Ctor> {
    type Target = core::syntax::Clause<core::syntax::Ctor>;
    type TargetInner = core::syntax::Clause<core::syntax::Ctor>;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        self.compile_with_cont(core::syntax::Consumer::Covar(new_cv), st)
    }
    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        core::syntax::Clause {
            xtor: self.xtor.compile(st),
            vars: self.vars,
            covars: vec![],
            rhs: Rc::new(self.rhs.compile_with_cont(cont, st)),
        }
    }
}

impl CompileWithCont for fun::syntax::Destructor {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Statement;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_cont = core::syntax::Destructor {
            id: self.id.compile(st),
            producers: self
                .args
                .iter()
                .cloned()
                .map(|p| p.compile_opt(st))
                .collect(),
            consumers: vec![cont],
        }
        .into();
        Rc::unwrap_or_clone(self.destructee).compile_with_cont(new_cont, st)
    }
}

impl CompileWithCont for fun::syntax::Cocase {
    type Target = core::syntax::Cocase;
    type TargetInner = core::syntax::Cut;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        core::syntax::Cocase {
            cocases: self
                .cocases
                .iter()
                .cloned()
                .map(|cc| cc.compile_opt(st))
                .collect(),
        }
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_cocase = self.compile_opt(st).into();
        core::syntax::Cut {
            producer: Rc::new(new_cocase),
            consumer: Rc::new(cont),
        }
    }
}

impl CompileWithCont for fun::syntax::Clause<fun::syntax::Dtor> {
    type Target = core::syntax::Clause<core::syntax::Dtor>;
    type TargetInner = core::syntax::Clause<core::syntax::Dtor>;
    type Continuation = ();

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        core::syntax::Clause {
            xtor: self.xtor.compile(st),
            vars: self.vars,
            covars: vec![new_cv.clone()],
            rhs: Rc::new(
                self.rhs
                    .compile_with_cont(core::syntax::Consumer::Covar(new_cv), st),
            ),
        }
    }
    fn compile_with_cont(self, _: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        self.compile_opt(st)
    }
}

impl CompileWithCont for fun::syntax::Lam {
    type Target = core::syntax::Cocase;
    type TargetInner = core::syntax::Cut;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        core::syntax::Cocase {
            cocases: vec![core::syntax::Clause {
                xtor: core::syntax::Dtor::Ap,
                vars: vec![self.variable],
                covars: vec![new_cv.clone()],
                rhs: self
                    .body
                    .compile_with_cont(core::syntax::Consumer::Covar(new_cv), st),
            }],
        }
    }
    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_prod = self.compile_opt(st).into();
        core::syntax::Cut {
            producer: Rc::new(new_prod),
            consumer: Rc::new(cont),
        }
    }
}

impl CompileWithCont for fun::syntax::App {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Statement;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
    }
    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_cont = core::syntax::Destructor {
            id: core::syntax::Dtor::Ap,
            producers: vec![Rc::unwrap_or_clone(self.argument).compile_opt(st)],
            consumers: vec![cont],
        }
        .into();
        Rc::unwrap_or_clone(self.function).compile_with_cont(new_cont, st)
    }
}

impl CompileWithCont for fun::syntax::Goto {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Statement;
    type Continuation = core::syntax::Consumer;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
    }

    fn compile_with_cont(self, _: Self::Continuation, st: &mut CompileState) -> Self::TargetInner {
        Rc::unwrap_or_clone(self.term)
            .compile_with_cont(core::syntax::Consumer::Covar(self.target), st)
    }
}

impl CompileWithCont for fun::syntax::Label {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Cut;
    type Continuation = core::syntax::Consumer;
    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st.into()),
        }
    }

    fn compile_with_cont(
        self,
        cont: Self::Continuation,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_cont = core::syntax::Consumer::Covar(self.label.clone());
        let new_st = self.term.compile_with_cont(new_cont, st);
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
