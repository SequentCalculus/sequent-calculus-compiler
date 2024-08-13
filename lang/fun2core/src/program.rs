//! Compiling a program from the source language `Fun` to the intermediate language `Core`.
use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};
use fun::syntax::Covariable;

pub fn compile_def<T>(def: fun::program::Def<T>) -> core::syntax::Def<T> {
    let mut initial_state: CompileState = CompileState {
        covars: def.cont.iter().map(|(cv, _)| cv).cloned().collect(),
    };
    let new_body: Rc<core::syntax::Producer> = Rc::new(def.body.compile_opt(&mut initial_state));
    initial_state.add_covars(Rc::as_ref(&new_body));
    let new_cv: Covariable = initial_state.free_covar_from_state();
    let new_covar: Rc<core::syntax::Consumer> = Rc::new(
        core::syntax::Covariable {
            covar: new_cv.clone(),
        }
        .into(),
    );
    let new_cut: core::syntax::Statement = core::syntax::Cut {
        producer: new_body,
        consumer: new_covar,
    }
    .into();
    let mut new_cont: Vec<(Covariable, T)> = def.cont;
    new_cont.insert(new_cont.len(), (new_cv, def.ret_ty));
    core::syntax::Def {
        name: def.name,
        pargs: def.args,
        cargs: new_cont,
        body: new_cut,
    }
}

pub fn compile_prog<T: Clone>(prog: fun::program::Prog<T>) -> core::syntax::Prog<T> {
    core::syntax::Prog {
        prog_defs: prog
            .prog_defs
            .iter()
            .cloned()
            .map(|x| compile_def(x.clone()))
            .collect(),
    }
}
