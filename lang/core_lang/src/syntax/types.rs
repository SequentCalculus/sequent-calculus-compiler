//! This module defines types in Core.

use printer::tokens::I64;
use printer::*;

use crate::mono::constraints::{ConstraintCollector, FlowConstraintSet, collect_type_flow};
use crate::mono::errors::Error;
use crate::syntax::declaration::lookup_type_declaration;
use crate::syntax::*;

/// This enum encodes the types of AxCut. They are either integers or names of user-declared types.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum Ty {
    /// Signed 64-Bit integer.
    I64,
    /// User-declared data or codata type with additional type arguments.
    Decl {
        name: Identifier,
        type_args: TypeArgs,
    },
    /// Type variable, used for type parameters.
    Var(Identifier),
}

impl Ty {
    /// This function checks whether a type is a codata type.
    /// - `codata_types` is the list of codata type declarations in the program.
    pub fn is_codata(&self, codata_types: &[CodataDeclaration]) -> bool {
        match self {
            Ty::I64 => false,
            Ty::Decl { name, .. } => codata_types
                .iter()
                .any(|declaration| declaration.name == *name),
            Ty::Var(_) => false,
        }
    }
}

impl ConstraintCollector for Ty {
    fn collect_constraints(
        &self,
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
    ) -> Result<FlowConstraintSet, Error> {
        match self {
            Ty::I64 => Ok(FlowConstraintSet::new()),
            Ty::Var(_) => Ok(FlowConstraintSet::new()),
            Ty::Decl { name, type_args } => {
                if self.is_codata(codata_declarations) {
                    let template = lookup_type_declaration(&name, codata_declarations);
                    type_args.args.iter().zip(&template.type_params).try_fold(
                        FlowConstraintSet::new(),
                        |mut acc, (arg, param)| {
                            acc.extend(collect_type_flow(arg, &Ty::Var(param.clone()))?);
                            Ok(acc)
                        },
                    )
                } else {
                    let template = lookup_type_declaration(&name, data_declarations);
                    type_args.args.iter().zip(&template.type_params).try_fold(
                        FlowConstraintSet::new(),
                        |mut acc, (arg, param)| {
                            acc.extend(collect_type_flow(arg, &Ty::Var(param.clone()))?);
                            Ok(acc)
                        },
                    )
                }
            }
        }
    }
}

impl Print for Ty {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            Ty::I64 => alloc.typ(I64),
            Ty::Decl { name, type_args } => {
                alloc.typ(&name.name).append(type_args.print(cfg, alloc))
            }
            Ty::Var(name) => alloc.typ(&name.print_to_string(None)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct TypeArgs {
    /// The type arguments
    pub args: Vec<Ty>,
}

impl Print for TypeArgs {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        if self.args.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.args.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
                .brackets()
                .group()
        }
    }
}

#[cfg(test)]
mod type_tests {
    use super::{Identifier, Ty, TypeArgs};
    use printer::Print;

    #[test]
    fn display_i64() {
        assert_eq!(Ty::I64.print_to_string(None), "i64");
    }

    #[test]
    fn display_decl() {
        let ty = Ty::Decl {
            name: Identifier::new("List".to_string()),
            type_args: TypeArgs {
                args: vec![Ty::I64],
            },
        };
        assert_eq!(ty.print_to_string(None), "List[i64]");
    }

    #[test]
    fn display_decl2() {
        let ty = Ty::Decl {
            name: Identifier::new("List".to_string()),
            type_args: TypeArgs {
                args: vec![Ty::Var(Identifier {
                    name: "A".to_string(),
                    id: 1,
                })],
            },
        };
        assert_eq!(ty.print_to_string(None), "List[A_1]");
    }
}
