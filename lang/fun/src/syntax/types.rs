//! This module defines types in Fun.

use codespan::Span;
use derivative::Derivative;
use printer::tokens::I64;
use printer::*;

use crate::parser::util::ToMiette;
use crate::syntax::*;
use crate::traits::SubstType;
use crate::typing::*;

use std::collections::HashMap;

/// This enum encodes the monomorphic types of AxCut. They are either integers, or instances of
/// user-declared type templates, or, during typechecking, type parameters standing for a
/// monomorphic type. They are never type templates and after typechecking also cannot be type
/// parameters anymore since then all of them are instantiated.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum Ty {
    /// Signed 64-bit integer.
    I64 {
        #[derivative(PartialEq = "ignore")]
        span: Span,
    },
    /// Monomorphic instance of user-declared (data or codata) type template, or the name of a type
    /// parameter.
    Decl {
        #[derivative(PartialEq = "ignore")]
        span: Span,
        /// For an instance this is the name of the template corresponding to this instance, for a
        /// type parameter it is just its name.
        name: Name,
        /// For an instance these are the type arguments for the type parameters of the
        /// corresponding template, for a type parameter this is empty.
        type_args: TypeArgs,
    },
}

impl Ty {
    /// This function checks the well-formedness of a type during typechecking. For a user-declared
    /// type it creates a monomorphic instance of the corresponding template if necessary. The type
    /// must not be a type parameter.
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

    /// This function checks the well-formedness of a type within a type template during
    /// typechecking. For a user-declared type this means that a template with its name must exist
    /// or it must be one of the type parameters of the template.
    /// - `symbol_table` is the symbol table during typechecking.
    /// - `type_params` is the list of type parameters of the template.
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

    /// This function creates an i64 type with no defined source location.
    pub fn mk_i64() -> Self {
        Ty::I64 {
            span: Span::default(),
        }
    }

    /// This function creates a monomorphic user-defined type with no source location.
    /// - `name` is the name of the type.
    /// - `type_args` are the type arguments for the type parameters of the corresponding template.
    pub fn mk_decl(name: &str, type_args: TypeArgs) -> Self {
        Ty::Decl {
            span: Span::default(),
            name: name.to_string(),
            type_args,
        }
    }
}

impl SubstType for Ty {
    fn subst_ty(self, mappings: &HashMap<Name, Ty>) -> Ty {
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
                    type_args: type_args.subst_ty(mappings),
                },
            },
        }
    }
}

/// This function creates a monomorphic instance of a type template and inserts it into the symbol
/// table.
/// - `span` is the source location of the instance.
/// - `instance_name` is the name of the instance.
/// - `type_args` is the list of type arguments for the instance.
/// - `pol` is the polarity of the instance, i.e., whether it is a data or codata type.
/// - `type_params` is the list of type parameters of the template from which the instance is
///   created.
/// - `xtors` is the list of xtor names of the template.
/// - `symbol_table` is the symbol table during typechecking.
///
/// The xtor names of the instance are appended with the type arguments (as in the instance name)
/// when inserted into the symbol table. This is only necessary during typechecking and will be
/// removed for the final instances.
///
/// Example:
/// ```text
/// data List[A] { Nil, Cons(x: A, xs: List[A]) }
/// let l: List[i64] = Cons(1, Nil); ...
/// ```
/// `List` is polymorphic in `A` and is used here with `A = i64`. The created instance will thus be
/// ```text
/// data List[i64] { Nil, Cons(x: i64, xs: List[i64]) }
/// ```
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

/// This trait provides a fallible method to obtain the type of a term.
pub trait OptTyped {
    /// This method returns the type of a term if it is known.
    fn get_type(&self) -> Option<Ty>;
}

impl Print for Ty {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            Ty::I64 { .. } => alloc.typ(I64),
            // the name of an instance is the name of the template with the type arguments appended
            Ty::Decl {
                name, type_args, ..
            } => alloc.typ(name).append(type_args.print(cfg, alloc)),
        }
    }
}

/// This struct defines a list of monomorphic type arguments.
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TypeArgs {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The type arguments
    pub args: Vec<Ty>,
}

impl TypeArgs {
    /// This function checks whether the type arguments form a valid instance for a list of type
    /// parameters.
    /// - `template` is the list of type parameters.
    /// - `symbol_table` is the symbol table during typechecking.
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

    /// This function creates type arguments with undefined source location from a given list of
    /// monomorphic types.
    pub fn mk(args: Vec<Ty>) -> Self {
        TypeArgs {
            span: Span::default(),
            args,
        }
    }
}

impl SubstType for TypeArgs {
    fn subst_ty(mut self, mappings: &HashMap<Name, Ty>) -> Self {
        self.args = self.args.subst_ty(mappings);
        self
    }
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
    use printer::Print;

    use super::Ty;

    #[test]
    fn display_i64() {
        assert_eq!(Ty::mk_i64().print_to_string(None), "i64".to_owned())
    }
}
