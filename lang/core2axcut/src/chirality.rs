#[must_use]
pub fn translate_chirality(chi: &core_lang::syntax::Chirality) -> axcut::syntax::Chirality {
    match chi {
        core_lang::syntax::Chirality::Prd => axcut::syntax::Chirality::Prd,
        core_lang::syntax::Chirality::Cns => axcut::syntax::Chirality::Cns,
    }
}
