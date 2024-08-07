use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Case {
    /// ```text
    /// 〚case t of { K_1(x_11,...) => t_1, ...} 〛_{c} = 〚t〛_{case{ K_1(x_11,...) => 〚t_1〛_{c}, ... }}
    ///
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let clauses_compiled = self
            .cases
            .into_iter()
            .map(|clause| compile_clause(clause, cont.clone(), st))
            .collect();
        //the new continuation case{ K_1(x_11,...) => 〚t_1〛_{c}, ... }
        let new_cont = core::syntax::Consumer::Case(clauses_compiled);
        //〚t〛_{new_cont}
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
