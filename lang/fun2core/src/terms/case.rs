// Case
//
//

use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl Compile for fun::syntax::Case {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let p = self.destructee.compile(state);
        state.add_covars(Rc::as_ref(&p));
        let rhs_comp: Vec<core::syntax::Producer> = self
            .cases
            .iter()
            .cloned()
            .map(|pt| pt.rhs.compile(state))
            .collect();
        let _ = rhs_comp.iter().map(|p| state.add_covars(p));
        let new_cv = state.free_covar_from_state();
        let new_covar: Rc<core::syntax::Consumer> =
            Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
        let rhs_cuts: Vec<Rc<core::syntax::Statement>> = rhs_comp
            .iter()
            .cloned()
            .map(|p| {
                Rc::new(
                    core::syntax::Cut {
                        producer: Rc::new(p),
                        consumer: Rc::clone(&new_covar),
                    }
                    .into(),
                )
            })
            .collect();
        let mut new_pts: Vec<core::syntax::Clause<core::syntax::Ctor>> = vec![];
        for i in 0..self.cases.len() - 1 {
            let pt_i: &fun::syntax::Clause<fun::syntax::Ctor> = self
                .cases
                .get(i)
                .expect("Invalid pattern (should never happen");
            let rhs_i: &Rc<core::syntax::Statement> = rhs_cuts
                .get(i)
                .expect("Invalid pattern (should never happen");
            let new_pt: core::syntax::Clause<core::syntax::Ctor> = core::syntax::Clause {
                xtor: pt_i.xtor.clone().compile(state),
                vars: pt_i.vars.clone(),
                covars: vec![],
                rhs: Rc::clone(rhs_i),
            };
            new_pts.insert(0, new_pt);
        }
        let new_cut: Rc<core::syntax::Statement> = Rc::new(
            core::syntax::Cut {
                producer: p,
                consumer: Rc::new(core::syntax::Consumer::Case(new_pts)),
            }
            .into(),
        );
        core::syntax::Mu {
            covariable: new_cv,
            statement: new_cut,
        }
        .into()
    }
}

impl CompileWithCont for fun::syntax::Case {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Statement;

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
        cont: core::syntax::Consumer,
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

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        self.compile_with_cont(core::syntax::Consumer::Covar(new_cv), st)
    }
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
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
