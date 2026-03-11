use std::rc::Rc;

use miette::SourceSpan;

use crate::{syntax::{Arguments, Ty, TypeArgs, TypingContext}, typing::{Error, SymbolTable}};


pub trait Inference: Sized {

    fn constraint_equations(
        &mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        var_name_generator: &mut VarNameGenerator,
        ty_var: Ty
    ) -> Result<Vec<(Ty,Ty)>, Error>;
}

impl<T: Inference + Clone> Inference for Rc<T> {
    fn constraint_equations(
        &mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        var_name_generator: &mut VarNameGenerator,
        ty_var: Ty
    ) -> Result<Vec<(Ty,Ty)>, Error> {
        Rc::make_mut(self).constraint_equations(symbol_table, context, var_name_generator, ty_var)
    }
}

impl<T: Inference> Inference for Option<T> {
    fn constraint_equations(
        &mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        var_name_generator: &mut VarNameGenerator,
        ty_var: Ty
    ) -> Result<Vec<(Ty,Ty)>, Error> {
        match self {
            None => Ok(vec![]),
            Some(t ) => t.constraint_equations(symbol_table, context, var_name_generator, ty_var)
        }
    }
}

pub fn args_constraint_equations(
    args: &mut Arguments,
    types: &TypingContext,
    symbol_table: &mut SymbolTable,
    context: &TypingContext,
    var_name_generator: &mut VarNameGenerator,
    span: SourceSpan
) -> Result<Vec<(Ty, Ty)>, Error> {

    let mut constraints: Vec<(Ty, Ty)> = Vec::new();

    if args.entries.len() != types.bindings.len() {
        return Err(Error::WrongNumberOfArguments {
            span: span,
            expected: types.bindings.len(),
            got: args.entries.len()
        });
    }

    for (arg, expected_type) in args.entries.iter_mut().zip(types.bindings.iter()) {
        
        constraints.append(&mut arg.constraint_equations(symbol_table, context, var_name_generator, expected_type.ty.clone())?);
    }

    Ok(constraints)
}

pub struct VarNameGenerator {
    internal_counter: u32
}

impl VarNameGenerator {
    pub fn new() -> Self {
        VarNameGenerator { internal_counter: 0 }
    }

    pub fn get_new_name(&mut self) -> String {
        let new_name = self.internal_counter.to_string();
        self.internal_counter = self.internal_counter + 1;
        return new_name;
    }

    pub fn get_new_ty_var(&mut self) -> Ty {
        let name = self.get_new_name();
        Ty::Decl { span: None, name, type_args: TypeArgs::mk(vec![]) }
    }
}