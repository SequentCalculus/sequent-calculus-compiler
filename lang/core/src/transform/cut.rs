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
            //N (⟨K (pi ; c j ) | c⟩) = bind(pi ) [λas.bind(c j ) [λbs.⟨K (as; bs) | N (c)⟩]]
            (Producer::Constructor(_ctor), _cons) => todo!("not implemented"),
            //N (⟨μα .s | D (pi ; c j )⟩) = ⟨N (μα .s) | N (D (pi ; c j ))⟩
            (Producer::Mu(mu), Consumer::Destructor(dest)) => Cut {
                producer: Rc::new(mu.transform(st).into()),
                consumer: Rc::new(dest.transform(st).into()),
            },
            //N (⟨p | D (pi ; c j )⟩) = bind(pi ) [λas.bind(c j ) [λbs.⟨N (p) | D (as; bs)⟩]]
            (_prod, Consumer::Destructor(_dest)) => todo!("not implemented"),
            //N (⟨p | c⟩) = ⟨N (p) | N (c)⟩
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
