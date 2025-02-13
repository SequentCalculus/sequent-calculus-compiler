use printer::{
    theme::ThemeExt,
    tokens::{CODATA, DATA},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{Chirality, ContextBinding, Name, Ty, TypingContext};

// Data / Codata
//
//

pub trait DataCodata {
    fn is_data(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Codata;

impl Print for Data {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(DATA)
    }
}

impl Print for Codata {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(CODATA)
    }
}

impl DataCodata for Data {
    fn is_data(&self) -> bool {
        true
    }
}

impl DataCodata for Codata {
    fn is_data(&self) -> bool {
        false
    }
}

// XtorSig / CtorSig / DtorSig
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XtorSig<T: DataCodata> {
    pub xtor: T,
    pub name: Name,
    pub args: TypingContext,
}

pub type CtorSig = XtorSig<Data>;
pub type DtorSig = XtorSig<Codata>;

impl<T: DataCodata> Print for XtorSig<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.xtor.is_data() {
            alloc.ctor(&self.name).append(self.args.print(cfg, alloc))
        } else {
            alloc.dtor(&self.name).append(self.args.print(cfg, alloc))
        }
    }
}

// TypeDeclaration / DataDeclaration / CodataDeclaration
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDeclaration<T: DataCodata> {
    pub dat: T,
    pub name: Name,
    pub xtors: Vec<XtorSig<T>>,
}

pub type DataDeclaration = TypeDeclaration<Data>;
pub type CodataDeclaration = TypeDeclaration<Codata>;

impl<T: Print + DataCodata> Print for TypeDeclaration<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.dat
            .print(cfg, alloc)
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

#[must_use]
pub fn lookup_type_declaration<'a, T: DataCodata>(
    type_name: &String,
    types: &'a [TypeDeclaration<T>],
) -> &'a TypeDeclaration<T> {
    let type_declaration = types
        .iter()
        .find(|declaration| declaration.name == *type_name)
        .expect("Type {type_name} not found");
    type_declaration
}

#[must_use]
pub fn cont_int() -> DataDeclaration {
    DataDeclaration {
        dat: Data,
        name: "_Cont".to_string(),
        xtors: vec![CtorSig {
            xtor: Data,
            name: "Ret".to_string(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: "x".to_string(),
                    chi: Chirality::Prd,
                    ty: Ty::I64,
                }],
            },
        }],
    }
}
