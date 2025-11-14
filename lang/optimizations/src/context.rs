use axcut::syntax::{Var, statements::Let};
use std::collections::HashMap;

pub struct RewriteContext {
    let_bindings: HashMap<Var, Let>,
}

impl RewriteContext {
    pub fn new() -> Self {
        Self {
            let_bindings: HashMap::new(),
        }
    }

    pub fn add_binding(&mut self, lt: &Let) {
        self.let_bindings.insert(lt.var.clone(), lt.clone());
    }

    pub fn get_binding(&self, var: &Var) -> Option<Let> {
        self.let_bindings.get(var).cloned()
    }
}

impl Default for RewriteContext {
    fn default() -> RewriteContext {
        RewriteContext::new()
    }
}
