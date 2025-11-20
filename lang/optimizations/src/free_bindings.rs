use axcut::syntax::{
    Chirality, ContextBinding, Ty,
    statements::{
        Call, Clause, Create, Exit, IfC, Invoke, Let, Literal, Op, PrintI64, Statement, Substitute,
        Switch,
    },
};
use std::collections::HashSet;

pub trait FreeBindings {
    fn free_bindings(&self) -> HashSet<ContextBinding>;
}

impl FreeBindings for Clause {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        let mut bindings = self.body.free_bindings();
        bindings.retain(|bnd| !self.context.bindings.contains(bnd));
        bindings
    }
}

impl FreeBindings for Statement {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        match self {
            Statement::Substitute(subst) => subst.free_bindings(),
            Statement::Call(call) => call.free_bindings(),
            Statement::Let(lt) => lt.free_bindings(),
            Statement::Switch(switch) => switch.free_bindings(),
            Statement::Create(cr) => cr.free_bindings(),
            Statement::Invoke(inv) => inv.free_bindings(),
            Statement::Literal(lit) => lit.free_bindings(),
            Statement::Op(op) => op.free_bindings(),
            Statement::PrintI64(prnt) => prnt.free_bindings(),
            Statement::IfC(ifc) => ifc.free_bindings(),
            Statement::Exit(ex) => ex.free_bindings(),
        }
    }
}

impl FreeBindings for Substitute {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        self.next.free_bindings()
    }
}

impl FreeBindings for Call {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        self.args.bindings.iter().cloned().collect()
    }
}

impl FreeBindings for Let {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        let mut bindings = self.next.free_bindings();
        bindings.retain(|bnd| bnd.var != self.var);
        bindings.extend(self.args.bindings.iter().cloned());
        bindings
    }
}

impl FreeBindings for Switch {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        let mut bindings = HashSet::from([ContextBinding {
            var: self.var.clone(),
            ty: self.ty.clone(),
            chi: Chirality::Cns,
        }]);
        for clause in self.clauses.iter() {
            bindings.extend(clause.free_bindings())
        }
        bindings
    }
}

impl FreeBindings for Create {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        let mut bindings = self.next.free_bindings();
        bindings.retain(|bnd| bnd.var != self.var);
        for clause in self.clauses.iter() {
            bindings.extend(clause.free_bindings());
        }
        bindings
    }
}

impl FreeBindings for Invoke {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        let mut bindings = HashSet::from([ContextBinding {
            var: self.var.clone(),
            ty: self.ty.clone(),
            chi: Chirality::Cns,
        }]);
        bindings.extend(self.args.bindings.iter().cloned());
        bindings
    }
}

impl FreeBindings for Literal {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        self.next
            .free_bindings()
            .into_iter()
            .filter(|bnd| bnd.var != self.var)
            .collect()
    }
}

impl FreeBindings for Op {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        let mut bindings = self.next.free_bindings();
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

impl FreeBindings for PrintI64 {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        let mut bindings = HashSet::from([ContextBinding {
            var: self.var.clone(),
            ty: Ty::I64,
            chi: Chirality::Ext,
        }]);
        bindings.extend(self.next.free_bindings());
        bindings
    }
}

impl FreeBindings for IfC {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        let mut bindings = HashSet::from([ContextBinding {
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
        bindings.extend(self.thenc.free_bindings());
        bindings.extend(self.elsec.free_bindings());
        bindings
    }
}

impl FreeBindings for Exit {
    fn free_bindings(&self) -> HashSet<ContextBinding> {
        HashSet::from([ContextBinding {
            var: self.var.clone(),
            ty: Ty::I64,
            chi: Chirality::Ext,
        }])
    }
}
