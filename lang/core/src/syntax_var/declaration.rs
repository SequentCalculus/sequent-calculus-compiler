use printer::{theme::ThemeExt, tokens::TYPE, util::BracesExt, DocAllocator, Print};

use crate::syntax::Ty;

use super::{Chirality, FsContextBinding, FsTypingContext, Name};

#[derive(Debug, Clone, PartialEq)]
pub struct FsXtorSig {
    pub name: Name,
    pub args: FsTypingContext,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FsTypeDeclaration {
    pub name: Name,
    pub xtors: Vec<FsXtorSig>,
}

#[must_use]
pub fn cont_int() -> FsTypeDeclaration {
    FsTypeDeclaration {
        name: "_Cont".to_string(),
        xtors: vec![FsXtorSig {
            name: "_Ret".to_string(),
            args: vec![FsContextBinding {
                var: "x".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Int(),
            }],
        }],
    }
}

#[must_use]
pub fn lookup_type_declaration<'a>(
    type_name: &String,
    types: &'a [FsTypeDeclaration],
) -> &'a FsTypeDeclaration {
    let type_declaration = types
        .iter()
        .find(|declaration| declaration.name == *type_name)
        .expect("Type {type_name} not found");
    type_declaration
}

impl Print for FsXtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.args.is_empty() {
            alloc.text(&self.name)
        } else {
            alloc
                .text(&self.name)
                .append(self.args.print(cfg, alloc).parens())
        }
    }
}

impl Print for FsTypeDeclaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(TYPE)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(alloc.space())
            .append(
                alloc
                    .space()
                    .append(self.xtors.print(cfg, alloc))
                    .append(alloc.space())
                    .braces_anno(),
            )
    }
}
