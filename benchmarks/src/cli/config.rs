use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct Config {
    pub args: Vec<String>,
    pub test: Vec<String>,
    pub runs: u32,
    pub suite: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            args: vec![],
            test: vec![],
            runs: 10,
            suite: "custom".to_owned(),
        }
    }
}

impl Config {
    pub fn from_file(path: PathBuf) -> Config {
        let contents = std::fs::read_to_string(path).unwrap();
        basic_toml::from_str(&contents).unwrap_or_default()
    }
}
