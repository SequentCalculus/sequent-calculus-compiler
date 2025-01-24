use super::examples::Example;
use std::{convert::Infallible, fs::read_to_string, path::PathBuf, str::FromStr};

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long, value_name = "NAME")]
    name: Option<String>,
}

#[derive(Debug)]
struct BenchResult {
    benchmark: String,
    data: Vec<BenchData>,
}

#[derive(Debug)]
struct BenchData {
    arg: f64,
    mean: f64,
    stddev: f64,
    median: f64,
    user: f64,
    system: f64,
    min: f64,
    max: f64,
}

impl BenchResult {
    fn from_file(file: PathBuf) -> BenchResult {
        let contents = read_to_string(&file).expect(&format!(
            "No benchmark for {}",
            file.file_name().unwrap().to_str().unwrap()
        ));
        let mut name = file;
        name.set_extension("");
        let benchmark = name.file_name().unwrap().to_str().unwrap().to_owned();

        let mut data = vec![];
        let mut lines = contents.lines();
        lines.next();
        for line in lines {
            data.push(line.parse().unwrap());
        }
        BenchResult { benchmark, data }
    }
}

impl FromStr for BenchData {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split(',');
        let mut command = fields.next().unwrap().trim().split(" ");
        command.next();
        let arg = command.next().unwrap_or_default().parse().unwrap();

        Ok(BenchData {
            arg,
            mean: fields.next().unwrap().parse::<f64>().unwrap(),
            stddev: fields.next().unwrap().parse::<f64>().unwrap(),
            median: fields.next().unwrap().parse::<f64>().unwrap(),
            user: fields.next().unwrap().parse::<f64>().unwrap(),
            system: fields.next().unwrap().parse::<f64>().unwrap(),
            min: fields.next().unwrap().parse::<f64>().unwrap(),
            max: fields.next().unwrap().parse::<f64>().unwrap(),
        })
    }
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let examples = Example::load(cmd.name);
    for example in examples {
        let results = BenchResult::from_file(example.result_path);
        println!("{results:?}");
    }
    Ok(())
}
