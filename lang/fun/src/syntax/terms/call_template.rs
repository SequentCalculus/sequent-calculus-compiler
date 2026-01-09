//! This module defines the call of a top-level function in Fun.

use codespan::Span;
use derivative::Derivative;
use printer::*;
use std::collections::HashMap;

use crate::parser::util::ToMiette;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

use std::collections::HashSet;

/// This struct defines the call of a top-level function in Fun. It consists of the name of the
/// top-level function to call, the arguments, and after typechecking also the inferred type.
///
/// Example:
/// `fac(10)`, calls the top-level function `fac` with argument `10`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct CallTemplate {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The name of the top-level function being called
    pub name: Name,
    /// The type arguments instantiating the type parameters of the type
    pub type_args: TypeArgs,
    /// The arguments
    pub args: Arguments,
    /// The (inferred) return type
    pub ret_ty: Option<Ty>,
}

impl CallTemplate {
    pub fn subst_ty(self, mappings: &HashMap<Name, Ty>) -> Self {
        todo!()
    }
}

impl OptTyped for CallTemplate {
    fn get_type(&self) -> Option<Ty> {
        self.ret_ty.clone()
    }
}

impl Print for CallTemplate {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name
            .print(cfg, alloc)
            .append(self.type_args.print(cfg, alloc))
            .append(self.args.print(cfg, alloc).parens().group())
    }
}

impl From<CallTemplate> for Term {
    fn from(value: CallTemplate) -> Self {
        Term::CallTemplate(value)
    }
}

impl Check for CallTemplate {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let def_template = symbol_table
            .def_templates
            .get(&self.name)
            .ok_or(Error::Undefined {
                span: self.span.to_miette(),
                name: self.name.clone(),
            })?;
        let def_instantiated = def_template.instantiate(self.span, self.type_args.args.clone())?;
        let def_checked = def_instantiated.check(symbol_table)?;

        let (def_context, def_ty) = (def_checked.context.clone(), def_checked.ret_ty.clone());
        symbol_table.defs.insert(
            def_checked.name.clone(),
            (def_context.clone(), def_ty.clone()),
        );
        symbol_table.instantiated_defs.push(def_checked);

        check_equality(&self.span, symbol_table, expected, &def_ty)?;

        self.args = check_args(
            &self.span.to_miette(),
            symbol_table,
            context,
            self.args,
            &def_context,
        )?;

        self.ret_ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for CallTemplate {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.args.entries.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use codespan::Span;
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::*;

    #[test]
    fn check_mult() {
        let mut symbol_table = symbol_table_list();
        let mut ctx = TypingContext::default();
        ctx.add_var("l", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        symbol_table
            .defs
            .insert("mult".to_owned(), (ctx.clone(), Ty::mk_i64()));
        let result = def_mult()
            .body
            .check(&mut symbol_table, &ctx, &Ty::mk_i64())
            .unwrap();
        let expected = def_mult_typed().body;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_call_fail() {
        let result = CallTemplate {
            span: Span::default(),
            name: "main".to_owned(),
            type_args: TypeArgs {
                span: Span::default(),
                args: Vec::new(),
            },
            args: vec![].into(),
            ret_ty: None,
        }
        .check(
            &mut SymbolTable::default(),
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            &Ty::mk_i64(),
        );
        assert!(result.is_err())
    }

    fn example_simple() -> CallTemplate {
        CallTemplate {
            span: Span::default(),
            name: "foo".to_string(),
            type_args: TypeArgs {
                span: Span::default(),
                args: Vec::new(),
            },
            args: vec![].into(),
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

    fn example_extended() -> CallTemplate {
        CallTemplate {
            span: Span::default(),
            name: "foo".to_string(),
            type_args: TypeArgs {
                span: Span::default(),
                args: Vec::new(),
            },
            args: vec![Term::Lit(Lit::mk(2)).into(), XVar::mk("a").into()].into(),
            ret_ty: None,
        }
    }

    #[test]
    fn display_extended() {
        assert_eq!(
            example_extended().print_to_string(Default::default()),
            "foo(2, a)"
        )
    }

    #[test]
    fn parse_extended() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo(2, a)"), Ok(example_extended().into()));
    }
}
