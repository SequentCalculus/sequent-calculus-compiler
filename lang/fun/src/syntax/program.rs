//! This module defines programs in Core.

use printer::*;
use std::collections::HashSet;

use crate::syntax::*;
use crate::typing::*;

/// This struct defines a module consisting of a list of [`Declaration`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

/// This struct defines a typechecked module created from a [`Program`] by checking each contained
/// [`Declaration`]. The checked module only contans monomorphic instances of data and codata types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProgram {
    /// Checked data type instances
    pub data_types: Vec<Data>,
    /// Checked codata type instances
    pub codata_types: Vec<Codata>,
    /// Checked top-level functions
    pub defs: Vec<Def>,
}

impl Program {
    /// This function typechecks all declarations in a module, creating a checked module with
    /// polymorphic type instances.
    pub fn check(self) -> Result<CheckedProgram, Error> {
        let symbol_table = build_symbol_table(&self)?;
        self.check_with_table_poly(symbol_table)
    }

    fn check_with_table_poly(self, mut symbol_table: SymbolTable) -> Result<CheckedProgram, Error> {
        let mut defs = Vec::new();
        let mut data_types = Vec::new();
        let mut codata_types = Vec::new();

        // we check the well-formedness of type declarations first
        for decl in self.declarations {
            match decl {
                Declaration::Data(data) => {
                    data.check(&symbol_table)?;

                    data_types.push(data);
                }
                Declaration::Codata(codata) => {
                    codata.check(&symbol_table)?;
                    codata_types.push(codata);
                }
                Declaration::Def(def) => {
                    defs.push(def);
                }
            }
        }

        let defs = defs
            .into_iter()
            .map(|def| def.check(&mut symbol_table))
            .collect::<Result<_, Error>>()?;

        // collect all uninstantiated type names from the symbol table, which are exactly those
        // which are actually instantiated somewhere in the term-level program
        let mut used_types: HashSet<Name> = symbol_table
            .types
            .keys()
            .map(|name| name.split_once("[").map_or(name.as_str(), |x| x.0).to_string())
            .collect();

        // A used type's constructors/destructors may reference other types that are never
        // explicitly instantiated on their own, e.g. an argument that is never used in any
        // clause body. Close `used_types` under all types referenced this way, since dropping
        // such a type would leave a dangling reference in the (still used) type that needs it.
        loop {
            let mut changed = false;
            for data in &data_types {
                if used_types.contains(&data.name) {
                    for name in data.referenced_types() {
                        changed |= used_types.insert(name);
                    }
                }
            }
            for codata in &codata_types {
                if used_types.contains(&codata.name) {
                    for name in codata.referenced_types() {
                        changed |= used_types.insert(name);
                    }
                }
            }
            if !changed {
                break;
            }
        }

        // filter out all unused type templates
        let checked = CheckedProgram {
            data_types: data_types
                .into_iter()
                .filter(|data| used_types.contains(&data.name))
                .collect(),
            codata_types: codata_types
                .into_iter()
                .filter(|codata| used_types.contains(&codata.name))
                .collect(),
            defs,
        };

        Ok(checked)
    }

    /// This function returns the names of all data type templates in a module.
    pub fn data_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for declaration in &self.declarations {
            if let Declaration::Data(data) = declaration {
                names.insert(data.name.clone());
            }
        }

        names
    }

    /// This function returns the names of all codata type templates in a module.
    pub fn codata_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for declaration in &self.declarations {
            if let Declaration::Codata(codata) = declaration {
                names.insert(codata.name.clone());
            }
        }
        names
    }
}

impl Print for Program {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        // We usually separate declarations with an empty line, except when the `omit_decl_sep`
        // option is set. This is useful for typesetting examples in papers which have to make
        // economic use of vertical space.
        let sep = if cfg.omit_decl_sep {
            alloc.line()
        } else {
            alloc.line().append(alloc.line())
        };

        let declarations = self.declarations.iter().map(|decl| decl.print(cfg, alloc));

        alloc.intersperse(declarations, sep)
    }
}

#[cfg(test)]
mod program_tests {
    use printer::Print;

    use crate::{
        parser::fun,
        syntax::{
            Chirality, CtorSig, DtorSig, Polarity, TypeArgs, TypeParams,
            context::{ContextBinding, TypingContext},
            declarations::{Codata, Data, Def},
            program::Program,
            terms::{Lit, Term},
            types::Ty,
            util::dummy_span,
        },
        typing::Error,
    };
    use std::collections::HashSet;

    // Program with one definition without arguments
    //
    //

    fn example_simple() -> Program {
        Program {
            declarations: vec![
                Def {
                    span: dummy_span(),
                    name: "x".to_string(),
                    type_params: TypeParams::default(),
                    context: TypingContext::default(),
                    body: Term::Lit(Lit::mk(4)),
                    ret_ty: Ty::mk_i64(),
                }
                .into(),
            ],
        }
    }

    fn existential_data() -> Program {
        Program {
            declarations: vec![
                Data {
                    span: Some(dummy_span()),
                    name: "Ex".to_owned(),
                    type_params: TypeParams::mk(&[("A", Polarity::Data)]),
                    ctors: vec![CtorSig {
                        span: Some(dummy_span()),
                        name: "Mk".to_owned(),
                        type_params: TypeParams::mk(&[("B", Polarity::Data)]),
                        args: TypingContext {
                            span: Some(dummy_span()),
                            bindings: vec![ContextBinding {
                                var: "x".to_owned(),
                                chi: Chirality::Prd,
                                ty: Ty::mk_decl("B", TypeArgs::default()),
                            }],
                        },
                    }]
                    .into(),
                }
                .into(),
            ],
        }
    }

    fn existential_codata() -> Program {
        Program {
            declarations: vec![
                Codata {
                    span: Some(dummy_span()),
                    name: "Ex".to_owned(),
                    type_params: TypeParams::mk(&[("A", Polarity::Data)]),
                    dtors: vec![DtorSig {
                        span: Some(dummy_span()),
                        name: "unmk".to_owned(),
                        type_params: TypeParams::mk(&[("B", Polarity::Data)]),
                        args: TypingContext {
                            span: Some(dummy_span()),
                            bindings: vec![ContextBinding {
                                var: "x".to_owned(),
                                chi: Chirality::Prd,
                                ty: Ty::mk_decl("B", TypeArgs::default()),
                            }],
                        },
                        cont_ty: Ty::mk_decl("B", TypeArgs::default()),
                    }]
                    .into(),
                }
                .into(),
            ],
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            example_simple().print_to_string(Default::default()),
            "def x(): i64 {\n    4\n}".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def x: i64 { 4 }"),
            Ok(example_simple().into())
        );
    }

    #[test]
    fn data_simple() {
        let result = example_simple().data_types();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn codata_simple() {
        let result = example_simple().codata_types();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    // Program with one definition which takes arguments
    //
    //

    fn example_args() -> Program {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_covar("a", Ty::mk_i64());
        Program {
            declarations: vec![
                Def {
                    span: dummy_span(),
                    name: "f".to_string(),
                    type_params: TypeParams::default(),
                    context: ctx,
                    body: Term::Lit(Lit::mk(4)),
                    ret_ty: Ty::mk_i64(),
                }
                .into(),
            ],
        }
    }

    #[test]
    fn display_args() {
        assert_eq!(
            example_args().print_to_string(Default::default()),
            "def f(x: i64, a: cns i64): i64 {\n    4\n}".to_string(),
        )
    }

    #[test]
    fn parse_args() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(x: i64, a:cns i64): i64 {\n    4\n}"),
            Ok(example_args().into())
        )
    }

    // Program with two definitions
    //
    //

    fn example_two() -> Program {
        let d1 = Def {
            span: dummy_span(),
            name: "f".to_string(),
            type_params: TypeParams::default(),
            context: TypingContext::default(),
            body: Term::Lit(Lit::mk(2)),
            ret_ty: Ty::mk_i64(),
        };

        let d2 = Def {
            span: dummy_span(),
            name: "g".to_string(),
            type_params: TypeParams::default(),
            context: TypingContext::default(),
            body: Term::Lit(Lit::mk(4)),
            ret_ty: Ty::mk_i64(),
        };
        Program {
            declarations: vec![d1.into(), d2.into()],
        }
    }

    #[test]
    fn display_two() {
        assert_eq!(
            example_two().print_to_string(Default::default()),
            "def f(): i64 {\n    2\n}\n\ndef g(): i64 {\n    4\n}".to_string(),
        )
    }

    #[test]
    fn parse_two() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(): i64 { 2 }\n def g(): i64 { 4 }"),
            Ok(example_two().into())
        )
    }

    #[test]
    fn parse_existential_data() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("data Ex[A+] { Mk[B+](x: B) }"),
            Ok(existential_data())
        );
    }

    #[test]
    fn parse_existential_codata() {
        let parser = fun::ProgParser::new();

        assert_eq!(
            parser.parse("codata Ex[A+] { unmk[B+](x: B): B }"),
            Ok(existential_codata())
        );
    }

    #[test]
    fn display_existential_data() {
        assert_eq!(
            existential_data().print_to_string(Default::default()),
            "data Ex[A+] { Mk[B+](x: B) }".to_string()
        )
    }

    #[test]
    fn display_existential_codata() {
        assert_eq!(
            existential_codata().print_to_string(Default::default()),
            "codata Ex[A+] { unmk[B+](x: B): B }".to_string()
        )
    }

    #[test]
    fn def_body_respects_declared_polarity_of_own_type_param() {
        let parser = fun::ProgParser::new();
        let result = parser
            .parse("def wrap[B+](x: B): B { x }\ndef f[A-](x: A): A { wrap[A](x) }")
            .unwrap()
            .check();
        assert!(
            matches!(result, Err(Error::PolarityMismatch { .. })),
            "expected a PolarityMismatch since f's own A- is used where wrap expects B+, got {result:?}"
        );
        // Sanity-check the Display impl (used for diagnostics) doesn't panic and mentions the
        // mismatching polarities.
        let message = result.unwrap_err().to_string();
        assert!(message.contains("Polarity mismatch"));
    }

    #[test]
    fn def_body_accepts_matching_declared_polarity_of_own_type_param() {
        let parser = fun::ProgParser::new();
        let result = parser
            .parse("def wrap[B+](x: B): B { x }\ndef f[A+](x: A): A { wrap[A](x) }")
            .unwrap()
            .check();
        assert!(result.is_ok(), "expected Ok, got {result:?}");
    }
}
