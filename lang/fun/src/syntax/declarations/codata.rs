//! This module contains the declaration of codata type templates.

use codespan::Span;
use derivative::Derivative;
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{CODATA, COLON, COMMA},
    util::BracesExt,
};

use crate::{
    syntax::{
        context::{TypeContext, TypingContext},
        names::Name,
        types::Ty,
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

use super::Declaration;

/// This struct defines a codata type destructor. It consists of a name (unique within its type),
/// a typing context defining its argument types, and a return type. The latter two can contain
/// type parameters abstracted by the codata type template.
///
/// Example:
/// ```text
/// Apply(x: A): B
/// ```
/// `Apply` is a destructor with a single (producer) argument `x` of type `A` and return type `B`,
/// where `A` and `B` are type parameter.
#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct DtorSig {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The dstructor name
    pub name: Name,
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
        self.args.check_template(symbol_table, type_params)?;
        self.cont_ty
            .check_template(&self.span, symbol_table, type_params)?;
        Ok(())
    }
}

impl Print for DtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .dtor(&self.name)
            .append(self.args.print(cfg, alloc))
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
/// codata Fun[A, B] { Apply(x: A): B }
/// ```
/// `Fun` is a codata type of (first-class) functions with two type arguments `A` and `B`. It has
/// a single destructor `Apply` with an argument of type `A` and return type `B`.
#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct Codata {
    #[derivative(PartialEq = "ignore")]
    /// The source location
    pub span: Span,
    /// The codata type name
    pub name: Name,
    /// The type parameters
    pub type_params: TypeContext,
    /// The list of destructors
    pub dtors: Vec<DtorSig>,
}

impl Codata {
    /// This function checks the well-formedness of the codata type template by checking each
    /// destructor.
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
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
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = alloc
            .keyword(CODATA)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(self.type_params.print(cfg, alloc))
            .append(alloc.space());

        let sep = alloc.text(COMMA).append(alloc.line());

        let body = if self.dtors.is_empty() {
            alloc.space().braces_anno()
        } else {
            alloc
                .line()
                .append(
                    alloc.intersperse(self.dtors.iter().map(|dtor| dtor.print(cfg, alloc)), sep),
                )
                .nest(cfg.indent)
                .append(alloc.line())
                .braces_anno()
        };

        head.append(body.group())
    }
}

#[cfg(test)]
mod codata_tests {
    use crate::{
        test_common::codata_stream,
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
    };
    use printer::Print;

    #[test]
    fn display_stream() {
        let result = codata_stream().print_to_string(Default::default());
        let expected = "codata Stream[A] { Hd: A, Tl: Stream[A] }";
        assert_eq!(result, expected)
    }

    #[test]
    fn codata_check() {
        let mut symbol_table = SymbolTable::default();
        codata_stream().build(&mut symbol_table).unwrap();
        let result = codata_stream().check(&mut symbol_table);
        assert!(result.is_ok())
    }
}
