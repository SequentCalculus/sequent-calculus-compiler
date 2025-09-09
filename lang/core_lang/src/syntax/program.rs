//! This module defines programs in Core.

use printer::*;

use crate::syntax::*;
use crate::traits::*;

/// This struct defines programs in Core. They consist of a list top-level functions, a list of
/// user-declared data types, and a list of user-declared codata types. The program is in the full
/// language if the type parameter `T` is instantiated with [`Def`] or in the focused fragment if the
/// type parameter `T` is instantiated with [`FsDef`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XProg<T> {
    /// The top-level definitions of the program, either unfocused ([`Def`]) or focused ([`FsDef`])
    pub defs: Vec<T>,
    /// The data types of the program
    pub data_types: Vec<DataDeclaration>,
    /// The codata types of the program
    pub codata_types: Vec<CodataDeclaration>,
}

/// Type alias for unfocused programs
pub type Prog = XProg<Def>;
/// Type alias for focused programs
pub type FsProg = XProg<FsDef>;

impl<T: Print> Print for XProg<T> {
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
                        .uniquify(&mut def.context.vars(), &mut def.used_vars);
                    def.focus()
                })
                .collect(),
            data_types: self.data_types,
            codata_types: self.codata_types,
        }
    }
}

#[cfg(test)]
mod program_tests {
    use crate::syntax::*;
    use std::collections::HashSet;

    fn example_def2_var() -> FsDef {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::I64);
        ctx.add_covar("a", Ty::I64);
        FsDef {
            name: "cut".to_string(),
            context: ctx,
            body: FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            used_vars: HashSet::from(["a".to_string(), "x".to_string()]),
        }
    }

    #[test]
    fn transform_prog2() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::I64);
        ctx.add_covar("a", Ty::I64);
        let prog = Prog {
            defs: vec![
                Def {
                    name: "cut".to_string(),
                    context: ctx,
                    body: Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
                        .into(),
                    used_vars: HashSet::from(["a".to_string(), "x".to_string()]),
                }
                .into(),
            ],
            data_types: vec![],
            codata_types: vec![],
        };
        let result = prog.focus();

        let expected = FsProg {
            defs: vec![example_def2_var()],
            data_types: vec![],
            codata_types: vec![],
        };
        assert_eq!(result, expected)
    }
}
