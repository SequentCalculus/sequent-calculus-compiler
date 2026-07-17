//! This module defines user-declared data and codata types in Core.

use crate::mono::specialize::{Specialize, SpecializeContext};
use crate::typing::check::Checked;
use crate::typing::env::GlobalEnv;
use crate::typing::errors::LocatedTypeError;
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
/// within its type), type arguments, and a typing context defining its parameters. The type parameter `P`
/// determines whether this is a constructor (if `P` is instantiated with [`Data`]) or destructor
/// (if `P` is instantiated with [`Codata`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XtorSig<P: Polarity> {
    /// Whether this is a constructor ([`Data`]) or destructor ([`Codata`])
    pub xtor: P,
    /// The xtor name
    pub name: Identifier,
    /// The type parameters of the xtor
    pub type_params: Vec<Identifier>,
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

        let type_params = if self.type_params.is_empty() {
            alloc.nil()
        } else {
            self.type_params.print(cfg, alloc).brackets()
        };

        if self.xtor.is_data() {
            alloc
                .ctor(&self.name.print_to_string(Some(cfg)))
                .append(type_params)
                .append(args.group())
        } else {
            alloc
                .dtor(&self.name.print_to_string(Some(cfg)))
                .append(type_params)
                .append(args.group())
        }
    }
}

impl<P: Polarity> Checked for XtorSig<P> {
    fn check(
        &self,
        type_params: &[Identifier],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        // extend the type parameters with the type parameters of the xtor
        let extended_type_params = [type_params, &self.type_params].concat();
        self.args.check(&extended_type_params, context, env)
    }
}

impl<P: Polarity + Clone> Specialize for XtorSig<P> {
    fn specialize(&self, context: &SpecializeContext) -> Self {
        XtorSig {
            xtor: self.xtor.clone(),
            name: self.name.clone(),
            type_params: self.type_params.clone(),
            args: self.args.specialize(context),
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
    pub name: Identifier,
    /// The xtors of the type
    pub xtors: Vec<XtorSig<P>>,
    /// The type parameters of the type
    pub type_params: Vec<Identifier>,
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
            .append(alloc.typ(&self.name.print_to_string(Some(cfg))))
            .append(if self.type_params.is_empty() {
                alloc.nil()
            } else {
                alloc
                    .text("[")
                    .append(
                        alloc.intersperse(
                            self.type_params
                                .iter()
                                .map(|param| alloc.typ(&param.print_to_string(Some(cfg)))),
                            alloc.text(COMMA).append(alloc.space()),
                        ),
                    )
                    .append(alloc.text("]"))
            })
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
    type_name: &Identifier,
    types: &'a [TypeDeclaration<P>],
) -> &'a TypeDeclaration<P> {
    types
        .iter()
        .find(|declaration| declaration.name == *type_name)
        .unwrap_or_else(|| panic!("Type {} not found", type_name.name))
}

/// This function returns the data type declaration for continuations of type `i64`, used in the
/// translation to AxCut.
pub fn cont_int() -> DataDeclaration {
    DataDeclaration {
        dat: Data,
        name: Identifier::new("_Cont".to_string()),
        xtors: vec![CtorSig {
            xtor: Data,
            name: Identifier::new("Ret".to_string()),
            type_params: vec![],
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: Identifier::new("x".to_string()),
                    chi: Chirality::Prd,
                    ty: Ty::I64,
                }],
            },
        }],
        type_params: vec![],
    }
}

impl<P: Polarity> Checked for TypeDeclaration<P> {
    fn check(
        &self,
        type_params: &[Identifier],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        // check xtors
        for xtor in &self.xtors {
            xtor.check(type_params, context, env)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod check_tests {
    use crate::{
        syntax::TypingContext,
        typing::{check::Checked, env::GlobalEnv},
    };
    extern crate self as core_lang;
    use core_macros::{bind, ctor_sig, data, id, prd, tvar, ty};

    #[test]
    fn ty_decl_check_arity_and_args() {
        // create a data declaration: List[A]
        let list = data!(id!("List"), [], [id!("A", 1)]);

        // well-formed: List[i64]
        let ty_good = ty!(id!("List"), [ty!("int")]);

        assert!(
            ty_good
                .check(
                    &[],
                    &TypingContext::default(),
                    &GlobalEnv::new(&[list.clone()], &[], &[])
                )
                .is_ok()
        );

        // arity mismatch: List[] against List[A]
        let ty_bad = ty!(id!("List"));

        let res = ty_bad.check(
            &[],
            &TypingContext::default(),
            &GlobalEnv::new(&[list.clone()], &[], &[]),
        );
        assert!(res.is_err());
    }

    #[test]
    fn type_declaration_and_xtor_check() {
        // create a constructor signature with one argument of type i64
        let decl = data!(
            id!("List"),
            [ctor_sig!(
                id!("Cons"),
                [],
                [bind!(id!("x"), prd!(), ty!("int"))]
            )],
            []
        );

        // xtor signature check
        assert!(
            decl.check(
                &[],
                &TypingContext::default(),
                &GlobalEnv::new(&[decl.clone()], &[], &[])
            )
            .is_ok()
        );
    }

    #[test]
    fn existential_type_decl_ok() {
        let box_decl = data!(
            id!("Box"),
            [ctor_sig!(
                id!("Pack"),
                [id!("A", 1)],
                [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))]
            )],
            []
        );

        assert!(
            box_decl
                .check(
                    &[],
                    &TypingContext::default(),
                    &GlobalEnv::new(&[box_decl.clone()], &[], &[]),
                )
                .is_ok()
        );
    }

    #[test]
    fn existential_type_decl_err() {
        let box_decl = data!(
            id!("Box"),
            [ctor_sig!(
                id!("Pack"),
                [],
                [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))]
            )],
            []
        );

        assert!(
            box_decl
                .check(
                    &[],
                    &TypingContext::default(),
                    &GlobalEnv::new(&[box_decl.clone()], &[], &[]),
                )
                .is_err()
        );
    }
}
