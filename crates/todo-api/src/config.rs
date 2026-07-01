use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub environment: String,
}

impl AppConfig {
    /// Evaluates system environment parameters and falls back to default settings when missing.
    pub fn load() -> Result<Self, String> {
        // Evaluate the presence of local .env profiles, continuing on failures in production.
        let _ = dotenv().ok();

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port_raw = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
        let server_port = server_port_raw
            .parse::<u16>()
            .map_err(|e| format!("Invalid SERVER_PORT parameter ({}): {}", server_port_raw, e))?;
        let environment = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

        Ok(Self {
            server_host,
            server_port,
            environment,
        })
    }
}
