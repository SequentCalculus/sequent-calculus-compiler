use crate::context::translate_context;

#[must_use]
pub fn translate_sig(sig: core_lang::syntax::declaration::FsXtorSig) -> axcut::syntax::XtorSig {
    axcut::syntax::XtorSig {
        name: sig.name,
        args: translate_context(sig.args),
    }
}

pub fn translate_declaration(
    declaration: core_lang::syntax::declaration::FsTypeDeclaration,
) -> axcut::syntax::TypeDeclaration {
    axcut::syntax::TypeDeclaration {
        name: declaration.name,
        xtors: declaration.xtors.into_iter().map(translate_sig).collect(),
    }
}
