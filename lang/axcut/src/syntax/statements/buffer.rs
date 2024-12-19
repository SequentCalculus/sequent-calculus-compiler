use printer::theme::ThemeExt;
use printer::tokens::{GET_BYTE, LEFT_ARROW, SEMI, SET_BYTE};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::filter_by_set, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetByte {
    pub buffer: Var,
    pub offset: Var,
    pub var: Var,
    pub case: Rc<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetByte {
    pub buffer: Var,
    pub offset: Var,
    pub var: Var,
    pub case: Rc<Statement>,
}

impl Print for GetByte {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.var)
            .append(alloc.space())
            .append(LEFT_ARROW)
            .append(alloc.space())
            .append(alloc.keyword(GET_BYTE))
            .append(alloc.space())
            .append(&self.buffer)
            .append(alloc.space())
            .append(&self.offset)
            .append(SEMI)
            .append(alloc.line())
            .append(self.case.print(cfg, alloc))
    }
}

impl Print for SetByte {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(SET_BYTE)
            .append(alloc.space())
            .append(&self.buffer)
            .append(alloc.space())
            .append(&self.offset)
            .append(alloc.space())
            .append(&self.var)
            .append(SEMI)
            .append(alloc.line())
            .append(self.case.print(cfg, alloc))
    }
}

impl From<GetByte> for Statement {
    fn from(value: GetByte) -> Self {
        Statement::GetByte(value)
    }
}

impl From<SetByte> for Statement {
    fn from(value: SetByte) -> Self {
        Statement::SetByte(value)
    }
}

impl FreeVars for GetByte {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        vars.remove(&self.var);
        vars.insert(self.buffer.clone());
        vars.insert(self.offset.clone());
    }
}

impl FreeVars for SetByte {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        vars.insert(self.buffer.clone());
        vars.insert(self.offset.clone());
        vars.insert(self.var.clone());
    }
}

impl Subst for GetByte {
    type Target = GetByte;

    fn subst_sim(self, subst: &[(Var, Var)]) -> GetByte {
        GetByte {
            buffer: self.buffer.subst_sim(subst),
            offset: self.offset.subst_sim(subst),
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl Subst for SetByte {
    type Target = SetByte;

    fn subst_sim(self, subst: &[(Var, Var)]) -> SetByte {
        SetByte {
            buffer: self.buffer.subst_sim(subst),
            offset: self.offset.subst_sim(subst),
            var: self.var.subst_sim(subst),
            case: self.case.subst_sim(subst),
        }
    }
}

impl Linearizing for GetByte {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.case.free_vars(&mut free_vars);
        free_vars.insert(self.buffer.clone());
        free_vars.insert(self.offset.clone());

        let mut new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        new_context.push(self.var.clone());
        let get_byte = GetByte {
            buffer: self.buffer,
            offset: self.offset,
            var: self.var,
            case: self.case.linearize(new_context, used_vars),
        }
        .into();

        if context == context_rearrange {
            get_byte
        } else {
            let rearrange = context_rearrange
                .clone()
                .into_iter()
                .zip(context_rearrange)
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(get_byte),
            }
            .into()
        }
    }
}

impl Linearizing for SetByte {
    type Target = SetByte;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> SetByte {
        SetByte {
            buffer: self.buffer,
            offset: self.offset,
            var: self.var,
            case: self.case.linearize(context.clone(), used_vars),
        }
    }
}
