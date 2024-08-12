use super::super::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::{Consumer, Cut, Producer},
};
use std::rc::Rc;

impl NamingTransformation for Cut {
    fn transform(self, st: &mut TransformState) -> Cut {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            (Producer::Constructor(_ctor), _cons) => todo!("not implemented"),
            (Producer::Mu(mu), Consumer::Destructor(dest)) => Cut {
                producer: Rc::new(mu.transform(st).into()),
                consumer: Rc::new(dest.transform(st).into()),
            },
            (_prod, Consumer::Destructor(_dest)) => todo!("not implemented"),
            (prod, cons) => Cut {
                producer: Rc::new(prod.transform(st)),
                consumer: Rc::new(cons.transform(st)),
            },
        }
    }
}
/*
#[cfg(test)]
mod transform_tests {
    use super::Cut;
    use crate::naming_transformation::NamingTransformation;

    fn example_ctor() -> Cut {
        todo!("not implemented")
    }
    fn example_mu_dtor() -> Cut {
        todo!("not implemented")
    }
    fn example_dtor() -> Cut {
        todo!("not implemented")
    }
    fn example_other() -> Cut {
        todo!("not implemented")
    }

    #[test]
    fn transform_ctor() {
        let result = example_ctor().transform(&mut Default::default());
        let expected = todo!("not implemented");
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_mu_dtor() {
        let result = example_mu_dtor().transform(&mut Default::default());
        let expected = todo!("not implemented");
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_dtor() {
        let result = example_dtor().transform(&mut Default::default());
        let expected = todo!("not implemented");
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_other() {
        let result = example_other().transform(&mut Default::default());
        let expected = todo!("not implemented");
        assert_eq!(result, expected);
    }
}*/
