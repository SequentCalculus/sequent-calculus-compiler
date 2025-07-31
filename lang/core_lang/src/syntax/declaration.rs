//! This module defines user-declared data and codata types in Core.

use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{CODATA, DATA},
    util::BracesExt,
};

use super::{Chirality, ContextBinding, Name, Ty, TypingContext};

/// This marker trait allows to abstract over the information of whether something is for data or
/// for codata.
pub trait DataCodata {
    /// This method returns whether a something is makred as data or not.
    fn is_data(&self) -> bool;
}

/// This marker struct is used to instantiate a type parameter satisfying the [DataCodata] marker
/// trait as data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data;

/// This marker struct is used to instantiate a type parameter satisfying the [DataCodata] marker
/// trait as codata.
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

/// This struct defines an xtor, i.e., a constructor or destructor. It consists of a name (unique
/// within its type) and a typing context defining its parameters. The type parameter `T`
/// determines whether this is a constructor (if `T` is instantiated with [`Data`]) or destructor
/// (if `T` is instantiated with [`Codata`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XtorSig<T: DataCodata> {
    /// Whether this is a constructor ([`Data`]) or destructor ([`Codata`])
    pub xtor: T,
    /// The xtor name
    pub name: Name,
    /// The argument context
    pub args: TypingContext,
}

/// Type alias for constructors
pub type CtorSig = XtorSig<Data>;
/// Type alias for destructors
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

/// This struct defines an xtor which represents a constructor or destructor. It consists of a
/// name (unique within its type) and a typing context defining its parameters. The type parameter
/// `T` determines whether this is a [`Data`] type or [`Codata`] type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDeclaration<T: DataCodata> {
    /// Whether this is a [`Data`] or [`Codata`] type
    pub dat: T,
    /// The type name
    pub name: Name,
    /// The xtors of the type
    pub xtors: Vec<XtorSig<T>>,
}

/// Type alias for data types
pub type DataDeclaration = TypeDeclaration<Data>;
/// Type alias for codata types
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

/// This function looks up a type declaration from its name in a list of type declarations.
///
/// # Panics
///
/// A panic is caused if the type declaration is not contained in the list.
pub fn lookup_type_declaration<'a, T: DataCodata>(
    type_name: &String,
    types: &'a [TypeDeclaration<T>],
) -> &'a TypeDeclaration<T> {
    types
        .iter()
        .find(|declaration| declaration.name == *type_name)
        .unwrap_or_else(|| panic!("Type {type_name} not found"))
}

/// This function returns the data type declaration for continuations of type `i64`, used in the
/// translation to AxCut.
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
