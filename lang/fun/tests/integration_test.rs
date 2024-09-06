use fun::parser::fun::ProgParser;
use test_each_file::test_each_file;

/// Check whether the given example parses.
fn parse_test(content: &str) {
    let parser = ProgParser::new();
    let res = match parser.parse(content) {
        Ok(_) => None,
        Err(err) => Some(err),
    };
    assert_eq!(res, None)
}

/// Check whether the given example parses after prettyprinting it.
fn reparse_test(content: &str) {
    let parser = ProgParser::new();
    let parsed = match parser.parse(content) {
        Ok(parsed) => Ok(format!("{}", parsed)),
        Err(err) => Err(err),
    };
    let res = match &parsed {
        Ok(pretty) => match parser.parse(pretty) {
            Ok(_) => None,
            Err(err) => Some(err),
        },
        Err(err) => Some(err.clone()),
    };
    assert_eq!(res, None)
}

// Rust analyzer currently displays an error, but the test works:
// Cp.: https://github.com/binary-banter/test-each-file/issues/6
test_each_file!(in "./examples" as parse_examples => parse_test);
test_each_file!(in "./examples" as reparse_examples => reparse_test);
