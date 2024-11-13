#[must_use]
pub fn translate_chirality(chi: &core::syntax_var::Chirality) -> axcut::syntax::Chirality {
    match chi {
        core::syntax_var::Chirality::Prd => axcut::syntax::Chirality::Prd,
        core::syntax_var::Chirality::Cns => axcut::syntax::Chirality::Cns,
    }
}
