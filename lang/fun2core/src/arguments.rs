//! This module defines the translation of arguments.

use std::{collections::HashMap, rc::Rc};

use crate::{
    compile::{Compile, CompileState},
    types::compile_ty,
};
use core_lang::syntax::{names::Identifier, terms::Cns, type_params::ParamPolarity};
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
    type_params: Rc<HashMap<String, (Identifier, ParamPolarity)>>,
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
                        ty: compile_ty(
                            &ty.expect("Types should be annotated before translation"),
                            type_params.clone(),
                        ),
                    }
                    .into(),
                ),
                term => {
                    let ty = compile_ty(
                        &term
                            .get_type()
                            .expect("Types should be annotated before translation"),
                        type_params.clone(),
                    );
                    core_lang::syntax::arguments::Argument::Producer(term.compile(
                        state,
                        ty,
                        type_params.clone(),
                    ))
                }
            })
            .collect(),
    }
}
