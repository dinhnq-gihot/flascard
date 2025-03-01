use {
    crate::enums::error::{Error, Result},
    clap::Parser,
    serde::Deserialize,
    std::path::PathBuf,
};

#[derive(Clone, Debug, Deserialize)]
pub struct HttpConfig {
    pub host: String,
    pub port: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JwtBlacklistConfig {
    pub path: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub http: HttpConfig,
    pub database: DatabaseConfig,
    pub jwt_blacklist: JwtBlacklistConfig,
}

impl Config {
    pub fn from_cfg(cfg: &str) -> Result<Self> {
        toml::from_str(cfg).map_err(|e| Error::Anyhow(e.into()))
    }
}

#[derive(Parser)]
pub struct Cli {
    /// Path of the configuration file
    #[arg(short, long, default_value_t = String::from("dist/develop.toml"))]
    pub cfg: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cfg_success() {
        let cfg = r#"
            [http]
            host = "0.0.0.0"
            port = "12345"

            [database]
            url = "postgres://username:password@localhost/diesel_demo"

            [jwt_blacklist]
            path = "data/jwt_blacklist.json"
        "#;

        let config = Config::from_cfg(cfg);
        assert!(config.is_ok());
    }
}
