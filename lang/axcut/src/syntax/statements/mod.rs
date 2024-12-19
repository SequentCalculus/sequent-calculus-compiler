pub mod buffer;
pub mod call;
pub mod ifc;
pub mod ifz;
pub mod invoke;
pub mod leta;
pub mod literal;
pub mod mmap;
pub mod new;
pub mod op;
pub mod ret;
pub mod stdio;
pub mod substitute;
pub mod switch;

pub use buffer::{GetByte, SetByte};
pub use call::Call;
pub use ifc::IfC;
pub use ifz::IfZ;
pub use invoke::Invoke;
pub use leta::Leta;
pub use literal::Literal;
pub use mmap::{MMapAnonymousPage, MUnmapPage};
pub use new::New;
pub use op::Op;
pub use ret::Return;
pub use stdio::{ReadStdin, WriteStdout};
pub use substitute::Substitute;
pub use switch::Switch;

use printer::{theme::ThemeExt, tokens::DONE, Print};

use super::Var;
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Substitute(Substitute),
    Call(Call),
    Leta(Leta),
    Switch(Switch),
    New(New),
    Invoke(Invoke),
    Literal(Literal),
    Op(Op),
    IfC(IfC),
    IfZ(IfZ),
    Return(Return),
    MMapAnonymousPage(MMapAnonymousPage),
    MUnmapPage(MUnmapPage),
    GetByte(GetByte),
    SetByte(SetByte),
    ReadStdin(ReadStdin),
    WriteStdout(WriteStdout),
    Done,
}

impl FreeVars for Statement {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        match self {
            Statement::Substitute(substitute) => substitute.free_vars(vars),
            Statement::Call(call) => call.free_vars(vars),
            Statement::Leta(leta) => leta.free_vars(vars),
            Statement::Switch(swich) => swich.free_vars(vars),
            Statement::New(new) => new.free_vars(vars),
            Statement::Invoke(invoke) => invoke.free_vars(vars),
            Statement::Literal(lit) => lit.free_vars(vars),
            Statement::Op(op) => op.free_vars(vars),
            Statement::IfC(ifc) => ifc.free_vars(vars),
            Statement::IfZ(ifz) => ifz.free_vars(vars),
            Statement::Return(Return { var }) => {
                vars.insert(var.clone());
            }
            Statement::MMapAnonymousPage(mmap) => mmap.free_vars(vars),
            Statement::MUnmapPage(munmap) => munmap.free_vars(vars),
            Statement::GetByte(get) => get.free_vars(vars),
            Statement::SetByte(set) => set.free_vars(vars),
            Statement::ReadStdin(read) => read.free_vars(vars),
            Statement::WriteStdout(write) => write.free_vars(vars),
            Statement::Done => {}
        }
    }
}

impl Subst for Statement {
    type Target = Statement;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Statement {
        match self {
            Statement::Substitute(substitute) => substitute.subst_sim(subst).into(),
            Statement::Call(call) => call.subst_sim(subst).into(),
            Statement::Leta(leta) => leta.subst_sim(subst).into(),
            Statement::Switch(switch) => switch.subst_sim(subst).into(),
            Statement::New(new) => new.subst_sim(subst).into(),
            Statement::Invoke(invoke) => invoke.subst_sim(subst).into(),
            Statement::Literal(lit) => lit.subst_sim(subst).into(),
            Statement::Op(op) => op.subst_sim(subst).into(),
            Statement::IfC(ifc) => ifc.subst_sim(subst).into(),
            Statement::IfZ(ifz) => ifz.subst_sim(subst).into(),
            Statement::Return(Return { var }) => Statement::Return(Return {
                var: var.subst_sim(subst),
            }),
            Statement::MMapAnonymousPage(mmap) => mmap.subst_sim(subst).into(),
            Statement::MUnmapPage(munmap) => munmap.subst_sim(subst).into(),
            Statement::GetByte(get) => get.subst_sim(subst).into(),
            Statement::SetByte(set) => set.subst_sim(subst).into(),
            Statement::ReadStdin(read) => read.subst_sim(subst).into(),
            Statement::WriteStdout(write) => write.subst_sim(subst).into(),
            Statement::Done => Statement::Done,
        }
    }
}

impl Linearizing for Statement {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        match self {
            Statement::Substitute(_) => {
                panic!("Linearization should only be done on terms without explicit substitutions")
            }
            Statement::Call(call) => call.linearize(context, used_vars),
            Statement::Leta(leta) => leta.linearize(context, used_vars),
            Statement::Switch(switch) => switch.linearize(context, used_vars),
            Statement::New(new) => new.linearize(context, used_vars),
            Statement::Invoke(invoke) => invoke.linearize(context, used_vars),
            Statement::Literal(lit) => lit.linearize(context, used_vars),
            Statement::Op(op) => op.linearize(context, used_vars),
            Statement::IfC(ifc) => ifc.linearize(context, used_vars).into(),
            Statement::IfZ(ifz) => ifz.linearize(context, used_vars).into(),
            Statement::Return(Return { var }) => Return { var }.into(),
            Statement::MMapAnonymousPage(mmap) => mmap.linearize(context, used_vars),
            Statement::MUnmapPage(munmap) => munmap.linearize(context, used_vars).into(),
            Statement::GetByte(get) => get.linearize(context, used_vars),
            Statement::SetByte(set) => set.linearize(context, used_vars).into(),
            Statement::ReadStdin(read) => read.linearize(context, used_vars),
            Statement::WriteStdout(write) => write.linearize(context, used_vars),
            Statement::Done => Statement::Done,
        }
    }
}

impl Print for Statement {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Statement::Substitute(substitute) => substitute.print(cfg, alloc),
            Statement::Call(call) => call.print(cfg, alloc),
            Statement::Leta(leta) => leta.print(cfg, alloc),
            Statement::Switch(switch) => switch.print(cfg, alloc),
            Statement::New(new) => new.print(cfg, alloc),
            Statement::Invoke(invoke) => invoke.print(cfg, alloc),
            Statement::Literal(lit) => lit.print(cfg, alloc),
            Statement::Op(op) => op.print(cfg, alloc),
            Statement::IfC(ifc) => ifc.print(cfg, alloc),
            Statement::IfZ(ifz) => ifz.print(cfg, alloc),
            Statement::Return(ret) => ret.print(cfg, alloc),
            Statement::MMapAnonymousPage(mmap) => mmap.print(cfg, alloc),
            Statement::MUnmapPage(munmap) => munmap.print(cfg, alloc),
            Statement::GetByte(get) => get.print(cfg, alloc),
            Statement::SetByte(set) => set.print(cfg, alloc),
            Statement::ReadStdin(read) => read.print(cfg, alloc),
            Statement::WriteStdout(write) => write.print(cfg, alloc),
            Statement::Done => alloc.keyword(DONE),
        }
    }
}
