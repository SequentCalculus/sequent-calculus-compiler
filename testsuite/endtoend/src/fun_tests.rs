use super::examples::{Example, ExampleResult, ExampleType};
use fun::parser::fun::ProgParser;
use printer::Print;

/// Check whether the given example parses.
fn parse_test(example_name: String, content: &str) -> ExampleResult {
    let parser = ProgParser::new();
    let res = match parser.parse(content) {
        Ok(_) => None,
        Err(err) => Some(err.to_string()),
    };
    ExampleResult::new(example_name, ExampleType::Parse, res)
}

/// Check whether the given example parses after prettyprinting it.
fn reparse_test(example_name: String, content: &str) -> ExampleResult {
    let mut example_res = ExampleResult::new(example_name, ExampleType::Reparse, None);

    let parser = ProgParser::new();
    let parsed = match parser.parse(content) {
        Ok(parsed) => parsed.print_to_string(Default::default()),
        Err(err) => {
            example_res.fail_msg = Some(err.to_string());
            return example_res;
        }
    };
    match parser.parse(&parsed) {
        Ok(_) => (),
        Err(err) => example_res.fail_msg = Some(err.to_string()),
    };
    example_res
}

fn typecheck_test(example_name: String, content: &str) -> ExampleResult {
    let parser = ProgParser::new();
    let parsed = parser.parse(content).unwrap();
    let tc_result = parsed.check();
    let res = match tc_result {
        Ok(_) => None,
        Err(err) => Some(err.to_string()),
    };
    ExampleResult::new(example_name, ExampleType::Typecheck, res)
}

fn test_example(example: &Example) -> Vec<ExampleResult> {
    let content = match std::fs::read_to_string(example.source_file.clone()) {
        Ok(content) => content,
        Err(err) => return vec![example.to_fail(err)],
    };
    vec![
        parse_test(example.example_name.clone(), &content),
        reparse_test(example.example_name.clone(), &content),
        typecheck_test(example.example_name.clone(), &content),
    ]
}

fn test_success(typecheck_examples: Vec<(String, String)>) -> Vec<ExampleResult> {
    let mut results = vec![];
    for (example_name, example_contents) in typecheck_examples {
        results.push(typecheck_test(example_name, &example_contents));
    }
    results
}

fn test_fail(fail_examples: Vec<(String, String)>) -> Vec<ExampleResult> {
    let mut results = vec![];

    for (example_name, example_contents) in fail_examples {
        let check_result = ExampleResult::new(
            example_name,
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

pub fn run_tests(
    examples: &Vec<Example>,
    success_examples: Vec<(String, String)>,
    fail_examples: Vec<(String, String)>,
) -> Vec<ExampleResult> {
    let mut results = vec![];
    for example in examples {
        results.extend(test_example(example));
    }
    results.extend(test_success(success_examples));
    results.extend(test_fail(fail_examples));
    results
}
