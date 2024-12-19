use printer::theme::ThemeExt;
use printer::tokens::{LEFT_ARROW, READ_STDIN, SEMI, WRITE_STDOUT};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::filter_by_set, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadStdin {
    pub buffer: Var,
    pub count: Var,
    pub var: Var,
    pub case: Rc<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteStdout {
    pub buffer: Var,
    pub count: Var,
    pub var: Var,
    pub case: Rc<Statement>,
}

impl Print for ReadStdin {
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
            .append(alloc.keyword(READ_STDIN))
            .append(alloc.space())
            .append(&self.buffer)
            .append(alloc.space())
            .append(&self.count)
            .append(SEMI)
            .append(alloc.line())
            .append(self.case.print(cfg, alloc))
    }
}

impl Print for WriteStdout {
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
            .append(alloc.keyword(WRITE_STDOUT))
            .append(alloc.space())
            .append(&self.buffer)
            .append(alloc.space())
            .append(&self.count)
            .append(SEMI)
            .append(alloc.line())
            .append(self.case.print(cfg, alloc))
    }
}

impl From<ReadStdin> for Statement {
    fn from(value: ReadStdin) -> Self {
        Statement::ReadStdin(value)
    }
}

impl From<WriteStdout> for Statement {
    fn from(value: WriteStdout) -> Self {
        Statement::WriteStdout(value)
    }
}

impl FreeVars for ReadStdin {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        vars.remove(&self.var);
        vars.insert(self.buffer.clone());
        vars.insert(self.count.clone());
    }
}

impl FreeVars for WriteStdout {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        vars.remove(&self.var);
        vars.insert(self.buffer.clone());
        vars.insert(self.count.clone());
    }
}

impl Subst for ReadStdin {
    type Target = ReadStdin;

    fn subst_sim(self, subst: &[(Var, Var)]) -> ReadStdin {
        ReadStdin {
            buffer: self.buffer.subst_sim(subst),
            count: self.count.subst_sim(subst),
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl Subst for WriteStdout {
    type Target = WriteStdout;

    fn subst_sim(self, subst: &[(Var, Var)]) -> WriteStdout {
        WriteStdout {
            buffer: self.buffer.subst_sim(subst),
            count: self.count.subst_sim(subst),
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl Linearizing for ReadStdin {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.case.free_vars(&mut free_vars);
        free_vars.insert(self.buffer.clone());
        free_vars.insert(self.count.clone());

        let mut new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        new_context.push(self.var.clone());
        let read_stdin = ReadStdin {
            buffer: self.buffer,
            count: self.count,
            var: self.var,
            case: self.case.linearize(new_context, used_vars),
        }
        .into();

        if context == context_rearrange {
            read_stdin
        } else {
            let rearrange = context_rearrange
                .clone()
                .into_iter()
                .zip(context_rearrange)
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(read_stdin),
            }
            .into()
        }
    }
}

impl Linearizing for WriteStdout {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.case.free_vars(&mut free_vars);
        free_vars.insert(self.buffer.clone());
        free_vars.insert(self.count.clone());

        let mut new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        new_context.push(self.var.clone());
        let write_stdout = WriteStdout {
            buffer: self.buffer,
            count: self.count,
            var: self.var,
            case: self.case.linearize(new_context, used_vars),
        }
        .into();

        if context == context_rearrange {
            write_stdout
        } else {
            let rearrange = context_rearrange
                .clone()
                .into_iter()
                .zip(context_rearrange)
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(write_stdout),
            }
            .into()
        }
    }
}
