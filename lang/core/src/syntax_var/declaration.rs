use printer::{theme::ThemeExt, tokens::TYPE, util::BracesExt, DocAllocator, Print};

use crate::syntax::Ty;

use super::{Chirality, ContextBinding, Name, TypingContext};

#[derive(Debug, Clone, PartialEq)]
pub struct XtorSig {
    pub name: Name,
    pub args: TypingContext,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclaration {
    pub name: Name,
    pub xtors: Vec<XtorSig>,
}

#[must_use]
pub fn cont_int() -> TypeDeclaration {
    TypeDeclaration {
        name: "_Cont".to_string(),
        xtors: vec![XtorSig {
            name: "_Ret".to_string(),
            args: vec![ContextBinding {
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
    types: &'a [TypeDeclaration],
) -> &'a TypeDeclaration {
    let type_declaration = types
        .iter()
        .find(|declaration| declaration.name == *type_name)
        .expect("Type {type_name} not found");
    type_declaration
}

impl Print for XtorSig {
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

impl Print for TypeDeclaration {
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
