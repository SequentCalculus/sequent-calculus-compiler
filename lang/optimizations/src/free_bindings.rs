use axcut::syntax::{
    ContextBinding,
    statements::{
        Call, Clause, Create, Exit, IfC, Invoke, Let, Literal, Op, PrintI64, Statement, Substitute,
        Switch,
    },
};

pub trait FreeBindings {
    fn free_bindings(&self) -> Vec<ContextBinding>;
}

impl FreeBindings for Clause {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        self.body
            .free_bindings()
            .into_iter()
            .filter(|bnd| !self.context.bindings.contains(&bnd))
            .collect()
    }
}

impl FreeBindings for Statement {
    fn free_bindings(&self) -> Vec<ContextBinding> {
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
    fn free_bindings(&self) -> Vec<ContextBinding> {
        self.next.free_bindings()
    }
}

impl FreeBindings for Call {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        self.context.bindings.clone()
    }
}

impl FreeBindings for Let {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        let mut bindings = self.context.bindings.clone();
        bindings.extend(self.next.free_bindings());
        bindings
    }
}

impl FreeBindings for Switch {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        let mut bindings = vec![];
        for clause in self.clauses.iter() {
            bindings.extend(clause.free_bindings())
        }
        bindings
    }
}
impl FreeBindings for Create {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        let mut bindings = self.next.free_bindings();
        for clause in self.clauses.iter() {
            bindings.extend(clause.free_bindings());
        }
        bindings
    }
}
impl FreeBindings for Invoke {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        self.context.bindings.clone()
    }
}

impl FreeBindings for Literal {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        self.next.free_bindings()
    }
}

impl FreeBindings for Op {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        self.next.free_bindings()
    }
}

impl FreeBindings for PrintI64 {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        self.next.free_bindings()
    }
}

impl FreeBindings for IfC {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        let mut bindings = self.thenc.free_bindings();
        bindings.extend(self.elsec.free_bindings());
        bindings
    }
}

impl FreeBindings for Exit {
    fn free_bindings(&self) -> Vec<ContextBinding> {
        Vec::new()
    }
}
