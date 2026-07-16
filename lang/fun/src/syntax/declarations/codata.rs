//! This module contains the declaration of codata type templates.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{CODATA, COLON, COMMA};
use printer::*;

use crate::syntax::*;
use crate::typing::check::check_overlapping_type_params;
use crate::typing::*;

/// This struct defines a codata type destructor. It consists of a name (unique within its type), an optional list of type parameters,
/// a typing context defining its argument types, and a return type. The latter two can contain
/// type parameters abstracted by the codata type template or the signature itself.
///
/// Example:
/// ```text
/// apply(x: A): B
/// ```
/// ```text
/// head[B]: B
/// ```
/// `apply` is a destructor with a single (producer) argument `x` of type `A` and return type `B`,
/// where `A` and `B` are type parameter.
#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct DtorSig {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Option<SourceSpan>,
    /// The dstructor name
    pub name: Name,
    /// The type parameters instantiating the type parameters of the codata type and the destructor
    pub type_params: TypeContext,
    /// The argument context
    pub args: TypingContext,
    /// The return type
    pub cont_ty: Ty,
}

impl DtorSig {
    /// This function checks the well-formedness of the dstructor by checking the argument context
    /// and the return type.
    /// - `symbol_table` is the symbol table during typechecking.
    /// - `type_params` is the list of type parameters of the template the constructor is in.
    fn check(&self, symbol_table: &SymbolTable, type_params: &TypeContext) -> Result<(), Error> {
        let extended_type_params = type_params.extend(self.type_params.clone());
        self.args
            .check_template(symbol_table, &extended_type_params)?;
        self.cont_ty
            .check_template(self.span, symbol_table, &extended_type_params)?;
        Ok(())
    }
}

impl Print for DtorSig {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.bindings.is_empty() {
            self.args.print(cfg, alloc)
        } else {
            self.args.print(cfg, alloc).parens()
        };

        alloc
            .dtor(&self.name)
            .append(self.type_params.print(cfg, alloc))
            .append(args.group())
            .append(COLON)
            .append(alloc.space())
            .append(self.cont_ty.print(cfg, alloc))
    }
}

/// This struct defines a user-declared codata type template. It consist of a name (unique in the
/// program), a list of type parameters, and a list of destructors.
///
/// Example:
/// ```text
/// codata Fun[A, B] { apply(x: A): B }
/// ```
/// `Fun` is a codata type of (first-class) functions with two type arguments `A` and `B`. It has
/// a single destructor `apply` with an argument of type `A` and return type `B`.
#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct Codata {
    #[derivative(PartialEq = "ignore")]
    /// The source location
    pub span: Option<SourceSpan>,
    /// The codata type name
    pub name: Name,
    /// The type parameters
    pub type_params: TypeContext,
    /// The list of destructors
    pub dtors: Vec<DtorSig>,
}

impl Codata {
    /// This function checks the well-formedness of the codata type template by checking each
    /// destructor and checks for overlapping type parameters.
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        let dtor_params: Vec<String> = self
            .dtors
            .iter()
            .flat_map(|dtor| dtor.type_params.bindings.clone())
            .collect();

        if let Some(overlapps) =
            check_overlapping_type_params(&self.type_params.bindings, &dtor_params)
        {
            return Err(Error::DefinedMultipleTimes {
                span: self.span,
                name: overlapps
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            });
        }

        for dtor in &self.dtors {
            dtor.check(symbol_table, &self.type_params)?;
        }
        Ok(())
    }
}

impl From<Codata> for Declaration {
    fn from(codata: Codata) -> Declaration {
        Declaration::Codata(codata)
    }
}

impl Print for Codata {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let head = alloc
            .keyword(CODATA)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(self.type_params.print(cfg, alloc))
            .append(alloc.space());

        let sep = alloc.text(COMMA).append(alloc.line());
        let body = if self.dtors.is_empty() {
            alloc.space()
        } else {
            alloc
                .line()
                .append(
                    alloc.intersperse(self.dtors.iter().map(|dtor| dtor.print(cfg, alloc)), sep),
                )
                .nest(cfg.indent)
                .append(alloc.line())
        };

        head.append(body.braces_anno().group())
    }
}

#[cfg(test)]
mod codata_tests {
    use crate::{
        syntax::{Codata, DtorSig, Ty, TypeArgs, TypeContext, TypingContext},
        test_common::codata_stream,
        typing::symbol_table::{self, BuildSymbolTable, SymbolTable},
    };
    use printer::Print;

    #[test]
    fn display_stream() {
        let result = codata_stream().print_to_string(Default::default());
        let expected = "codata Stream[A] { head: A, tail: Stream[A] }";
        assert_eq!(result, expected)
    }

    #[test]
    fn codata_check() {
        let mut symbol_table = SymbolTable::default();
        codata_stream().build(&mut symbol_table).unwrap();
        let result = codata_stream().check(&mut symbol_table);
        assert!(result.is_ok())
    }

    #[test]
    fn check_overlapping_type_params() {
        let data = Codata {
            span: None,
            name: "Box".to_owned(),
            type_params: TypeContext {
                span: None,
                bindings: vec!["A".to_owned()].into(),
            },
            dtors: vec![DtorSig {
                span: None,
                name: "Pack".to_owned(),
                type_params: TypeContext {
                    span: None,
                    bindings: vec!["A".to_owned()].into(),
                },
                args: TypingContext::default(),
                cont_ty: Ty::mk_decl(&"A", TypeArgs::default()),
            }],
        };

        let mut symbol_table = symbol_table::SymbolTable::default();
        data.build(&mut symbol_table).unwrap();
        assert!(data.check(&symbol_table).is_err());
    }
}
