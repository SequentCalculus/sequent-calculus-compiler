use crate::{CONFIG_PATH, errors::Error};
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EvalConfig {
    pub version_git_hashes: HashMap<String, String>,
}

impl EvalConfig {
    pub fn load() -> Result<EvalConfig, Error> {
        let config_path = PathBuf::from(CONFIG_PATH);
        let config_contents = read_to_string(&config_path)
            .map_err(|err| Error::read_conf(&config_path, err))?
            // Newlines have to be replaced because basic_toml will only parse without them
            .replace('\n', "");
        basic_toml::from_str::<EvalConfig>(&config_contents)
            .map_err(|err| Error::toml(&config_path, err))
    }
}
