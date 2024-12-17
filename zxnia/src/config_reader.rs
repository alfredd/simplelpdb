use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub edge: ServerConfig,
    pub rte: ServerConfig
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    /// Read configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    /// Read configuration from a YAML string
    pub fn from_str(yaml_str: &str) -> Result<Self, ConfigError> {
        let config: Config = serde_yaml::from_str(yaml_str)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_str() {
        let yaml = r#"
            database:
            file: "./edge"

            edge:
            host: 0.0.0.0
            port: 50051

            rte:
            host: 0.0.0.0
            port: 50053
        "#;

        let config = Config::from_str(yaml).unwrap();
        assert_eq!(config.edge.host, "0.0.0.0");
        assert_eq!(config.edge.port, 50051);
        assert_eq!(config.rte.host, "0.0.0.0");
        assert_eq!(config.rte.port, 50053);
    }

    // You might want to add more tests here
}