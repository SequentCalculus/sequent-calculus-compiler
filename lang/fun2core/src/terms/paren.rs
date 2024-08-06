use crate::definition::{Compile, CompileState};

impl Compile for fun::syntax::Paren {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let x = self.inner.compile(state);
        (*x).clone()
    }
}
