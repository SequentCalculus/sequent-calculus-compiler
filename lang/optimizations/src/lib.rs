use axcut::syntax::{
    Arguments, ContextBinding, Def, Prog, Statement, TypeDeclaration, TypingContext, XtorSig,
    statements::{
        Call, Clause, Create, Exit, IfC, Invoke, Let, Literal, Op, PrintI64, Substitute, Switch,
    },
};
use std::rc::Rc;

pub trait Inline {
    type Target;
    fn inline(self) -> Self::Target;
}

impl Inline for Prog {
    type Target = Prog;
    fn inline(self) -> Self::Target {
        Prog {
            defs: self.defs.into_iter().map(|def| def.inline()).collect(),
            types: self.types.into_iter().map(|ty| ty.inline()).collect(),
        }
    }
}

impl Inline for Def {
    type Target = Def;
    fn inline(self) -> Self::Target {
        Def {
            name: self.name,
            context: self.context.inline(),
            body: self.body.inline(),
            used_vars: self.used_vars,
        }
    }
}

impl Inline for TypeDeclaration {
    type Target = TypeDeclaration;
    fn inline(self) -> Self::Target {
        TypeDeclaration {
            name: self.name,
            xtors: self.xtors.into_iter().map(|xtor| xtor.inline()).collect(),
        }
    }
}

impl Inline for TypingContext {
    type Target = TypingContext;
    fn inline(self) -> Self::Target {
        TypingContext {
            bindings: self
                .bindings
                .into_iter()
                .map(|bind| bind.inline())
                .collect(),
        }
    }
}

impl Inline for Statement {
    type Target = Statement;
    fn inline(self) -> Self::Target {
        match self {
            Statement::Substitute(subst) => subst.inline().into(),
            Statement::Call(call) => call.inline().into(),
            Statement::Let(lt) => lt.inline().into(),
            Statement::Switch(sw) => sw.inline().into(),
            Statement::Create(cr) => cr.inline().into(),
            Statement::Invoke(inv) => inv.inline().into(),
            Statement::Literal(lit) => lit.inline().into(),
            Statement::Op(op) => op.inline().into(),
            Statement::PrintI64(print) => print.inline().into(),
            Statement::IfC(ifc) => ifc.inline().into(),
            Statement::Exit(ex) => ex.inline().into(),
        }
    }
}

impl<T> Inline for Rc<T>
where
    T: Inline + Clone,
{
    type Target = Rc<T::Target>;
    fn inline(self) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).inline())
    }
}

impl Inline for PrintI64 {
    type Target = PrintI64;
    fn inline(self) -> Self::Target {
        PrintI64 {
            newline: self.newline,
            var: self.var,
            next: self.next.inline(),
            free_vars_next: self.free_vars_next,
        }
    }
}

impl Inline for Exit {
    type Target = Exit;
    fn inline(self) -> Self::Target {
        Exit { var: self.var }
    }
}

impl Inline for IfC {
    type Target = IfC;
    fn inline(self) -> Self::Target {
        IfC {
            sort: self.sort,
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.inline(),
            elsec: self.elsec.inline(),
        }
    }
}

impl Inline for Literal {
    type Target = Literal;
    fn inline(self) -> Self::Target {
        Literal {
            lit: self.lit,
            var: self.var,
            next: self.next.inline(),
            free_vars_next: self.free_vars_next,
        }
    }
}

impl Inline for Op {
    type Target = Op;
    fn inline(self) -> Self::Target {
        Op {
            fst: self.fst,
            snd: self.snd,
            op: self.op,
            var: self.var,
            next: self.next.inline(),
            free_vars_next: self.free_vars_next,
        }
    }
}

impl Inline for Invoke {
    type Target = Invoke;
    fn inline(self) -> Self::Target {
        Invoke {
            var: self.var,
            tag: self.tag,
            ty: self.ty,
            args: self.args.inline(),
        }
    }
}

impl Inline for Create {
    type Target = Create;
    fn inline(self) -> Self::Target {
        Create {
            var: self.var,
            ty: self.ty,
            context: self.context.map(|ctx| ctx.inline()),
            clauses: self
                .clauses
                .into_iter()
                .map(|clause| clause.inline())
                .collect(),
            free_vars_clauses: self.free_vars_clauses,
            free_vars_next: self.free_vars_next,
            next: self.next.inline(),
        }
    }
}

impl Inline for Switch {
    type Target = Switch;
    fn inline(self) -> Self::Target {
        Switch {
            var: self.var,
            ty: self.ty,
            clauses: self
                .clauses
                .into_iter()
                .map(|clause| clause.inline())
                .collect(),
            free_vars_clauses: self.free_vars_clauses,
        }
    }
}

impl Inline for Let {
    type Target = Let;
    fn inline(self) -> Self::Target {
        Let {
            var: self.var,
            ty: self.ty,
            tag: self.tag,
            args: self.args.inline(),
            next: self.next.inline(),
            free_vars_next: self.free_vars_next,
        }
    }
}

impl Inline for Substitute {
    type Target = Substitute;
    fn inline(self) -> Self::Target {
        Substitute {
            rearrange: self.rearrange,
            next: self.next.inline(),
        }
    }
}

impl Inline for Call {
    type Target = Call;
    fn inline(self) -> Self::Target {
        Call {
            label: self.label,
            args: self.args.inline(),
        }
    }
}

impl Inline for XtorSig {
    type Target = XtorSig;
    fn inline(self) -> Self::Target {
        XtorSig {
            name: self.name,
            args: self.args.inline(),
        }
    }
}

impl Inline for ContextBinding {
    type Target = ContextBinding;
    fn inline(self) -> Self::Target {
        ContextBinding {
            var: self.var,
            chi: self.chi,
            ty: self.ty,
        }
    }
}

impl Inline for Arguments {
    type Target = Arguments;
    fn inline(self) -> Self::Target {
        Arguments {
            entries: self.entries,
        }
    }
}

impl Inline for Clause {
    type Target = Clause;
    fn inline(self) -> Self::Target {
        Clause {
            xtor: self.xtor,
            context: self.context.inline(),
            body: self.body.inline(),
        }
    }
}
