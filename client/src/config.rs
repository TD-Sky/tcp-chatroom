use config::{Config as TomlConfig, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: String,
}

impl Config {
    pub fn try_read() -> Result<Self, ConfigError> {
        let mut path =
            dirs::config_dir().ok_or_else(|| ConfigError::NotFound("$XDG_DATA_HOME".to_owned()))?;
        path.push("tcp-chatroom-client/config");
        let file = File::from(path);
        let config = TomlConfig::builder().add_source(file).build()?;
        config.try_deserialize()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_try_read() {
        use super::Config;

        let config = Config::try_read();
        println!("{config:?}");
    }
}
