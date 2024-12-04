use super::examples::{load_fail, load_success, Example};
use fun::parser::fun::ProgParser;
use printer::Print;
use std::fmt;

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

fn test_examples(examples: &Vec<Example>) -> Vec<ExampleResult> {
    let mut results = vec![];
    for example in examples {
        let example_contents = std::fs::read_to_string(example.source_file.clone())
            .expect("Could not load example contents");
        let parse_result = ExampleResult::new(
            example.example_name.clone(),
            ExampleType::Parse,
            parse_test(&example_contents),
        );
        results.push(parse_result);
        let reparse_result = ExampleResult::new(
            example.example_name.clone(),
            ExampleType::Reparse,
            reparse_test(&example_contents),
        );
        results.push(reparse_result);
        let typecheck_result = ExampleResult::new(
            example.example_name.clone(),
            ExampleType::Typecheck,
            typecheck_test(&example_contents),
        );
        results.push(typecheck_result)
    }
    results
}

fn test_success() -> Vec<ExampleResult> {
    let mut results = vec![];
    let typecheck_examples = load_success();
    for (example_name, example_contents) in typecheck_examples {
        let name_str = example_name
            .to_str()
            .expect("Could not load example name")
            .to_owned();
        let check_result = ExampleResult::new(
            name_str,
            ExampleType::Typecheck,
            typecheck_test(&example_contents),
        );
        results.push(check_result)
    }
    results
}

fn test_fail() -> Vec<ExampleResult> {
    let mut results = vec![];
    let fail_examples = load_fail();

    for (example_name, example_contents) in fail_examples {
        let name_str = example_name
            .to_str()
            .expect("Could not load example name")
            .to_owned();
        let check_result = ExampleResult::new(
            name_str,
            ExampleType::Typecheck,
            typecheck_fail(&example_contents),
        );
        results.push(check_result)
    }
    results
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
    pub name: String,
    pub ty: ExampleType,
    pub fail: bool,
    pub err: Option<String>,
}

impl ExampleResult {
    pub fn new(example_name: String, ty: ExampleType, result: Option<String>) -> ExampleResult {
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

pub fn run_tests(examples: &Vec<Example>) {
    let mut results = test_examples(examples);
    results.extend(test_success());
    results.extend(test_fail());
    ExampleResult::assert_success(results);
}
