//! This module defines the translation of arguments.

use crate::{
    compile::{Compile, CompileState},
    names::compile_var,
    types::compile_ty,
};
use core_lang::syntax::terms::Cns;
use fun::syntax::types::OptTyped;

/// This function translates [arguments in Fun](fun::syntax::arguments::Arguments) to
/// [arguments in Core](core_lang::syntax::arguments::Arguments).
///
/// # Panics
///
/// A panic is caused if the types are not annotated in the program.
pub fn compile_subst(
    arguments: fun::syntax::arguments::Arguments,
    state: &mut CompileState,
) -> core_lang::syntax::arguments::Arguments {
    core_lang::syntax::arguments::Arguments {
        entries: arguments
            .entries
            .into_iter()
            .map(|term| match term {
                fun::syntax::terms::Term::XVar(fun::syntax::terms::XVar {
                    var,
                    ty,
                    chi: Some(fun::syntax::context::Chirality::Cns),
                    ..
                }) => core_lang::syntax::arguments::Argument::Consumer(
                    core_lang::syntax::terms::XVar {
                        prdcns: Cns,
                        var: compile_var(var),
                        ty: compile_ty(&ty.expect("Types should be annotated before translation")),
                    }
                    .into(),
                ),
                term => {
                    let ty = compile_ty(
                        &term
                            .get_type()
                            .expect("Types should be annotated before translation"),
                    );
                    core_lang::syntax::arguments::Argument::Producer(term.compile(state, ty))
                }
            })
            .collect(),
    }
}
