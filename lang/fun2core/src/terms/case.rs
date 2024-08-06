use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Case {
    type TargetInner = core::syntax::Statement;

    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
        .into()
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
            .map(|clause| compile_clause(clause, cont.clone(), st))
            .collect();
        let new_cont = core::syntax::Consumer::Case(clauses_compiled);
        Rc::unwrap_or_clone(self.destructee).compile_with_cont(new_cont, st)
    }
}
fn compile_clause(
    clause: fun::syntax::Clause<fun::syntax::Ctor>,
    cont: core::syntax::Consumer,
    st: &mut CompileState,
) -> core::syntax::Clause<core::syntax::Ctor> {
    core::syntax::Clause {
        xtor: clause.xtor.compile(st),
        vars: clause.vars,
        covars: vec![],
        rhs: Rc::new(clause.rhs.compile_with_cont(cont, st)),
    }
}
