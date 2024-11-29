use printer::{theme::ThemeExt, tokens::TYPE, util::BracesExt, DocAllocator, Print};

use super::{Name, Ty, TypingContext};

#[derive(Debug, Clone)]
pub struct XtorSig {
    pub name: Name,
    pub args: TypingContext,
}

#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    pub name: Name,
    pub xtors: Vec<XtorSig>,
}

impl Print for XtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(&self.name).append(self.args.print(cfg, alloc))
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
            .append(self.xtors.print(cfg, alloc).braces_anno())
    }
}

#[must_use]
pub fn lookup_type_declaration<'a>(ty: &Ty, types: &'a [TypeDeclaration]) -> &'a TypeDeclaration {
    if let Ty::Decl(type_name) = ty {
        let type_declaration = types
            .iter()
            .find(|declaration| declaration.name == *type_name)
            .expect("Type {type_name} not found");
        type_declaration
    } else {
        panic!("User-defined type cannot be {}", ty.print_to_string(None));
    }
}

#[must_use]
pub fn xtor_position(tag: &Name, type_declaration: &TypeDeclaration) -> usize {
    type_declaration
        .xtors
        .iter()
        .position(|xtor| xtor.name == *tag)
        .unwrap_or_else(|| {
            panic!(
                "Constructor {tag} not found in type declaration {}",
                type_declaration.print_to_string(None)
            )
        })
}
