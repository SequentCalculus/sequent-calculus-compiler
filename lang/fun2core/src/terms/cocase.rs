use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Cocase {
    /// ```text
    /// 〚cocase { D_1(x_11,...) => t_1, ...} 〛_{c} =⟨ cocase{D_1(x_11,...;a_1) => 〚t_1〛_{a_1},...} | c⟩
    /// 〚cocase { D_1(x_11,...) => t_1, ...} 〛 = cocase{D_1(x_11,...;a_1) => 〚t_1〛_{a_1},...}
    ///
    /// ```
    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        core::syntax::Cocase {
            cocases: self
                .cocases
                .into_iter()
                .map(|clause| compile_clause(clause, st))
                .collect(),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let new_cocase = self.compile_opt(st);
        core::syntax::Cut {
            producer: Rc::new(new_cocase),
            consumer: Rc::new(cont),
        }
        .into()
    }
}

fn compile_clause(
    clause: fun::syntax::Clause<fun::syntax::Dtor>,
    st: &mut CompileState,
) -> core::syntax::Clause<core::syntax::Dtor> {
    let new_cv = st.free_covar_from_state();
    core::syntax::Clause {
        xtor: clause.xtor.compile(st),
        vars: clause.vars,
        covars: vec![new_cv.clone()],
        rhs: Rc::new(
            clause
                .rhs
                .compile_with_cont(core::syntax::Consumer::Covar(new_cv), st),
        ),
    }
}
