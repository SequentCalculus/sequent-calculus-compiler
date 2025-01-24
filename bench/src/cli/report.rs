use super::examples::Example;
use driver::paths::BENCH_REPORTS;
use plotters::{
    chart::ChartBuilder,
    coord::Shift,
    prelude::{
        BitMapBackend, DrawingArea, IntoDrawingArea, IntoFont, LineSeries, RGBColor, Rectangle,
        BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW,
    },
};
use std::{convert::Infallible, fs::read_to_string, path::PathBuf, str::FromStr};

const PLOT_RES: (u32, u32) = (640, 480);
const FONT_SIZE: u32 = 40;
const LABEL_SIZE: u32 = 20;
const NUM_X_LABELS: usize = 10;
const NUM_Y_LABELS: usize = 10;
const MARGIN: u32 = 10;

const COLOR_MEANS: RGBColor = RED;
const COLOR_STDDEV: RGBColor = BLUE;
const COLOR_MEDIAN: RGBColor = GREEN;
const COLOR_USER: RGBColor = YELLOW;
const COLOR_SYSTEM: RGBColor = CYAN;
const COLOR_MIN: RGBColor = MAGENTA;
const COLOR_MAX: RGBColor = BLACK;

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

    fn add_plot(
        &self,
        data: Vec<f64>,
        root: &DrawingArea<BitMapBackend, Shift>,
        color: RGBColor,
        label: &str,
    ) {
        let args: Vec<f64> = self.data.iter().map(|data| data.arg).collect();
        let x_min = args.iter().fold(f64::INFINITY, |a, b| a.min(*b));
        let x_max = args.iter().fold(f64::NEG_INFINITY, |a, b| a.max(*b));
        let x_range = x_min..(x_max);
        let y_min = data.iter().fold(f64::INFINITY, |a, b| a.min(*b));
        let y_max = data.iter().fold(f64::NEG_INFINITY, |a, b| a.max(*b));
        let y_range = y_min..(y_max + ((y_max - y_min) / args.len() as f64));

        let mut chart = ChartBuilder::on(root)
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
                args.into_iter().zip(data.into_iter()),
                &color,
            ))
            .unwrap()
            .label(label)
            .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color));
        chart.configure_series_labels().border_style(BLACK).draw();
    }

    fn generate_plot(&self) {
        assert!(!self.data.is_empty());
        let mut out_path = PathBuf::from(BENCH_REPORTS).join(self.benchmark.clone());
        out_path.set_extension("png");

        let root = BitMapBackend::new(&out_path, PLOT_RES).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.margin(MARGIN, MARGIN, MARGIN, MARGIN);

        let means: Vec<f64> = self.data.iter().map(|data| data.mean).collect();
        self.add_plot(means, &root, COLOR_MEANS, "mean");
        let stddevs: Vec<f64> = self.data.iter().map(|data| data.stddev).collect();
        self.add_plot(stddevs, &root, COLOR_STDDEV, "stddev");
        let medians: Vec<f64> = self.data.iter().map(|data| data.median).collect();
        self.add_plot(medians, &root, COLOR_MEDIAN, "median");
        let users: Vec<f64> = self.data.iter().map(|data| data.user).collect();
        self.add_plot(users, &root, COLOR_USER, "user");
        let systems: Vec<f64> = self.data.iter().map(|data| data.system).collect();
        self.add_plot(systems, &root, COLOR_SYSTEM, "system");
        let mins: Vec<f64> = self.data.iter().map(|data| data.min).collect();
        self.add_plot(mins, &root, COLOR_MIN, "min");
        let maxs: Vec<f64> = self.data.iter().map(|data| data.max).collect();
        self.add_plot(maxs, &root, COLOR_MAX, "max");

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
        results.generate_plot();
    }
    Ok(())
}
