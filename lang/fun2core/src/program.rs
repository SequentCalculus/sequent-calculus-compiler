//! Compiling a program from the source language `Fun` to the intermediate language `Core`.

use crate::definition::{CompileState, CompileWithCont};
use fun::syntax::Covariable;

pub fn compile_def<T>(def: fun::program::Def<T>) -> core::syntax::Def<T> {
    let mut initial_state: CompileState = CompileState {
        covars: def.cont.iter().map(|(cv, _)| cv).cloned().collect(),
    };
    let new_covar = initial_state.free_covar_from_state();
    let body = def.body.compile_with_cont(
        core::syntax::Covariable {
            covar: new_covar.clone(),
        }
        .into(),
        &mut initial_state,
    );

    let mut new_cont: Vec<(Covariable, T)> = def.cont;
    new_cont.push((new_covar, def.ret_ty));

    core::syntax::Def {
        name: def.name,
        pargs: def.args,
        cargs: new_cont,
        body,
    }
}

pub fn compile_prog<T: Clone>(prog: fun::program::Prog<T>) -> core::syntax::Prog<T> {
    core::syntax::Prog {
        prog_defs: prog.prog_defs.into_iter().map(|x| compile_def(x)).collect(),
    }
}
