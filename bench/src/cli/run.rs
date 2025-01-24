use super::examples::Example;
use driver::Driver;

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long, value_name = "NAME")]
    name: Option<String>,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let examples;
    if let Some(name) = cmd.name {
        let example = Example::new(&name).expect("Could not find benchmark {name}");
        examples = vec![example];
    } else {
        examples = Example::load_examples();
    }

    let mut driver = Driver::new();

    for example in examples {
        #[cfg(target_arch = "x86_64")]
        let _ = driver.compile_x86_64(&example.example_path, false);

        #[cfg(target_arch = "aarch64")]
        let _ = driver.compile_aarch64(&example.example_path, false);

        example.run_hyperfine();
    }

    Ok(())
}
