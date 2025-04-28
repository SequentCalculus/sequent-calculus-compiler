use super::{end_to_end_tests::EndToEndTest, errors::Error, load_tests::AllTests};

use fun::parser::fun::ProgParser;
use printer::Print;

use std::fmt;

pub enum TestType {
    Parse,
    Reparse,
    Typecheck,
    Compile,
}

pub struct TestResult {
    pub name: String,
    pub ty: TestType,
    pub fail_msg: Option<String>,
}

impl TestResult {
    pub fn new(name: String, ty: TestType, result: Option<String>) -> TestResult {
        TestResult {
            name,
            ty,
            fail_msg: result,
        }
    }

    pub fn report(results: Vec<TestResult>) -> Result<(), Error> {
        println!("Ran {} tests", results.len());
        let mut num_success = 0;
        let mut num_fail = 0;
        for result in results {
            println!("\t{}", result);
            if result.fail_msg.is_none() {
                num_success += 1
            } else {
                num_fail += 1
            }
        }
        println!(
            "\ntest result: {} passed; {} failed\n",
            num_success, num_fail
        );
        if num_fail == 0 {
            Ok(())
        } else {
            Err(Error::TestFailure { num_fail })
        }
    }
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fail_str = "\x1B[38;2;255;0;0mfail\x1B[0m";
        let ok_str = "\x1b[38;2;0;255;0mok\x1B[0m";
        let succ = match &self.fail_msg {
            None => ok_str.to_owned(),
            Some(err) => format!("{fail_str}:\n\t\tError: {err}\n"),
        };
        write!(f, "Test: {} {} ... {}", self.ty, self.name, succ)
    }
}

impl fmt::Display for TestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestType::Parse => f.write_str("parse"),
            TestType::Reparse => f.write_str("reparse"),
            TestType::Typecheck => f.write_str("typecheck"),
            TestType::Compile => f.write_str("compile"),
        }
    }
}

/// Check whether the given test parses.
fn parse_test(name: String, content: &str) -> TestResult {
    let parser = ProgParser::new();
    let res = match parser.parse(content) {
        Ok(_) => None,
        Err(err) => Some(err.to_string()),
    };
    TestResult::new(name, TestType::Parse, res)
}

/// Check whether the given test parses after prettyprinting it.
fn reparse_test(name: String, content: &str) -> TestResult {
    let mut result = TestResult::new(name, TestType::Reparse, None);

    let parser = ProgParser::new();
    let parsed = match parser.parse(content) {
        Ok(parsed) => parsed.print_to_string(Default::default()),
        Err(err) => {
            result.fail_msg = Some(err.to_string());
            return result;
        }
    };
    match parser.parse(&parsed) {
        Ok(_) => (),
        Err(err) => result.fail_msg = Some(err.to_string()),
    };
    result
}

fn typecheck_test(name: String, content: &str) -> TestResult {
    let mut result = TestResult::new(name, TestType::Typecheck, None);

    let parser = ProgParser::new();
    let parsed = match parser.parse(content) {
        Ok(md) => md,
        Err(err) => {
            result.fail_msg = Some(err.to_string());
            return result;
        }
    };
    let tc_result = parsed.check();
    let res = match tc_result {
        Ok(_) => None,
        Err(err) => Some(err.to_string()),
    };
    result.fail_msg = res;
    result
}

fn typecheck_fail(content: &str) -> Option<String> {
    let parser = ProgParser::new();
    let parsed = parser.parse(content).unwrap();
    let tc_result = parsed.check();
    match tc_result {
        Ok(_) => Some("Test did not fail typecheck".to_owned()),
        Err(_) => None,
    }
}

fn test_end_to_end(test: &EndToEndTest) -> Vec<TestResult> {
    let content = match std::fs::read_to_string(test.source_file.clone()) {
        Ok(content) => content,
        Err(err) => return vec![test.to_fail(err)],
    };
    vec![
        parse_test(test.name.clone(), &content),
        reparse_test(test.name.clone(), &content),
        typecheck_test(test.name.clone(), &content),
    ]
}

fn test_success(success_tests: &Vec<(String, String)>) -> Vec<TestResult> {
    let mut results = vec![];
    for (test_name, test_contents) in success_tests {
        results.push(typecheck_test(test_name.clone(), &test_contents));
    }
    results
}

fn test_fail(fail_tests: &Vec<(String, String)>) -> Vec<TestResult> {
    let mut results = vec![];

    for (test_name, test_contents) in fail_tests {
        let check_result = TestResult::new(
            test_name.clone(),
            TestType::Typecheck,
            typecheck_fail(&test_contents),
        );
        results.push(check_result)
    }
    results
}

pub fn run_tests(tests: &AllTests) -> Vec<TestResult> {
    let mut results = vec![];
    for test in tests.end_to_end_tests.iter() {
        let test = test
            .iter()
            .filter(|tst| {
                tst.source_file
                    .extension()
                    .expect("Could not read test file name")
                    == "sc"
            })
            .next()
            .expect("Could not find test source");
        results.extend(test_end_to_end(test))
    }
    results.extend(test_success(&tests.success_tests));
    results.extend(test_fail(&tests.fail_tests));
    results
}
