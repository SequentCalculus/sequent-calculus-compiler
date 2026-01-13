//! This module defines user-declared data and codata types in Core.

use printer::tokens::{CODATA, COMMA, DATA};
use printer::*;

use crate::syntax::*;

/// This marker trait allows to abstract over the information of whether something is for data or
/// for codata.
pub trait Polarity {
    /// This method returns whether a something is makred as data or not.
    fn is_data(&self) -> bool;
}

/// This marker struct is used to instantiate a type parameter satisfying the [Polarity] marker
/// trait as data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data;

/// This marker struct is used to instantiate a type parameter satisfying the [Polarity] marker
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

impl Polarity for Data {
    fn is_data(&self) -> bool {
        true
    }
}

impl Polarity for Codata {
    fn is_data(&self) -> bool {
        false
    }
}

/// This struct defines an xtor, i.e., a constructor or destructor. It consists of a name (unique
/// within its type) and a typing context defining its parameters. The type parameter `P`
/// determines whether this is a constructor (if `P` is instantiated with [`Data`]) or destructor
/// (if `P` is instantiated with [`Codata`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XtorSig<P: Polarity> {
    /// Whether this is a constructor ([`Data`]) or destructor ([`Codata`])
    pub xtor: P,
    /// The xtor name
    pub name: Name,
    /// The argument context
    pub args: TypingContext,
}

/// Type alias for constructors
pub type CtorSig = XtorSig<Data>;
/// Type alias for destructors
pub type DtorSig = XtorSig<Codata>;

impl<P: Polarity> Print for XtorSig<P> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.bindings.is_empty() {
            self.args.print(cfg, alloc)
        } else {
            self.args.print(cfg, alloc).parens()
        };

        if self.xtor.is_data() {
            alloc.ctor(&self.name).append(args.group())
        } else {
            alloc.dtor(&self.name).append(args.group())
        }
    }
}

/// This struct defines an xtor which represents a constructor or destructor. It consists of a
/// name (unique within its type) and a typing context defining its parameters. The type parameter
/// `P` determines whether this is a [`Data`] type or [`Codata`] type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDeclaration<P: Polarity> {
    /// Whether this is a [`Data`] or [`Codata`] type
    pub dat: P,
    /// The type name
    pub name: Name,
    /// The xtors of the type
    pub xtors: Vec<XtorSig<P>>,
}

/// Type alias for data types
pub type DataDeclaration = TypeDeclaration<Data>;
/// Type alias for codata types
pub type CodataDeclaration = TypeDeclaration<Codata>;

impl<P: Print + Polarity> Print for TypeDeclaration<P> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = self
            .dat
            .print(cfg, alloc)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(alloc.space());

        let sep = alloc.text(COMMA).append(alloc.line());
        let body = if self.xtors.is_empty() {
            alloc.space()
        } else {
            alloc
                .line()
                .append(
                    alloc.intersperse(self.xtors.iter().map(|xtor| xtor.print(cfg, alloc)), sep),
                )
                .nest(cfg.indent)
                .append(alloc.line())
        };

        head.append(body.braces_anno().group())
    }
}

/// This function looks up a type declaration from its name in a list of type declarations.
///
/// # Panics
///
/// A panic is caused if the type declaration is not contained in the list.
pub fn lookup_type_declaration<'a, P: Polarity>(
    type_name: &String,
    types: &'a [TypeDeclaration<P>],
) -> &'a TypeDeclaration<P> {
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
                    var: Var {
                        name: "x".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Prd,
                    ty: Ty::I64,
                }],
            },
        }],
    }
}
