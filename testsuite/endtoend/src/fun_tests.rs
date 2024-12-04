use fun::parser::fun::ProgParser;
use printer::Print;

use std::{
    ffi::OsString,
    fmt,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

fn load_examples() -> Vec<(Box<OsString>, String)> {
    let dir = PathBuf::from("../../examples/");
    let mut examples = vec![];
    for example in read_dir(dir).expect("Could not load examples") {
        let path = example.expect("Could not load example").path();
        let example_name = path
            .file_name()
            .expect("Could not load file name")
            .to_owned();
        let contents = read_to_string(path).expect("Could not read example");
        examples.push((Box::new(example_name), contents));
    }
    examples
}

fn load_success() -> Vec<(Box<OsString>, String)> {
    let dir = PathBuf::from("../../testsuite/success");
    let mut examples = vec![];
    for example in read_dir(dir).expect("Could not load test suite") {
        let path = example.expect("Could not load example").path();
        let example_name = path
            .file_name()
            .expect("Could not load file name")
            .to_owned();
        let contents = read_to_string(path).expect("Could not read example");
        examples.push((Box::new(example_name), contents));
    }
    examples
}

fn load_fail() -> Vec<(Box<OsString>, String)> {
    let dir = PathBuf::from("../../testsuite/fail_check");
    let mut examples = vec![];
    for example in read_dir(dir).expect("Could not load test suite") {
        let path = example.expect("Could not load example").path();
        let example_name = path
            .file_name()
            .expect("Could not load file name")
            .to_owned();
        let contents = read_to_string(path).expect("Could not read example");
        examples.push((Box::new(example_name), contents));
    }
    examples
}

/// Check whether the given example parses.
fn parse_test(content: &str) -> Option<String> {
    let parser = ProgParser::new();
    match parser.parse(content) {
        Ok(_) => None,
        Err(err) => Some(err.to_string()),
    }
}

/// Check whether the given example parses after prettyprinting it.
fn reparse_test(content: &str) -> Option<String> {
    let parser = ProgParser::new();
    let parsed = match parser.parse(content) {
        Ok(parsed) => parsed.print_to_string(Default::default()),
        Err(err) => return Some(err.to_string()),
    };
    match parser.parse(&parsed) {
        Ok(_) => None,
        Err(err) => Some(err.to_string()),
    }
}

fn typecheck_test(content: &str) -> Option<String> {
    let parser = ProgParser::new();
    let parsed = parser.parse(content).unwrap();
    let tc_result = parsed.check();
    match tc_result {
        Ok(_) => None,
        Err(err) => Some(err.to_string()),
    }
}

fn typecheck_fail(content: &str) -> Option<String> {
    let parser = ProgParser::new();
    let parsed = parser.parse(content).unwrap();
    let tc_result = parsed.check();
    match tc_result {
        Ok(_) => Some("Example did not fail typecheck".to_owned()),
        Err(_) => None,
    }
}

enum ExampleType {
    Parse,
    Reparse,
    Typecheck,
}

impl fmt::Display for ExampleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExampleType::Parse => f.write_str("parse"),
            ExampleType::Reparse => f.write_str("reparse"),
            ExampleType::Typecheck => f.write_str("typecheck"),
        }
    }
}

struct ExampleResult {
    pub name: Box<OsString>,
    pub ty: ExampleType,
    pub fail: bool,
    pub err: Option<String>,
}

impl ExampleResult {
    pub fn new(
        example_name: Box<OsString>,
        ty: ExampleType,
        result: Option<String>,
    ) -> ExampleResult {
        ExampleResult {
            name: example_name,
            ty,
            fail: result.is_some(),
            err: result,
        }
    }

    pub fn assert_success(results: Vec<ExampleResult>) {
        let mut err_msg = "".to_owned();
        let mut assertion = true;
        for result in results {
            assertion = assertion && !result.fail;
            if result.fail {
                err_msg += &format!(
                    "Failed to {} example {:?}: {}\n",
                    result.ty,
                    result.name,
                    result.err.unwrap()
                );
            }
        }
        assert!(assertion, "{}", err_msg)
    }
}

pub fn run_tests() {
    let examples = load_examples();
    let mut results = vec![];
    for (example_name, example_contents) in examples {
        let parse_result = ExampleResult::new(
            example_name.clone(),
            ExampleType::Parse,
            parse_test(&example_contents),
        );
        results.push(parse_result);
        let reparse_result = ExampleResult::new(
            example_name.clone(),
            ExampleType::Reparse,
            reparse_test(&example_contents),
        );
        results.push(reparse_result);
        let typecheck_result = ExampleResult::new(
            example_name.clone(),
            ExampleType::Typecheck,
            typecheck_test(&example_contents),
        );
        results.push(typecheck_result)
    }
    let typecheck_examples = load_success();
    for (example_name, example_contents) in typecheck_examples {
        let check_result = ExampleResult::new(
            example_name,
            ExampleType::Typecheck,
            typecheck_test(&example_contents),
        );
        results.push(check_result)
    }
    let fail_examples = load_fail();

    for (example_name, example_contents) in fail_examples {
        let check_result = ExampleResult::new(
            example_name,
            ExampleType::Typecheck,
            typecheck_fail(&example_contents),
        );
        results.push(check_result)
    }
    ExampleResult::assert_success(results);
}
