use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub jwt_exp_seconds: usize,
}

impl AppConfig {
    pub fn try_from_file(path: Option<&str>) -> anyhow::Result<Self> {
        let path = path.unwrap_or("app_config.json");

        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Error when reading config file {}: {}", path, e))?;

        let config: AppConfig = serde_json::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Error when parsing config file {}: {}", path, e))?;

        Ok(config)
    }
}
