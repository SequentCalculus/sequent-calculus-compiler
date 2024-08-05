use fun::parser::fun::ProgParser;
use test_each_file::test_each_file;

fn test(content: &str) {
    let parser = ProgParser::new();
    let res = match parser.parse(content) {
        Ok(_) => None,
        Err(err) => Some(err),
    };
    assert_eq!(res, None)
}

// Rust analyzer currently displays an error, but the test works:
// Cp.: https://github.com/binary-banter/test-each-file/issues/6
test_each_file!(in "./examples" as examples => test);
