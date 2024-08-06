use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Fun {
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let mut new_coargs: Vec<core::syntax::Consumer> = self
            .coargs
            .into_iter()
            .map(core::syntax::Consumer::Covar)
            .collect();
        new_coargs.push(cont);
        core::syntax::Fun {
            name: self.name,
            producers: self.args.into_iter().map(|p| p.compile_opt(st)).collect(),
            consumers: new_coargs,
        }
        .into()
    }
}
