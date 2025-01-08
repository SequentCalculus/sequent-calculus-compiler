use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print};

use super::Term;
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        substitution::Substitution,
        types::{OptTyped, Ty},
        Name, XVar,
    },
    traits::UsedBinders,
    typing::{
        check::{check_args, check_equality, Check},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

use std::collections::HashSet;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Fun {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub args: Substitution,
    pub ret_ty: Option<Ty>,
}

impl OptTyped for Fun {
    fn get_type(&self) -> Option<Ty> {
        self.ret_ty.clone()
    }
}

impl Print for Fun {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(self.name.clone())
            .append(self.args.print(cfg, alloc).parens())
    }
}

impl From<Fun> for Term {
    fn from(value: Fun) -> Self {
        Term::Fun(value)
    }
}

impl Check for Fun {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        match symbol_table.funs.get(&self.name) {
            Some((types, ret_ty)) => {
                check_equality(&self.span.to_miette(), expected, ret_ty)?;
                let new_args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    types,
                )?;
                Ok(Fun {
                    args: new_args,
                    ret_ty: Some(expected.clone()),
                    ..self
                })
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.name.clone(),
            }),
        }
    }
}

impl UsedBinders for Fun {
    fn used_binders(&self, used: &mut HashSet<XVar>) {
        self.args.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use super::{Check, Fun, Term};
    use crate::{
        parser::fun,
        syntax::{
            context::TypingContext, substitution::SubstitutionBinding, terms::Lit, types::Ty,
        },
        test_common::{def_mult, def_mult_typed, symbol_table_list},
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;

    #[test]
    fn check_mult() {
        let mut symbol_table = symbol_table_list();
        let mut ctx = TypingContext::default();
        ctx.add_var("l", Ty::mk_decl("ListInt"));
        symbol_table
            .funs
            .insert("mult".to_owned(), (ctx.clone(), Ty::mk_i64()));
        let result = def_mult()
            .body
            .check(&symbol_table, &ctx, &Ty::mk_i64())
            .unwrap();
        let expected = def_mult_typed().body;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fun_fail() {
        let result = Fun {
            span: Span::default(),
            name: "main".to_owned(),
            args: vec![],
            ret_ty: None,
        }
        .check(
            &SymbolTable::default(),
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            &Ty::mk_i64(),
        );
        assert!(result.is_err())
    }

    fn example_simple() -> Fun {
        Fun {
            span: Span::default(),
            name: "foo".to_string(),
            args: vec![],
            ret_ty: None,
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            example_simple().print_to_string(Default::default()),
            "foo()"
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo()"), Ok(example_simple().into()));
    }

    fn example_extended() -> Fun {
        Fun {
            span: Span::default(),
            name: "foo".to_string(),
            args: vec![
                Term::Lit(Lit::mk(2)).into(),
                SubstitutionBinding::CovarBinding {
                    covar: "a".to_string(),
                    ty: None,
                },
            ],
            ret_ty: None,
        }
    }

    #[test]
    fn display_extended() {
        assert_eq!(
            example_extended().print_to_string(Default::default()),
            "foo(2, 'a)"
        )
    }

    #[test]
    fn parse_extended() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo(2, 'a)"), Ok(example_extended().into()));
    }
}
