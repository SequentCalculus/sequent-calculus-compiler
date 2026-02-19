//! This module defines programs in Core.

use printer::*;

use crate::syntax::*;
use crate::traits::*;

/// This struct defines programs in Core. They consist of a list top-level functions, a list of
/// user-declared data types, and a list of user-declared codata types. The type parameter `D`
/// determines whether the program is in the full language (if `D` is instantiated with [`Def`]) or
/// in the focused fragment (if `D` is instantiated with [`FsDef`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prog<D = Def> {
    /// The top-level definitions of the program, either unfocused ([`Def`]) or focused ([`FsDef`])
    pub defs: Vec<D>,
    /// The data types of the program
    pub data_types: Vec<DataDeclaration>,
    /// The codata types of the program
    pub codata_types: Vec<CodataDeclaration>,
}

pub type FsProg = Prog<FsDef>;

impl Prog {
    /// This function applies the focusing transformation to a program. As a preprocessing step it
    /// makes all binders in each path through a top-level function unique.
    pub fn focus(self) -> FsProg {
        FsProg {
            defs: self
                .defs
                .into_iter()
                .map(|mut def| {
                    def.body = def
                        .body
                        .uniquify(&mut UniquifyState::new(def.context.vars()));
                    def.focus()
                })
                .collect(),
            data_types: self.data_types,
            codata_types: self.codata_types,
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

#[cfg(test)]
mod program_tests {
    use crate::syntax::*;
    extern crate self as core_lang;
    use core_macros::{bind, cns, covar, cut, def, fs_cut, fs_def, id, prd, prog, var};

    fn example_def2_var() -> FsDef {
        fs_def!(
            id!("cut"),
            [bind!(id!("x"), prd!()), bind!(id!("a"), cns!())],
            fs_cut!(var!(id!("x")), covar!(id!("a"))),
            [id!("a"), id!("x")]
        )
    }

    #[test]
    fn transform_prog2() {
        let prog = prog!(
            [def!(
                id!("cut"),
                [bind!(id!("x"), prd!()), bind!(id!("a"), cns!())],
                cut!(var!(id!("x")), covar!(id!("a"))),
                [id!("a"), id!("x")]
            )],
            [],
            []
        );
        let result = prog.focus();

        let expected = prog!([example_def2_var()], [], []);
        assert_eq!(result, expected)
    }
}
