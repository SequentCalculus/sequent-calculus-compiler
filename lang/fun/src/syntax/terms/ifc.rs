//! This module defines the conditionals comparing two integers in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{ELSE, EQQ, GT, GTE, IF, LT, LTE, NEQ, ZERO};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::{Constraint, ConstraintBank, Inference};
use crate::typing::*;

use std::collections::HashMap;
use std::{collections::HashSet, rc::Rc};

/// This enum encodes the comparison operation used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfSort {
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `<`
    Less,
    /// `<=`
    LessOrEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterOrEqual,
}

impl Print for IfSort {
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            IfSort::Equal => alloc.text(EQQ),
            IfSort::NotEqual => alloc.text(NEQ),
            IfSort::Less => alloc.text(LT),
            IfSort::LessOrEqual => alloc.text(LTE),
            IfSort::Greater => alloc.text(GT),
            IfSort::GreaterOrEqual => alloc.text(GTE),
        }
    }
}

/// This struct defines the conditionals comparing either two terms or one term to zero in Fun. It
/// consists of the comparison operation, the first term and an optional second term, and the
/// then-branch and else-branch, and after typechecking also of the inferred type.
///
/// Example:
/// ```text
/// if n == 0 { 1 } else { n * fac(n - 1) }
/// ```
/// If `n` is `0` return `1` else calculate `n * fac(n - 1)`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct IfC {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The comparison operation
    pub sort: IfSort,
    /// The first term of the comparison
    pub fst: Rc<Term>,
    /// The optional second term of the comparison
    pub snd: Option<Rc<Term>>,
    /// The then-branch
    pub thenc: Rc<Term>,
    /// The else-branch
    pub elsec: Rc<Term>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
}

impl OptTyped for IfC {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for IfC {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let snd = match self.snd {
            None => alloc.text(ZERO),
            Some(ref snd) => snd.print(cfg, alloc),
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(self.sort.print(cfg, alloc))
            .append(alloc.space())
            .append(snd)
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.thenc.print(cfg, alloc).group())
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
            .append(alloc.space())
            .append(alloc.keyword(ELSE))
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.elsec.print(cfg, alloc).group())
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
    }
}

impl From<IfC> for Term {
    fn from(value: IfC) -> Self {
        Term::IfC(value)
    }
}

impl Inference for IfC {
    fn gather_constraints(
            &mut self,
            constraint_bank: &mut ConstraintBank,
            context: &TypingContext,
            ty_var: Ty
        ) -> Result<(), Error> {
        // adding a new type var as the type of the term for easier lookup after unification
        let new_type_var = constraint_bank.var_name_generator.get_new_ty_var();
        self.ty = Some(new_type_var.clone());
        constraint_bank.constraints.push(Constraint::mk_only_ty(new_type_var, ty_var.clone()));

        self.fst.gather_constraints(constraint_bank, context, Ty::mk_i64())?;
        self.snd.gather_constraints(constraint_bank, context, Ty::mk_i64())?;

        self.thenc.gather_constraints(constraint_bank, context, ty_var.clone())?;
        self.elsec.gather_constraints(constraint_bank, context, ty_var)?;

        Ok(())
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>
    ) -> Result<(), Error> {
        self.fst.insert_inferred_type(mappings, symbol_table, choices)?;
        self.snd.insert_inferred_type(mappings, symbol_table, choices)?;

        self.thenc.insert_inferred_type(mappings, symbol_table, choices)?;
        self.elsec.insert_inferred_type(mappings, symbol_table, choices)?;

        match &mut self.ty {
            Some(ty_var) => {
                ty_var.mut_subst_ty(mappings);
                ty_var.check(&Some(self.span), symbol_table)
            },
            None => panic!("The Type of the term {:?} is not set after type inference", self)
        }
    }
}

impl UsedBinders for IfC {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.fst.used_binders(used);
        self.snd.used_binders(used);
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::typing::inference::{Constraint, ConstraintBank, Inference};

    use std::rc::Rc;

    #[test]
    fn inference_ife() {
        let mut term = IfC {
            span: dummy_span(),
            sort: IfSort::Equal,
            fst: Rc::new(Lit::mk(2).into()),
            snd: Some(Rc::new(Lit::mk(1).into())),
            thenc: Rc::new(Lit::mk(2).into()),
            elsec: Rc::new(Lit::mk(3).into()),
            ty: None,
        };

        let mut constraint_bank = ConstraintBank{
            symbol_table: Default::default(),
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        term.gather_constraints(&mut constraint_bank, &TypingContext::default(), Ty::mk_ty_var("x")).unwrap();

        let expected = vec![
            Constraint::mk_only_ty(Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),
            Constraint::mk_only_ty(Ty::mk_i64(), Ty::mk_i64()),
            Constraint::mk_only_ty(Ty::mk_i64(), Ty::mk_i64()),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64()),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64())
        ];

        let ConstraintBank { constraints: result, .. } = constraint_bank;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("0")));
        
    }

    fn example() -> IfC {
        IfC {
            span: dummy_span(),
            sort: IfSort::Equal,
            fst: Rc::new(Term::Lit(Lit::mk(1))),
            snd: Some(Rc::new(Term::Lit(Lit::mk(1)))),
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "if 1 == 1 {\n    2\n} else {\n    4\n}"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("if 1 == 1 {2 } else { 4}"),
            Ok(example().into())
        );
    }

    fn example_zero() -> IfC {
        IfC {
            span: dummy_span(),
            sort: IfSort::Equal,
            fst: Rc::new(Term::Lit(Lit::mk(0))),
            snd: None,
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    fn example_zero_not() -> IfC {
        IfC {
            span: dummy_span(),
            sort: IfSort::NotEqual,
            fst: Rc::new(Term::Lit(Lit::mk(1))),
            snd: None,
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display_zero() {
        assert_eq!(
            example_zero().print_to_string(Default::default()),
            "if 0 == 0 {\n    2\n} else {\n    4\n}"
        )
    }

    #[test]
    fn display_zero_not() {
        assert_eq!(
            example_zero_not().print_to_string(Default::default()),
            "if 1 != 0 {\n    2\n} else {\n    4\n}"
        )
    }

    #[test]
    fn parse_zero() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("if 0 == 0 { 2} else {4 }"),
            Ok(example_zero().into())
        );
    }

    #[test]
    fn parse_zero_not() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("if 1 != 0 { 2} else {4 }"),
            Ok(example_zero_not().into())
        );
    }
}
