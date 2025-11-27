use crate::syntax::{
    Chirality, ContextBinding, Ty,
    statements::{
        Call, Clause, Create, Exit, IfC, Invoke, Let, Literal, Op, PrintI64, Statement, Substitute,
        Switch,
    },
};
use std::collections::BTreeSet;

pub trait TypedFreeVars {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding>;
}

impl TypedFreeVars for Clause {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        let mut bindings = self.body.typed_free_vars();
        bindings.retain(|bnd| !self.context.bindings.contains(bnd));
        bindings
    }
}

impl TypedFreeVars for Statement {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        match self {
            Statement::Substitute(subst) => subst.typed_free_vars(),
            Statement::Call(call) => call.typed_free_vars(),
            Statement::Let(lt) => lt.typed_free_vars(),
            Statement::Switch(switch) => switch.typed_free_vars(),
            Statement::Create(cr) => cr.typed_free_vars(),
            Statement::Invoke(inv) => inv.typed_free_vars(),
            Statement::Literal(lit) => lit.typed_free_vars(),
            Statement::Op(op) => op.typed_free_vars(),
            Statement::PrintI64(prnt) => prnt.typed_free_vars(),
            Statement::IfC(ifc) => ifc.typed_free_vars(),
            Statement::Exit(ex) => ex.typed_free_vars(),
        }
    }
}

impl TypedFreeVars for Substitute {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        self.next.typed_free_vars()
    }
}

impl TypedFreeVars for Call {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        self.args.bindings.iter().cloned().collect()
    }
}

impl TypedFreeVars for Let {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        let mut bindings = self.next.typed_free_vars();
        bindings.retain(|bnd| bnd.var != self.var);
        bindings.extend(self.args.bindings.iter().cloned());
        bindings
    }
}

impl TypedFreeVars for Switch {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        let mut bindings = BTreeSet::from([ContextBinding {
            var: self.var.clone(),
            ty: self.ty.clone(),
            chi: Chirality::Prd,
        }]);
        for clause in self.clauses.iter() {
            bindings.extend(clause.typed_free_vars())
        }
        bindings
    }
}

impl TypedFreeVars for Create {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        let mut bindings = self.next.typed_free_vars();
        bindings.retain(|bnd| bnd.var != self.var);
        for clause in self.clauses.iter() {
            bindings.extend(clause.typed_free_vars());
        }
        bindings
    }
}

impl TypedFreeVars for Invoke {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        let mut bindings = BTreeSet::from([ContextBinding {
            var: self.var.clone(),
            ty: self.ty.clone(),
            chi: Chirality::Cns,
        }]);
        bindings.extend(self.args.bindings.iter().cloned());
        bindings
    }
}

impl TypedFreeVars for Literal {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        self.next
            .typed_free_vars()
            .into_iter()
            .filter(|bnd| bnd.var != self.var)
            .collect()
    }
}

impl TypedFreeVars for Op {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        let mut bindings = self.next.typed_free_vars();
        bindings.retain(|bnd| bnd.var != self.var);
        bindings.insert(ContextBinding {
            var: self.fst.clone(),
            ty: Ty::I64,
            chi: Chirality::Ext,
        });
        bindings.insert(ContextBinding {
            var: self.snd.clone(),
            ty: Ty::I64,
            chi: Chirality::Ext,
        });
        bindings
    }
}

impl TypedFreeVars for PrintI64 {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        let mut bindings = BTreeSet::from([ContextBinding {
            var: self.var.clone(),
            ty: Ty::I64,
            chi: Chirality::Ext,
        }]);
        bindings.extend(self.next.typed_free_vars());
        bindings
    }
}

impl TypedFreeVars for IfC {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        let mut bindings = BTreeSet::from([ContextBinding {
            var: self.fst.clone(),
            ty: Ty::I64,
            chi: Chirality::Ext,
        }]);
        if let Some(ref snd) = self.snd {
            bindings.insert(ContextBinding {
                var: snd.clone(),
                ty: Ty::I64,
                chi: Chirality::Ext,
            });
        }
        bindings.extend(self.thenc.typed_free_vars());
        bindings.extend(self.elsec.typed_free_vars());
        bindings
    }
}

impl TypedFreeVars for Exit {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        BTreeSet::from([ContextBinding {
            var: self.var.clone(),
            ty: Ty::I64,
            chi: Chirality::Ext,
        }])
    }
}
