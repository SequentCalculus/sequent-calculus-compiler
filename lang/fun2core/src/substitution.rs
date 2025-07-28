//! Compilation for [fun::syntax::substitution::Substitution]
//! Compiles to [core_lang::syntax::substitution::Substitution]
use crate::{
    compile::{CompileState, CompileWithCont},
    types::compile_ty,
};
use core_lang::syntax::terms::Cns;
use fun::syntax::types::OptTyped;

/// Compiles a (typechecked) [fun::syntax::substitution::Substitution] to [core_lang]
pub fn compile_subst(
    subst: fun::syntax::substitution::Substitution,
    state: &mut CompileState,
) -> core_lang::syntax::substitution::Substitution {
    core_lang::syntax::substitution::Substitution {
        bindings: subst
            .into_iter()
            .map(|term| match term {
                fun::syntax::terms::Term::XVar(fun::syntax::terms::XVar {
                    var,
                    ty,
                    chi: Some(fun::syntax::context::Chirality::Cns),
                    ..
                }) => core_lang::syntax::substitution::SubstitutionBinding::ConsumerBinding(
                    core_lang::syntax::terms::XVar {
                        prdcns: Cns,
                        var,
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
                    core_lang::syntax::substitution::SubstitutionBinding::ProducerBinding(
                        term.compile_opt(state, ty),
                    )
                }
            })
            .collect(),
    }
}
