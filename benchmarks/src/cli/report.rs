use super::benchmark::Benchmark;
use driver::paths::BENCHMARKS_REPORTS;
use plotters::{
    chart::ChartBuilder,
    prelude::{
        BitMapBackend, CandleStick, IntoDrawingArea, IntoFont, LineSeries, RGBColor, Rectangle,
        BLACK, BLUE, RED, WHITE,
    },
    style::Color,
};
use std::{
    convert::Infallible,
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
    str::FromStr,
};

const PLOT_RES: (u32, u32) = (640, 480);
const FONT_SIZE: u32 = 40;
const LABEL_SIZE: u32 = 20;
const NUM_X_LABELS: usize = 10;
const NUM_Y_LABELS: usize = 10;
const MARGIN: u32 = 10;

const COLOR_MEANS: RGBColor = RED;
const COLOR_STDDEV: RGBColor = BLUE;

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long, value_name = "NAME")]
    name: Option<String>,
}

#[derive(Debug)]
struct BenchResult {
    benchmark: String,
    data: Vec<BenchData>,
    report_path: PathBuf,
}

#[derive(Debug)]
struct BenchData {
    arg: f64,
    mean: f64,
    stddev: f64,
}

impl BenchResult {
    fn from_file(file: PathBuf, report_path: PathBuf) -> BenchResult {
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let contents = read_to_string(&file)
            .unwrap_or_else(|_| panic!("No benchmark file found for {}", file_name));
        let mut name = file;
        name.set_extension("");
        let benchmark = name.file_name().unwrap().to_str().unwrap().to_owned();

        let mut data = vec![];
        let mut lines = contents.lines();
        lines.next();
        for line in lines {
            data.push(line.parse().unwrap());
        }
        BenchResult {
            benchmark,
            data,
            report_path,
        }
    }

    fn generate_plot(&self) {
        assert!(!self.data.is_empty());
        create_dir_all(BENCHMARKS_REPORTS).unwrap();

        let root = BitMapBackend::new(&self.report_path, PLOT_RES).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.margin(MARGIN, MARGIN, MARGIN, MARGIN);

        let means: Vec<f64> = self.data.iter().map(|data| data.mean).collect();
        let stddevs: Vec<f64> = self.data.iter().map(|data| data.stddev).collect();
        let args: Vec<f64> = self.data.iter().map(|data| data.arg).collect();

        let x_min = args.iter().fold(f64::INFINITY, |a, b| a.min(*b));
        let x_max = args.iter().fold(f64::NEG_INFINITY, |a, b| a.max(*b));
        let x_range = x_min..(x_max);
        let y_min = means.iter().fold(f64::INFINITY, |a, b| a.min(*b));
        let y_max = means.iter().fold(f64::NEG_INFINITY, |a, b| a.max(*b));
        let y_range = y_min..(y_max + ((y_max - y_min) / args.len() as f64));

        let mut chart = ChartBuilder::on(&root)
            .caption(&self.benchmark, ("sans-serif", FONT_SIZE).into_font())
            .x_label_area_size(LABEL_SIZE)
            .y_label_area_size(LABEL_SIZE)
            .build_cartesian_2d(x_range, y_range)
            .unwrap();
        chart
            .configure_mesh()
            .x_labels(NUM_X_LABELS)
            .y_labels(NUM_Y_LABELS)
            .draw()
            .unwrap();
        chart
            .draw_series(LineSeries::new(
                args.iter().copied().zip(means.iter().copied()),
                &COLOR_MEANS,
            ))
            .unwrap()
            .label("mean")
            .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], COLOR_MEANS));

        chart
            .draw_series(args.iter().zip(stddevs.iter()).zip(means.iter()).map(
                |((x, y_diff), y)| {
                    CandleStick::new(
                        *x,
                        *y,
                        y + y_diff,
                        y - y_diff,
                        *y,
                        COLOR_STDDEV.filled(),
                        COLOR_STDDEV,
                        15,
                    )
                },
            ))
            .unwrap();
        chart
            .configure_series_labels()
            .border_style(BLACK)
            .draw()
            .unwrap();

        root.present().unwrap();
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
        })
    }
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let examples = Benchmark::load(cmd.name);
    for example in examples {
        if !example.results_exist() {
            println!("Skipping {}, no results found", example.name);
            continue;
        }
        let results = BenchResult::from_file(example.result_path, example.report_path);
        results.generate_plot();
        println!("generated plot for {}", example.name);
    }
    Ok(())
}
