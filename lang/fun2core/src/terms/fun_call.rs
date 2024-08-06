use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Fun {
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let mut new_coargs: Vec<core::syntax::Consumer> = self
            .coargs
            .iter()
            .cloned()
            .map(core::syntax::Consumer::Covar)
            .collect();
        new_coargs.push(cont);
        let new_args = self
            .args
            .iter()
            .cloned()
            .map(|p| p.compile_opt(st))
            .collect();
        core::syntax::Fun {
            name: self.name,
            producers: new_args,
            consumers: new_coargs,
        }
        .into()
    }
}
