use driver::Driver;
use std::{fs, path::PathBuf, process::Command, str};

const EXAMPLES_PATH: &'static str = "examples";
const OUT_PATH: &'static str = "target_grk/bin/x86_64";

fn expected(example: &str) -> &str {
    match example {
        "ArithmeticExpressions" => "4",
        "FastMultiplication" => "0",
        "Lambdas" => "8",
        "LookupTree" => "0",
        "Stream" => "97133012178238",
        "EraseUnused" => "0",
        "FibonacciRecursive" => "0",
        "LazyPair" => "0",
        "MatchOptions" => "0",
        "SumRange" => "0",
        "FactorialAccumulator" => "1",
        "IterateIncrement" => "0",
        "Lists" => "4",
        "paper_examples" => "109849936008348",
        "Tuples" => "2",
        _ => panic!("Unexpected example {example}"),
    }
}
fn main() {
    let mut driver = Driver::new();
    let path = PathBuf::from(EXAMPLES_PATH);
    let path_contents = fs::read_dir(path).expect("Could not find examples");
    for path in path_contents {
        let file_path = path.expect("Could not read filename").path();
        if !(file_path.extension().expect("Could not get file extension") == "sc") {
            continue;
        }
        driver
            .compile_x86_64(&file_path, false)
            .expect("Could not compile example {file_path:?}");
    }

    let out_path = PathBuf::from(OUT_PATH);
    let out_contents = fs::read_dir(out_path).expect("Could not read out path");
    for out_path in out_contents {
        let out_file = out_path.expect("Could not read filename").path();

        let run_out = Command::new(out_file.clone())
            .output()
            .expect("Could not run {out_file:?}");
        let result = str::from_utf8(&run_out.stdout)
            .expect("Could not parse output")
            .trim();
        let example = out_file
            .file_name()
            .expect("Could not get file name for {out_file:?}")
            .to_str()
            .expect("Unexpected file name {out_file:?}");
        let expected = expected(example);
        assert_eq!(result, expected)
    }
}
