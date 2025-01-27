use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct ExampleConfig {
    pub args: Vec<String>,
    pub runs: u32,
}

impl Default for ExampleConfig {
    fn default() -> ExampleConfig {
        ExampleConfig {
            args: vec![],
            runs: 10,
        }
    }
}

impl ExampleConfig {
    pub fn from_file(path: PathBuf) -> ExampleConfig {
        let contents = std::fs::read_to_string(path).unwrap();
        basic_toml::from_str(&contents).unwrap_or_default()
    }
}
