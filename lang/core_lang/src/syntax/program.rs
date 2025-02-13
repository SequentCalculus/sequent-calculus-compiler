use printer::{DocAllocator, Print};

use crate::traits::*;

use super::{
    declaration::{CodataDeclaration, DataDeclaration},
    def::FsDef,
    Def,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XProg<T> {
    pub defs: Vec<T>,
    pub data_types: Vec<DataDeclaration>,
    pub codata_types: Vec<CodataDeclaration>,
}

pub type Prog = XProg<Def>;
pub type FsProg = XProg<FsDef>;

impl<T: Print> Print for XProg<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        // We usually separate declarations with an empty line, except when the `omit_decl_sep` option is set.
        // This is useful for typesetting examples in papers which have to make economic use of vertical space.
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

#[must_use]
pub fn transform_prog(prog: Prog) -> FsProg {
    FsProg {
        defs: prog
            .defs
            .into_iter()
            .map(|mut def| {
                def.body = def
                    .body
                    .uniquify(&mut def.context.vars(), &mut def.used_vars);
                def.focus()
            })
            .collect(),
        data_types: prog.data_types,
        codata_types: prog.codata_types,
    }
}

#[cfg(test)]
mod program_tests {
    use super::{Def, Prog};
    use crate::syntax::{
        context::TypingContext,
        def::FsDef,
        program::{transform_prog, FsProg},
        statements::{Cut, FsCut},
        terms::XVar,
        types::Ty,
    };
    use std::collections::HashSet;

    fn example_def2_var() -> FsDef {
        let mut ctx = TypingContext::empty();
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
        let mut ctx = TypingContext::empty();
        ctx.add_var("x", Ty::I64);
        ctx.add_covar("a", Ty::I64);
        let prog = Prog {
            defs: vec![Def {
                name: "cut".to_string(),
                context: ctx,
                body: Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
                used_vars: HashSet::from(["a".to_string(), "x".to_string()]),
            }
            .into()],
            data_types: vec![],
            codata_types: vec![],
        };
        let result = transform_prog(prog);

        let expected = FsProg {
            defs: vec![example_def2_var()],
            data_types: vec![],
            codata_types: vec![],
        };
        assert_eq!(result, expected)
    }
}
