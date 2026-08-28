//! This module contains the declaration of data type templates.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{COMMA, DATA};
use printer::*;

use crate::syntax::*;
use crate::typing::check::check_overlapping_type_params;
use crate::typing::*;

use std::collections::HashSet;

/// This struct defines a data type constructor. It consists of a name (unique within its type), optional type parameters, and
/// a typing context defining its argument types. The latter can contain type parameters abstracted
/// by the data type template or the signature itself.
///
/// Example:
/// ```text
/// Cons(x: A, xs: List[A])
/// ```
/// ```text
/// Pack[B](x: B)
/// ```
/// The constructor `Cons` has two producer arguments, one of type `A` and one of `List[A]`,
/// where `A` is a type parameter.
#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct CtorSig {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Option<SourceSpan>,
    /// The constructor name
    pub name: Name,
    /// The type parameters
    pub type_params: TypeParams,
    /// The argument context
    pub args: TypingContext,
}

impl CtorSig {
    /// This function checks the well-formedness of the constructor by checking the argument
    /// context.
    /// - `symbol_table` is the symbol table during typechecking.
    /// - `type_params` is the list of type parameters of the template the constructor is in.
    fn check(&self, symbol_table: &SymbolTable, type_params: &TypeContext) -> Result<(), Error> {
        self.args.check_template(
            symbol_table,
            &type_params.extend(self.type_params.to_type_context()),
        )?;
        Ok(())
    }
}

impl Print for CtorSig {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.bindings.is_empty() {
            self.args.print(cfg, alloc)
        } else {
            self.args.print(cfg, alloc).parens()
        };

        alloc
            .ctor(&self.name)
            .append(self.type_params.print(cfg, alloc))
            .append(args.group())
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
    pub span: Option<SourceSpan>,
    /// The data type name
    pub name: Name,
    /// The type paramenters
    pub type_params: TypeParams,
    /// The constructors
    pub ctors: Vec<CtorSig>,
}

impl Data {
    /// This function checks the well-formedness of the data type template by checking each
    /// constructor and checks for overlapping type parameters.
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        let ctor_params: Vec<String> = self
            .ctors
            .iter()
            .flat_map(|ctor| ctor.type_params.names())
            .collect();

        if let Some(overlapps) =
            check_overlapping_type_params(&self.type_params.names(), &ctor_params)
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

        for ctor in &self.ctors {
            ctor.check(symbol_table, &self.type_params.to_type_context())?;
        }
        Ok(())
    }

    /// This function collects the names of all user-declared types referenced by any
    /// constructor's argument types, excluding this type's own and each constructor's own type
    /// parameters.
    pub fn referenced_types(&self) -> HashSet<Name> {
        let mut out = HashSet::new();
        for ctor in &self.ctors {
            let mut bound: HashSet<Name> = self.type_params.names().into_iter().collect();
            bound.extend(ctor.type_params.names());
            for binding in &ctor.args.bindings {
                binding.ty.collect_referenced_types(&bound, &mut out);
            }
        }
        out
    }
}

impl From<Data> for Declaration {
    fn from(data: Data) -> Declaration {
        Declaration::Data(data)
    }
}

impl Print for Data {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let head = alloc
            .keyword(DATA)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(self.type_params.print(cfg, alloc))
            .append(alloc.space());

        let sep = alloc.text(COMMA).append(alloc.line());
        let body = if self.ctors.is_empty() {
            alloc.space()
        } else {
            alloc
                .line()
                .append(
                    alloc.intersperse(self.ctors.iter().map(|ctor| ctor.print(cfg, alloc)), sep),
                )
                .nest(cfg.indent)
                .append(alloc.line())
        };

        head.append(body.braces_anno().group())
    }
}

#[cfg(test)]
mod data_tests {
    use printer::Print;

    use crate::{
        syntax::{CtorSig, Data, Polarity, TypeParams, TypingContext},
        test_common::data_list,
        typing::symbol_table::{self, BuildSymbolTable, SymbolTable},
    };

    #[test]
    fn display_list() {
        let result = data_list().print_to_string(Default::default());
        let expected = "data List[A+] { Nil, Cons(x: A, xs: List[A]) }";
        assert_eq!(result, expected)
    }

    #[test]
    fn data_check() {
        let mut symbol_table = SymbolTable::default();
        data_list().build(&mut symbol_table).unwrap();
        let result = data_list().check(&mut symbol_table);
        assert!(result.is_ok())
    }

    #[test]
    fn check_overlapping_type_params() {
        let data = Data {
            span: None,
            name: "Box".to_owned(),
            type_params: TypeParams::mk(&[("A", Polarity::Data)]),
            ctors: vec![CtorSig {
                span: None,
                name: "Pack".to_owned(),
                type_params: TypeParams::mk(&[("A", Polarity::Data)]),
                args: TypingContext::default(),
            }],
        };
        let mut symbol_table = symbol_table::SymbolTable::default();
        data.build(&mut symbol_table).unwrap();
        assert!(data.check(&symbol_table).is_err());
    }
}
