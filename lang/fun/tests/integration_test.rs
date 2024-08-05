use fun::parser::fun::TermParser;
use test_each_file::test_each_file;

fn test(content: &str) {
    let parser = TermParser::new();
    assert!(parser.parse(content).is_ok())
}

// Rust analyzer currently displays an error, but the test works:
// Cp.: https://github.com/binary-banter/test-each-file/issues/6
test_each_file!(in "./examples" as examples => test);
