use super::{types::Ty, Covariable, Variable};

pub enum ContextBinding {
    VarBinding { var: Variable, ty: Ty },
    CovarBinding { covar: Covariable, ty: Ty },
}

pub type TypingContext = Vec<ContextBinding>;
