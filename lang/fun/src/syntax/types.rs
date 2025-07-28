//! Defines types in Fun
use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COMMA, I64},
    DocAllocator, Print,
};

use crate::{
    parser::util::ToMiette,
    syntax::{context::TypeContext, declarations::Polarity, Name},
    typing::{errors::Error, symbol_table::SymbolTable},
};

use std::collections::HashMap;

/// Types
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum Ty {
    /// Signed 64-bit integer.
    I64 {
        #[derivative(PartialEq = "ignore")]
        span: Span,
    },
    /// Declared data or codata type.
    Decl {
        #[derivative(PartialEq = "ignore")]
        span: Span,
        name: Name,
        type_args: TypeArgs,
    },
}

/// Monomorphises a type using given type arguments
/// Example: `data List[A] { Nil, Cons(A,List[A]) }`
/// `let x : List[i64] = Cons(1,Nil);`
/// `let y : List[Bool] = Cons(True,Nil);`
/// `List` is polymorphic in `A` and is here used with `A=i64` and `A=Bool`
/// This function will then create two monomorphic types
/// `data ListI64 { NilI,ConsI(i64,ListI64) }`
/// `data ListBool { NilB,ConsB(Bool,ListBool) }`
fn create_instance(
    span: Span,
    instance_name: String,
    type_args: &TypeArgs,
    pol: Polarity,
    type_params: TypeContext,
    xtors: Vec<Name>,
    symbol_table: &mut SymbolTable,
) -> Result<(), Error> {
    type_args.is_instance(&type_params, symbol_table)?;
    let mappings: HashMap<Name, Ty> = type_params
        .bindings
        .iter()
        .cloned()
        .zip(type_args.args.clone())
        .collect();

    let xtor_names: Vec<(Name, Name)> = xtors
        .clone()
        .into_iter()
        .zip(
            xtors
                .clone()
                .into_iter()
                .map(|xtor| xtor + &type_args.print_to_string(None)),
        )
        .collect();

    // insert xtor instances
    match pol {
        Polarity::Data => {
            for (base_name, full_name) in &xtor_names {
                let Some(args_template) = symbol_table.ctor_templates.get(base_name) else {
                    return Err(Error::Undefined {
                        span: span.to_miette(),
                        name: base_name.clone(),
                    });
                };
                symbol_table
                    .ctors
                    .insert(full_name.clone(), args_template.clone().subst_ty(&mappings));
            }
        }
        Polarity::Codata => {
            for (base_name, full_name) in &xtor_names {
                let Some((args_template, cont_ty_template)) =
                    symbol_table.dtor_templates.get(base_name)
                else {
                    return Err(Error::Undefined {
                        span: span.to_miette(),
                        name: base_name.clone(),
                    });
                };
                symbol_table.dtors.insert(
                    full_name.clone(),
                    (
                        args_template.clone().subst_ty(&mappings),
                        cont_ty_template.clone().subst_ty(&mappings),
                    ),
                );
            }
        }
    }

    symbol_table
        .types
        .insert(instance_name, (pol, type_args.clone(), xtors));
    Ok(())
}

impl Ty {
    /// Checks the validity of a given type within the context of a given symbol table
    pub fn check(&self, span: &Span, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        match self {
            Ty::I64 { .. } => Ok(()),
            Ty::Decl {
                name, type_args, ..
            } => {
                let instance_name = name.clone() + &type_args.print_to_string(None);
                match symbol_table.types.get(&instance_name) {
                    Some(_) => Ok(()),
                    None => match symbol_table.type_templates.get(name) {
                        None => Err(Error::Undefined {
                            span: span.to_miette(),
                            name: name.clone(),
                        }),
                        Some((pol, type_params, xtors)) => create_instance(
                            *span,
                            instance_name,
                            type_args,
                            pol.clone(),
                            type_params.clone(),
                            xtors.clone(),
                            symbol_table,
                        ),
                    },
                }
            }
        }
    }

    /// Checks the validity of a polymorphic type
    pub fn check_template(
        &self,
        span: &Span,
        symbol_table: &SymbolTable,
        type_params: &TypeContext,
    ) -> Result<(), Error> {
        match self {
            Ty::I64 { .. } => Ok(()),
            Ty::Decl { name, .. } => match symbol_table.type_templates.get(name) {
                Some(_) => Ok(()),
                None => {
                    if type_params.bindings.contains(name) {
                        Ok(())
                    } else {
                        Err(Error::Undefined {
                            span: span.to_miette(),
                            name: name.clone(),
                        })
                    }
                }
            },
        }
    }

    /// Creates a type int with no defined source location
    pub fn mk_i64() -> Self {
        Ty::I64 {
            span: Span::default(),
        }
    }

    /// Creates a data/codata type with no source location
    pub fn mk_decl(name: &str, type_args: TypeArgs) -> Self {
        Ty::Decl {
            span: Span::default(),
            name: name.to_string(),
            type_args,
        }
    }

    /// substitutes names with types inside a given type
    pub fn subst_ty(self, mappings: &HashMap<Name, Ty>) -> Ty {
        match self {
            Ty::I64 { .. } => self,
            Ty::Decl {
                span,
                name,
                type_args,
            } => match mappings.get(&name) {
                Some(ty) => ty.clone(),
                None => Ty::Decl {
                    span,
                    name,
                    type_args: TypeArgs {
                        span: type_args.span,
                        args: type_args
                            .args
                            .into_iter()
                            .map(|ty| ty.subst_ty(mappings))
                            .collect(),
                    },
                },
            },
        }
    }
}

pub trait OptTyped {
    fn get_type(&self) -> Option<Ty>;
}

impl Print for Ty {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Ty::I64 { .. } => alloc.keyword(I64),
            Ty::Decl {
                name, type_args, ..
            } => alloc.typ(name).append(type_args.print(cfg, alloc)),
        }
    }
}

/// A list of type parameters.
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TypeArgs {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The arguments
    pub args: Vec<Ty>,
}

impl TypeArgs {
    /// Check whether the type context is an instance of the expected one.
    pub fn is_instance(
        &self,
        template: &TypeContext,
        symbol_table: &mut SymbolTable,
    ) -> Result<(), Error> {
        if self.args.len() != template.bindings.len() {
            return Err(Error::WrongNumberOfTypeArguments {
                span: self.span.to_miette(),
                expected: template.bindings.len(),
                got: self.args.len(),
            });
        }
        for typ in &self.args {
            typ.check(&self.span, symbol_table)?;
        }
        Ok(())
    }

    /// Creates type parameters with undefined source location
    pub fn mk(args: Vec<Ty>) -> Self {
        TypeArgs {
            span: Span::default(),
            args,
        }
    }
}

impl Print for TypeArgs {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.args.is_empty() {
            alloc.nil()
        } else {
            let sep = alloc.text(COMMA).append(alloc.space());
            alloc
                .intersperse(self.args.iter().map(|arg| arg.print(cfg, alloc)), sep)
                .brackets()
        }
    }
}

#[cfg(test)]
mod type_tests {
    use printer::Print;

    use super::Ty;

    #[test]
    fn display_i64() {
        assert_eq!(Ty::mk_i64().print_to_string(None), "i64".to_owned())
    }
}
