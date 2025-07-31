//! This module contains the declaration of data type templates.

use codespan::Span;
use derivative::Derivative;
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{COMMA, DATA},
    util::BracesExt,
};

use crate::{
    syntax::{
        Name,
        context::{TypeContext, TypingContext},
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

use super::Declaration;

/// This struct defines a data type constructor. It consists of a name (unique within its type) and
/// a typing context defining its argument types. The latter can contain type parameters abstracted
/// by the data type template.
///
/// Example:
/// ```text
/// Cons(x: A, xs: List[A])
/// ```
/// The constructor `Cons` has two producer arguments, one of type `A` and one of `List[A]`,
/// where `A` is a type parameter.
#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct CtorSig {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The constructor name
    pub name: Name,
    /// The argument context
    pub args: TypingContext,
}

impl CtorSig {
    /// This function checks the well-formedness of the constructor by checking the argument
    /// context.
    /// - `symbol_table` is the symbol table during typechecking.
    /// - `type_params` is the list of type parameters of the template the constructor is in.
    fn check(&self, symbol_table: &SymbolTable, type_params: &TypeContext) -> Result<(), Error> {
        self.args.check_template(symbol_table, type_params)?;
        Ok(())
    }
}

impl Print for CtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.ctor(&self.name).append(self.args.print(cfg, alloc))
    }
}

/// This struct defines a user-declared data type template. It consist of a name (unique in the
/// program), a list of type parameters, and a list of constructors.
///
/// Example:
/// ```text
/// data List[A] { Nil, Cons(x: A, xs: List[A]) }
/// ```
/// The type `List` has a single type parameter `A` and two constructors `Nil` and `Cons`. `Nil`
/// has no arguments while `Cons` has two of types `A` and `List[A]`.
#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct Data {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The data type name
    pub name: Name,
    /// The type paramenters
    pub type_params: TypeContext,
    /// The constructors
    pub ctors: Vec<CtorSig>,
}

impl Data {
    /// This function checks the well-formedness of the data type template by checking each
    /// constructor.
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        for ctor in &self.ctors {
            ctor.check(symbol_table, &self.type_params)?;
        }
        Ok(())
    }
}

impl From<Data> for Declaration {
    fn from(data: Data) -> Declaration {
        Declaration::Data(data)
    }
}

impl Print for Data {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = alloc
            .keyword(DATA)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(self.type_params.print(cfg, alloc))
            .append(alloc.space());

        let sep = alloc.text(COMMA).append(alloc.line());

        let body = if self.ctors.is_empty() {
            alloc.space().braces_anno()
        } else {
            alloc
                .line()
                .append(
                    alloc.intersperse(self.ctors.iter().map(|ctor| ctor.print(cfg, alloc)), sep),
                )
                .nest(cfg.indent)
                .append(alloc.line())
                .braces_anno()
        };

        head.append(body.group())
    }
}

#[cfg(test)]
mod data_tests {
    use printer::Print;

    use crate::{
        test_common::data_list,
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
    };

    #[test]
    fn display_list() {
        let result = data_list().print_to_string(Default::default());
        let expected = "data List[A] { Nil, Cons(x : A, xs : List[A]) }";
        assert_eq!(result, expected)
    }

    #[test]
    fn data_check() {
        let mut symbol_table = SymbolTable::default();
        data_list().build(&mut symbol_table).unwrap();
        let result = data_list().check(&mut symbol_table);
        assert!(result.is_ok())
    }
}
