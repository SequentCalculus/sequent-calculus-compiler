use std::collections::HashMap;

use crate::syntax::{
    context::TypingContext,
    declarations::{
        CodataDeclaration, CtorSig, DataDeclaration, Declaration, Definition, DtorSig, Module,
    },
    types::Ty,
    Name,
};

use super::errors::Error;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub funs: HashMap<Name, (TypingContext, Ty)>,
    pub ctors: HashMap<Name, TypingContext>,
    pub dtors: HashMap<Name, (TypingContext, Ty)>,
}

pub trait BuildSymbolTable {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error>;
}

impl BuildSymbolTable for Module {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        for decl in self.declarations.iter() {
            decl.build(symbol_table)?
        }
        Ok(())
    }
}

impl BuildSymbolTable for Declaration {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        match self {
            Declaration::Definition(definition) => definition.build(symbol_table),
            Declaration::DataDeclaration(data_declaration) => data_declaration.build(symbol_table),
            Declaration::CodataDeclaration(codata_declaration) => {
                codata_declaration.build(symbol_table)
            }
        }
    }
}

impl BuildSymbolTable for Definition {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.funs.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes(self.name.clone()));
        } else {
            symbol_table.funs.insert(
                self.name.clone(),
                (self.context.clone(), self.ret_ty.clone()),
            );
        }
        Ok(())
    }
}

impl BuildSymbolTable for DataDeclaration {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        for ctor in self.ctors.iter() {
            ctor.build(symbol_table)?;
        }
        Ok(())
    }
}

impl BuildSymbolTable for CtorSig {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.ctors.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes(self.name.clone()));
        }
        symbol_table
            .ctors
            .insert(self.name.clone(), self.args.clone());
        Ok(())
    }
}

impl BuildSymbolTable for CodataDeclaration {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        for dtor in self.dtors.iter() {
            dtor.build(symbol_table)?
        }
        Ok(())
    }
}

impl BuildSymbolTable for DtorSig {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.dtors.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes(self.name.clone()));
        }
        symbol_table
            .dtors
            .insert(self.name.clone(), (self.args.clone(), self.cont_ty.clone()));
        Ok(())
    }
}
