//! Declarations in the core language
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

/// Helper trait for whether something is data or codata
/// only implemented for [Data] and [Codata]
pub trait DataCodata {
    /// Is `&self` data
    fn is_data(&self) -> bool;
}

/// struct for Data types
/// usually used as type argument (e.g. [crate::syntax::terms::Term])
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data;

/// Struct for Codata types
/// usually used as type argument (e.g. [crate::syntax::terms::Term])
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

/// Xtor Signature, that is either a constructor or a destructor signature
/// Whether this is a constructor or destructor is defined by the type argument `T`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XtorSig<T: DataCodata> {
    /// Whether this is a constructor or destructor (either [Data] or [Codata])
    pub xtor: T,
    /// The xtor name
    pub name: Name,
    /// The xtor argument context
    pub args: TypingContext,
}

/// Type alias for constructor signatures (always [Data])
pub type CtorSig = XtorSig<Data>;
/// Type alias for destructyor signatures (always [Codata])
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

/// A declaration for a data or codata type
/// Whether this defines a data or codata type is defined by the type argument `T`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDeclaration<T: DataCodata> {
    /// Whether this is a data or codata type (always [Data] or [Codata])
    pub dat: T,
    /// The Type name
    pub name: Name,
    /// The constructors/destructors of the type
    /// data types only have constructors and codata types only destructors
    pub xtors: Vec<XtorSig<T>>,
}

/// Type alias for data type declarations (always [Data])
pub type DataDeclaration = TypeDeclaration<Data>;
/// Type alias for codata type declarations (always [Codata])
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

/// Find a type declaration from its name
pub fn lookup_type_declaration<'a, T: DataCodata>(
    type_name: &String,
    types: &'a [TypeDeclaration<T>],
) -> &'a TypeDeclaration<T> {
    types
        .iter()
        .find(|declaration| declaration.name == *type_name)
        .unwrap_or_else(|| panic!("Type {type_name} not found"))
}

/// Data type for a continuation of type `i64`
/// Used in the compilation to axcut
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
