use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct GatewayConfig {
    pub bind_host: String,
    pub bind_port: u16,
    pub django_backend_url: String,
    pub nextjs_frontend_url: String,
    pub api_prefix: String,
    pub strip_api_prefix: bool,
    pub max_body_bytes: usize,
    pub request_timeout_secs: u64,
}

impl GatewayConfig {
    /// Loads gateway-specific settings from the environment with robust defaults.
    pub fn load() -> Result<Self, String> {
        let _ = dotenv().ok();

        let bind_host = env::var("GATEWAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        
        let bind_port_raw = env::var("GATEWAY_PORT").unwrap_or_else(|_| "8080".to_string());
        let bind_port = bind_port_raw
            .parse::<u16>()
            .map_err(|e| format!("Invalid GATEWAY_PORT parameter ({}): {}", bind_port_raw, e))?;
            
        let django_backend_url = env::var("DJANGO_BACKEND_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
            
        let nextjs_frontend_url = env::var("NEXTJS_FRONTEND_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
            
        let api_prefix = env::var("API_PREFIX").unwrap_or_else(|_| "/api".to_string());
        
        let strip_api_prefix_raw = env::var("STRIP_API_PREFIX").unwrap_or_else(|_| "true".to_string());
        let strip_api_prefix = strip_api_prefix_raw
            .parse::<bool>()
            .map_err(|e| format!("Invalid STRIP_API_PREFIX parameter ({}): {}", strip_api_prefix_raw, e))?;
            
        let max_body_bytes_raw = env::var("MAX_BODY_BYTES").unwrap_or_else(|_| "10485760".to_string());
        let max_body_bytes = max_body_bytes_raw
            .parse::<usize>()
            .map_err(|e| format!("Invalid MAX_BODY_BYTES parameter ({}): {}", max_body_bytes_raw, e))?;
            
        let request_timeout_secs_raw = env::var("REQUEST_TIMEOUT_SECS").unwrap_or_else(|_| "30".to_string());
        let request_timeout_secs = request_timeout_secs_raw
            .parse::<u64>()
            .map_err(|e| format!("Invalid REQUEST_TIMEOUT_SECS parameter ({}): {}", request_timeout_secs_raw, e))?;

        Ok(Self {
            bind_host,
            bind_port,
            django_backend_url,
            nextjs_frontend_url,
            api_prefix,
            strip_api_prefix,
            max_body_bytes,
            request_timeout_secs,
        })
    }
}
