//! This module defines programs in Core.

use printer::*;

use crate::{
    mono::{
        constraints::{ConstraintCollector, FlowConstraintSet},
        errors::Error,
    },
    syntax::*,
    typing::{check::Checked, errors::LocatedTypeError},
};

/// This struct defines programs in Core. They consist of a list top-level functions, a list of
/// user-declared data types, and a list of user-declared codata types. Moreover, it contains the
/// highest [`ID`] currently used for [`Identifier`]s in the program. The type parameter `D`
/// determines whether the program is in the full language (if `D` is instantiated with [`Def`],
/// which is the default) or in the focused fragment (if `D` is instantiated with [`FsDef`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prog<D = Def> {
    /// The top-level definitions of the program, either unfocused ([`Def`]) or focused ([`FsDef`])
    pub defs: Vec<D>,
    /// The data types of the program
    pub data_types: Vec<DataDeclaration>,
    /// The codata types of the program
    pub codata_types: Vec<CodataDeclaration>,
    /// Highest [`ID`] currently used for [`Identifier`]s in the program
    pub max_id: ID,
    /// This field is used to determine whether the program contains monomorphic type instances
    pub is_mono: bool,
}

pub type FsProg = Prog<FsDef>;

impl Prog {
    /// This function applies the focusing transformation to a program. As a preprocessing step, it
    /// makes all binders in the program unique.
    pub fn focus(mut self) -> FsProg {
        self.uniquify();
        let mut max_id = self.max_id;
        let mut new_defs = Vec::with_capacity(self.defs.len());
        for def in self.defs {
            new_defs.push(def.focus(&mut max_id));
        }
        FsProg {
            defs: new_defs,
            data_types: self.data_types,
            codata_types: self.codata_types,
            max_id,
            is_mono: self.is_mono,
        }
    }

    /// This function makes all binders in the program unique.
    pub fn uniquify(&mut self) {
        let new_defs = Vec::with_capacity(self.defs.len());
        for mut def in std::mem::replace(&mut self.defs, new_defs) {
            def = def.uniquify(&mut self.max_id);
            self.defs.push(def);
        }
    }
}

impl<D: Print> Print for Prog<D> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> printer::Builder<'a> {
        // We usually separate declarations with an empty line, except when the `omit_decl_sep`
        // option is set. This is useful for typesetting examples in papers which have to make
        // economic use of vertical space.
        let sep = if cfg.omit_decl_sep {
            alloc.line()
        } else {
            alloc.line().append(alloc.line())
        };

        let defs = self.defs.iter().map(|def| def.print(cfg, alloc));
        let data_types = self.data_types.iter().map(|typ| typ.print(cfg, alloc));
        let codata_types = self.codata_types.iter().map(|typ| typ.print(cfg, alloc));

        alloc
            .intersperse(data_types, alloc.line())
            .append(alloc.line())
            .append(alloc.intersperse(codata_types, alloc.line()))
            .append(sep.clone())
            .append(alloc.intersperse(defs, sep))
    }
}

impl ConstraintCollector for Prog {
    fn collect_constraints(
        &self,
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
    ) -> Result<FlowConstraintSet, Error> {
        // collect all type parameters from the data and codata declarations in the program
        let type_params: Vec<Identifier> = self
            .data_types
            .iter()
            .flat_map(|data| data.type_params.clone())
            .chain(
                self.codata_types
                    .iter()
                    .flat_map(|codata| codata.type_params.clone()),
            )
            .collect();

        // type check the program before collecting constraints, to ensure that all type annotations in the program are well-formed
        self.check(
            &type_params,
            data_declarations,
            codata_declarations,
            &self.defs,
        )
        .unwrap();

        let mut constraints = FlowConstraintSet::new();

        for def in &self.defs {
            constraints.extend(def.collect_constraints(data_declarations, codata_declarations)?);
        }
        Ok(constraints)
    }
}

impl Checked for Prog {
    fn check(
        &self,
        type_params: &[Identifier],
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
        defs: &[Def],
    ) -> Result<(), LocatedTypeError> {
        for data in &self.data_types {
            data.check(
                &data.type_params,
                data_declarations,
                codata_declarations,
                defs,
            )?;
        }
        for codata in &self.codata_types {
            codata.check(
                &codata.type_params,
                data_declarations,
                codata_declarations,
                defs,
            )?;
        }
        for def in &self.defs {
            def.check(type_params, data_declarations, codata_declarations, defs)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod program_tests {

    use std::collections::HashSet;

    use crate::mono::constraints::{ConstraintCollector, FlowConstraint, FlowConstraintSet};
    use crate::syntax::*;
    use crate::typing::check::Checked;
    extern crate self as core_lang;
    use core_macros::{
        bind, cns, covar, ctor_sig, cut, data, def, exit, fs_cut, fs_def, id, lit, prd, prog, tvar,
        ty, var,
    };

    fn example_def2_var() -> FsDef {
        fs_def!(
            id!("cut"),
            [bind!(id!("x", 1), prd!()), bind!(id!("a", 2), cns!())],
            fs_cut!(var!(id!("x", 1)), covar!(id!("a", 2))),
        )
    }

    #[test]
    fn transform_prog2() {
        let prog = prog!(
            [def!(
                id!("cut"),
                [bind!(id!("x"), prd!()), bind!(id!("a"), cns!())],
                cut!(var!(id!("x")), covar!(id!("a"))),
            )],
            [],
            []
        );
        let result = prog.focus();

        let expected = prog!([example_def2_var()], [], [], 2);
        assert_eq!(result, expected)
    }

    #[test]
    fn collect_constraints_prog() {
        let list = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), []),
                ctor_sig!(
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))]))
                    ]
                )
            ],
            [id!("A", 1)]
        );

        let prog = prog!(
            [def!(
                id!("main"),
                [],
                exit!(lit!(1), ty!(id!("List"), [ty!("int")]))
            )],
            [list],
            []
        );

        let constraints = prog
            .collect_constraints(&prog.data_types, &prog.codata_types)
            .unwrap();

        let expected = FlowConstraintSet {
            constraints: HashSet::from_iter(vec![FlowConstraint {
                from: Ty::I64,
                to: Ty::Var(Identifier {
                    name: "A".to_string(),
                    id: 1,
                }),
            }]),
        };

        assert_eq!(constraints, expected)
    }

    #[test]
    fn check_undeclared_type_in_prog() {
        // program uses an undeclared type in a top-level definition
        let prog = prog!(
            [def!(
                id!("main"),
                [],
                exit!(lit!(1), ty!(id!("NonExistent")))
            )],
            [],
            []
        );

        let type_params: Vec<Identifier> = prog
            .data_types
            .iter()
            .flat_map(|data| data.type_params.clone())
            .chain(
                prog.codata_types
                    .iter()
                    .flat_map(|codata| codata.type_params.clone()),
            )
            .collect();

        assert!(
            prog.check(
                &type_params,
                &prog.data_types,
                &prog.codata_types,
                &prog.defs
            )
            .is_err(),
            "expected error for undeclared type in program"
        );
    }

    #[test]
    fn check_declared_type_annotation_ok() {
        // declared type exists and is used as an annotation on an exit statement
        let list = data!(id!("List"), [ctor_sig!(id!("Nil"), [])], []);

        let prog = prog!(
            [def!(id!("main"), [], exit!(lit!(1), ty!(id!("List"))))],
            [list],
            []
        );

        let type_params: Vec<Identifier> = prog
            .data_types
            .iter()
            .flat_map(|data| data.type_params.clone())
            .chain(
                prog.codata_types
                    .iter()
                    .flat_map(|codata| codata.type_params.clone()),
            )
            .collect();

        assert!(
            prog.check(
                &type_params,
                &prog.data_types,
                &prog.codata_types,
                &prog.defs
            )
            .is_ok(),
            "expected declared type annotation to be accepted"
        );
    }

    #[test]
    fn check_type_arity_mismatch_in_prog() {
        // data declaration with one type parameter but used without arguments in a def
        let list = data!(id!("List"), [ctor_sig!(id!("Nil"), [])], [id!("A", 1)]);

        let prog = prog!(
            [def!(id!("main"), [], exit!(lit!(1), ty!(id!("List"))))],
            [list],
            []
        );

        let type_params: Vec<Identifier> = prog
            .data_types
            .iter()
            .flat_map(|data| data.type_params.clone())
            .chain(
                prog.codata_types
                    .iter()
                    .flat_map(|codata| codata.type_params.clone()),
            )
            .collect();

        assert!(
            prog.check(
                &type_params,
                &prog.data_types,
                &prog.codata_types,
                &prog.defs
            )
            .is_err(),
            "expected arity mismatch for type application in program"
        );
    }
}
