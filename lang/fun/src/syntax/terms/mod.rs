//! This module defines the terms of Fun.

pub mod call;
pub mod case;
pub mod clause;
pub mod constructor;
pub mod destructor;
pub mod exit;
pub mod goto;
pub mod ifc;
pub mod label;
pub mod r#let;
pub mod literal;
pub mod new;
pub mod op;
pub mod paren;
pub mod print;
pub mod var;

pub use call::*;
pub use case::*;
pub use clause::*;
pub use constructor::*;
pub use destructor::*;
pub use exit::*;
pub use goto::*;
pub use ifc::*;
pub use label::*;
pub use r#let::*;
pub use literal::*;
pub use new::*;
pub use op::*;
pub use paren::*;
pub use print::*;
pub use var::*;

use printer::Print;

use crate::{
    syntax::names::Var,
    traits::used_binders::UsedBinders,
    typing::{errors::Error, inference::Inference, symbol_table::SymbolTable},
};

use super::{
    context::TypingContext,
    types::{OptTyped, Ty},
};

use std::collections::{HashMap, HashSet};

/// This enum defines the terms of Fun. It contains one variant for each construct which simply
/// wraps the struct defining the corresponding construct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// Variable or Covariable
    XVar(XVar),
    /// Integer literal
    Lit(Lit),
    /// Arithmetic binary operations
    Op(Op),
    /// Conditional comparing two integers
    IfC(IfC),
    /// Printing an integer
    PrintI64(PrintI64),
    /// Let-binding of a term
    Let(Let),
    /// Call of a top-level function
    Call(Call),
    /// Constructor of a data type
    Constructor(Constructor),
    /// Destructor of a codata type
    Destructor(Destructor),
    /// Pattern match for a data type
    Case(Case),
    /// Copattern match for a codata type
    New(New),
    /// Control operator for capturing current continuation/program context
    Label(Label),
    /// control operator for invoking a captured continuation/program context
    Goto(Goto),
    /// Exiting the program
    Exit(Exit),
    /// Parethesized term
    Paren(Paren),
}

impl OptTyped for Term {
    fn get_type(&self) -> Option<Ty> {
        match self {
            Term::XVar(var) => var.get_type(),
            Term::Lit(lit) => lit.get_type(),
            Term::Op(op) => op.get_type(),
            Term::IfC(ifc) => ifc.get_type(),
            Term::PrintI64(print) => print.get_type(),
            Term::Let(lt) => lt.get_type(),
            Term::Call(call) => call.get_type(),
            Term::Constructor(ctor) => ctor.get_type(),
            Term::Destructor(dtor) => dtor.get_type(),
            Term::Case(case) => case.get_type(),
            Term::New(new) => new.get_type(),
            Term::Goto(goto) => goto.get_type(),
            Term::Label(label) => label.get_type(),
            Term::Exit(exit) => exit.get_type(),
            Term::Paren(paren) => paren.get_type(),
        }
    }
}

impl Print for Term {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Term::XVar(var) => var.print(cfg, alloc),
            Term::Lit(lit) => lit.print(cfg, alloc),
            Term::Op(op) => op.print(cfg, alloc),
            Term::IfC(ifc) => ifc.print(cfg, alloc),
            Term::PrintI64(print) => print.print(cfg, alloc),
            Term::Let(r#let) => r#let.print(cfg, alloc),
            Term::Call(call) => call.print(cfg, alloc),
            Term::Constructor(constructor) => constructor.print(cfg, alloc),
            Term::Destructor(destructor) => destructor.print(cfg, alloc),
            Term::Case(case) => case.print(cfg, alloc),
            Term::New(new) => new.print(cfg, alloc),
            Term::Goto(goto) => goto.print(cfg, alloc),
            Term::Label(label) => label.print(cfg, alloc),
            Term::Exit(exit) => exit.print(cfg, alloc),
            Term::Paren(paren) => paren.print(cfg, alloc),
        }
    }
}

impl Inference for Term {
    fn gather_constraints(
            &mut self,
            constraint_bank: &mut crate::typing::inference::ConstraintBank,
            context: &TypingContext,
            ty_var: Ty
        ) -> Result<(), Error> {
        match self {
            Term::XVar(xvar) => xvar.gather_constraints(constraint_bank, context, ty_var),
            Term::Lit(lit) => lit.gather_constraints(constraint_bank, context, ty_var),
            Term::Op(op) => op.gather_constraints(constraint_bank, context, ty_var),
            Term::IfC(if_c) => if_c.gather_constraints(constraint_bank, context, ty_var),
            Term::PrintI64(print_i64) => print_i64.gather_constraints(constraint_bank, context, ty_var),
            Term::Let(let_block) => let_block.gather_constraints(constraint_bank, context, ty_var),
            Term::Call(call) => call.gather_constraints(constraint_bank, context, ty_var),
            Term::Constructor(constructor) => constructor.gather_constraints(constraint_bank, context, ty_var),
            Term::Destructor(destructor) => destructor.gather_constraints(constraint_bank, context, ty_var),
            Term::Case(case) => case.gather_constraints(constraint_bank, context, ty_var),
            Term::New(new_block) => new_block.gather_constraints(constraint_bank, context, ty_var),
            Term::Label(label) => label.gather_constraints(constraint_bank, context, ty_var),
            Term::Goto(goto) => goto.gather_constraints(constraint_bank, context, ty_var),
            Term::Exit(exit) => exit.gather_constraints(constraint_bank, context, ty_var),
            Term::Paren(paren) => paren.gather_constraints(constraint_bank, context, ty_var),
        }
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &std::collections::HashMap<super::Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>
    ) -> Result<(), Error> {
        match self {
            Term::XVar(xvar) => xvar.insert_inferred_type(mappings, symbol_table, choices),
            Term::Lit(lit) => lit.insert_inferred_type(mappings, symbol_table, choices),
            Term::Op(op) => op.insert_inferred_type(mappings, symbol_table, choices),
            Term::IfC(if_c) => if_c.insert_inferred_type(mappings, symbol_table, choices),
            Term::PrintI64(print_i64) => print_i64.insert_inferred_type(mappings, symbol_table, choices),
            Term::Let(let_block) => let_block.insert_inferred_type(mappings, symbol_table, choices),
            Term::Call(call) => call.insert_inferred_type(mappings, symbol_table, choices),
            Term::Constructor(constructor) => constructor.insert_inferred_type(mappings, symbol_table, choices),
            Term::Destructor(destructor) => destructor.insert_inferred_type(mappings, symbol_table, choices),
            Term::Case(case) => case.insert_inferred_type(mappings, symbol_table, choices),
            Term::New(new_block) => new_block.insert_inferred_type(mappings, symbol_table, choices),
            Term::Label(label) => label.insert_inferred_type(mappings, symbol_table, choices),
            Term::Goto(goto) => goto.insert_inferred_type(mappings, symbol_table, choices),
            Term::Exit(exit) => exit.insert_inferred_type(mappings, symbol_table, choices),
            Term::Paren(paren) => paren.insert_inferred_type(mappings, symbol_table, choices),
        }
    }
}

impl UsedBinders for Term {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            Term::XVar(_) | Term::Lit(_) => {}
            Term::Op(op) => op.used_binders(used),
            Term::IfC(ifc) => ifc.used_binders(used),
            Term::PrintI64(print) => print.used_binders(used),
            Term::Let(r#let) => r#let.used_binders(used),
            Term::Call(call) => call.used_binders(used),
            Term::Constructor(constructor) => constructor.used_binders(used),
            Term::Destructor(destructor) => destructor.used_binders(used),
            Term::Case(case) => case.used_binders(used),
            Term::New(new) => new.used_binders(used),
            Term::Goto(goto) => goto.used_binders(used),
            Term::Label(label) => label.used_binders(used),
            Term::Exit(exit) => exit.used_binders(used),
            Term::Paren(paren) => paren.used_binders(used),
        }
    }
}

#[cfg(feature = "test-common")]
pub mod inferr_helper {
    use std::collections::HashMap;

    use crate::{syntax::{Term, Ty, TypingContext}, typing::{Error, inference::{ConstraintBank, Inference, constraint_unification}, symbol_table::SymbolTable}};
    

    pub fn inferr_term(term: &mut Term, symbol_table: &mut SymbolTable, context: &TypingContext) -> Result<(), Error> {
        let mut constraint_bank = ConstraintBank{
            symbol_table: symbol_table.clone(),
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        let ty_var = constraint_bank.var_name_generator.get_new_ty_var();

        term.gather_constraints(&mut constraint_bank, context, ty_var)?;

        let ConstraintBank { mut symbol_table, constraints, possible_choices, .. } = constraint_bank;

        let (solutions, conflicts) = constraint_unification(constraints);

        // generating a type and choice mapping, either with overload resolution or without
        // overload resolution is only done, if there are any overloads to resolve
        let (mut type_mapping, choices_map): (HashMap<String, Ty>, HashMap<u32, usize>) = if possible_choices.len() > 0 {
            let selected_world = crate::typing::world_resolution::resolve_worlds(&possible_choices, conflicts)?;

            let choices_map: HashMap<u32, usize> = selected_world.iter().cloned().collect();

            // now all solutions that are part of the selected world are filtered.
            let mut selected_solutions = solutions;
            for (choice_id, signature_id) in selected_world {
                selected_solutions.retain(|s| match s.choices.get(&choice_id) {
                    Some(id) => signature_id == *id,

                    // if the solution doesn't have a choice for the wanted name, it is invariant to the choice. So it is part of the world
                    None => true
                });
            }

            // the solutions are converted to a HashMap and then they are inserted in the program
            (selected_solutions.into_iter().map(crate::typing::inference::Solution::get_only_solution).collect(), choices_map)
        } else {
            (solutions.into_iter().map(crate::typing::inference::Solution::get_only_solution).collect(), Default::default())
        };

        // the type mapping is applied on it self, to get the complete transitive hull
        let reference_mapping = type_mapping.clone();

        for (_, ty) in type_mapping.iter_mut() {
            loop {
                let var_names = ty.collect_var_names();

                if var_names.is_empty() {
                    break;
                }

                if var_names.iter().any(|s| !reference_mapping.contains_key(s)) {
                    let missing_names: Vec<String> = ty.collect_var_names().into_iter().filter(|k| !reference_mapping.contains_key(k)).collect();
                    panic!("Missing type var names in the final type mapping: {:?}", missing_names);
                }

                ty.mut_subst_ty(&reference_mapping);
            }            
        }

        println!("Term before insertion: {:?}", term);
        term.insert_inferred_type(&type_mapping, &mut symbol_table, &choices_map)
    }
}
