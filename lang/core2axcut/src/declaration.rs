use crate::context::translate_context;

pub fn translate_sig(sig: core::syntax_var::XtorSig) -> axcut::syntax::XtorSig {
    axcut::syntax::XtorSig {
        name: sig.name,
        args: translate_context(sig.args),
    }
}

pub fn translate_declaration(
    declaration: core::syntax_var::TypeDeclaration,
) -> axcut::syntax::TypeDeclaration {
    axcut::syntax::TypeDeclaration {
        name: declaration.name,
        xtors: declaration.xtors.into_iter().map(translate_sig).collect(),
    }
}
