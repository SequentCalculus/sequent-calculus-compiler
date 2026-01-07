use axcut::syntax::{Def, Name, Var};

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mark {
    None,
    Once,
    Retain,
}

impl Mark {
    pub fn increment(&mut self) {
        match self {
            Mark::None => *self = Mark::Once,
            Mark::Once => *self = Mark::Retain,
            Mark::Retain => *self = Mark::Retain,
        }
    }
}

#[derive(Debug)]
pub struct DefInfo {
    pub position: usize,
    pub mark: Mark,
}

pub struct CleanupInlineState {
    pub defs: Vec<Def>,
    pub def_map: HashMap<Name, DefInfo>,
    pub used_vars: HashSet<Var>,
}

pub trait CleanupInlineGather {
    type Target;
    fn cleanup_inline_gather(self, state: &mut CleanupInlineState) -> Self::Target;
}

impl<T: CleanupInlineGather + Clone> CleanupInlineGather for Rc<T> {
    type Target = Rc<T::Target>;
    fn cleanup_inline_gather(self, state: &mut CleanupInlineState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).cleanup_inline_gather(state))
    }
}

impl<T: CleanupInlineGather> CleanupInlineGather for Vec<T> {
    type Target = Vec<T::Target>;
    fn cleanup_inline_gather(self, state: &mut CleanupInlineState) -> Self::Target {
        self.into_iter()
            .map(|element| element.cleanup_inline_gather(state))
            .collect()
    }
}

pub trait CleanupInline {
    type Target;
    fn cleanup_inline(self, state: &mut CleanupInlineState) -> Self::Target;
}

impl<T: CleanupInline + Clone> CleanupInline for Rc<T> {
    type Target = Rc<T::Target>;
    fn cleanup_inline(self, state: &mut CleanupInlineState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).cleanup_inline(state))
    }
}

impl<T: CleanupInline> CleanupInline for Vec<T> {
    type Target = Vec<T::Target>;
    fn cleanup_inline(self, state: &mut CleanupInlineState) -> Self::Target {
        self.into_iter()
            .map(|element| element.cleanup_inline(state))
            .collect()
    }
}

pub trait Rename {
    fn rename(self, vars_to_rename: &HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self;
}

impl<T: Rename + Clone> Rename for Rc<T> {
    fn rename(self, vars_to_rename: &HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).rename(vars_to_rename, used_vars))
    }
}
