use lalrpop_util::lalrpop_mod;


lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fun, "/grammar/fun.rs"
);
