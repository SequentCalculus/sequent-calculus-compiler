//! This module defines the translation of arguments.

use std::collections::HashMap;

use crate::{
    compile::{Compile, CompilePoly, CompileState},
    types::{compile_ty, compile_ty_poly},
};
use core_lang::syntax::{names::Identifier, terms::Cns};
use fun::traits::OptTyped;

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
                        var: Identifier::new(var),
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

pub fn compile_subst_poly(
    arguments: fun::syntax::arguments::Arguments,
    state: &mut CompileState,
    type_params: &HashMap<String, Identifier>,
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
                        var: Identifier::new(var),
                        ty: compile_ty_poly(
                            &ty.expect("Types should be annotated before translation"),
                            type_params,
                        ),
                    }
                    .into(),
                ),
                term => {
                    let ty = compile_ty_poly(
                        &term
                            .get_type()
                            .expect("Types should be annotated before translation"),
                        type_params,
                    );
                    core_lang::syntax::arguments::Argument::Producer(term.compile_poly(
                        state,
                        ty,
                        type_params,
                    ))
                }
            })
            .collect(),
    }
}
