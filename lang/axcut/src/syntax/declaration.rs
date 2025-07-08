//! This module defines user-declared types in AxCut.

use printer::{DocAllocator, Print, theme::ThemeExt, tokens::TYPE, util::BracesExt};

use super::{Name, TypingContext};

/// This struct defines an xtor which represents a constructor or destructor. It consists of a
/// name (unique within its type) and a typing context defining its parameters.
#[derive(Debug, Clone)]
pub struct XtorSig {
    pub name: Name,
    pub args: TypingContext,
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

/// This struct defines a user-declared type. It consist of a name (unique in the program) and a
/// list of xtors (constructors or destructors).
#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    pub name: Name,
    pub xtors: Vec<XtorSig>,
}

impl TypeDeclaration {
    /// This function returns the position of an xtor within a type declaration, i.e., the index of
    /// the xtor in the list of the types' xtors.
    /// - `tag` is the name of the xtor to look up.
    ///
    /// # Panics
    ///
    /// A panic is caused if the xtor is not in the type declaration.
    pub fn xtor_position(&self, tag: &Name) -> usize {
        self.xtors
            .iter()
            .position(|xtor| xtor.name == *tag)
            .unwrap_or_else(|| {
                panic!(
                    "Xtor {tag} not found in type declaration {}",
                    self.print_to_string(None)
                )
            })
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
