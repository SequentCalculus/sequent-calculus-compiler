use std::fmt;

#[derive(Debug)]
pub enum Kind {
    Prim,
    Data,
    Codata,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Prim => f.write_str("prim"),
            Kind::Data => f.write_str("data"),
            Kind::Codata => f.write_str("codata"),
        }
    }
}

#[cfg(test)]
mod kind_tests {
    use super::Kind;

    #[test]
    fn display_prim() {
        let result = format!("{}", Kind::Prim);
        let expected = "prim";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_data() {
        let result = format!("{}", Kind::Data);
        let expected = "data";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_codata() {
        let result = format!("{}", Kind::Codata);
        let expected = "codata";
        assert_eq!(result, expected)
    }
}
