#[must_use]
pub fn translate_chirality(chi: &core::syntax::Chirality) -> axcut::syntax::Chirality {
    match chi {
        core::syntax::Chirality::Prd => axcut::syntax::Chirality::Prd,
        core::syntax::Chirality::Cns => axcut::syntax::Chirality::Cns,
    }
}
