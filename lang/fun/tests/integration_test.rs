use fun::{parser::fun::ProgParser, typing::check::check_module};
use printer::Print;
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
        Ok(parsed) => Ok(parsed.print_to_string(Default::default())),
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

fn typecheck_test(content: &str) {
    let parser = ProgParser::new();
    let parsed = parser.parse(content).unwrap();
    let tc_result = check_module(&parsed);
    assert!(tc_result.is_ok())
}

fn typecheck_fail(content: &str) {
    let parser = ProgParser::new();
    let parsed = parser.parse(content).unwrap();
    let tc_result = check_module(&parsed);
    assert!(tc_result.is_err())
}

// Rust analyzer currently displays an error, but the test works:
// Cp.: https://github.com/binary-banter/test-each-file/issues/6
test_each_file!(in "./examples" as parse_examples => parse_test);
test_each_file!(in "./examples" as reparse_examples => reparse_test);
test_each_file!(in "./examples" as typecheck_examples => typecheck_test);
test_each_file!(in "./testsuite/success" as typecheck_success => typecheck_test);
test_each_file!(in "./testsuite/fail_check" as typecheck_fail => typecheck_fail);
