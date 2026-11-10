use std::rc::Rc;

use crate::syntax::{AnyPattern, Chi, Statement, Ty};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedClause {
    pub pattern: AnyPattern,
    pub body: Rc<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedXCase<C: Chi> {
    pub prdcns: C,
    pub clauses: Vec<NestedClause>,
    pub ty: Ty,
}
