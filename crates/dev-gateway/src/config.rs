use dotenvy::dotenv;
use serde::Deserialize;
use std::{env, fs, path::PathBuf};

const DEFAULT_CONFIG_PATH: &str = "gateway.toml";

#[derive(Clone, Debug)]
pub struct GatewayConfig {
    pub bind_host: String,
    pub bind_port: u16,
    pub routes: Vec<RouteConfig>,
    pub max_body_bytes: usize,
    pub request_timeout_secs: u64,
    pub retry: RetryConfig,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteConfig {
    pub path: String,
    pub target: String,
    pub strip_prefix: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetryConfig {
    pub attempts: usize,
    pub backoff_ms: u64,
}

#[derive(Debug, Deserialize)]
struct FileGatewayConfig {
    server: Option<FileServerConfig>,
    limits: Option<FileLimitConfig>,
    retries: Option<FileRetryConfig>,
    routes: Vec<FileRouteConfig>,
}

#[derive(Debug, Deserialize)]
struct FileServerConfig {
    host: Option<String>,
    port: Option<u16>,
}

#[derive(Debug, Deserialize)]
struct FileLimitConfig {
    max_body_bytes: Option<usize>,
    request_timeout_secs: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct FileRetryConfig {
    attempts: Option<usize>,
    backoff_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct FileRouteConfig {
    path: String,
    target: String,
    #[serde(default)]
    strip_prefix: bool,
}

impl GatewayConfig {
    /// Loads gateway-specific settings. A TOML file can define arbitrary routes,
    /// while the older env-driven Next.js/Django layout remains as a fallback.
    pub fn load() -> Result<Self, String> {
        let _ = dotenv().ok();

        let explicit_config_path = env::var("GATEWAY_CONFIG").ok().map(PathBuf::from);
        let config_path = explicit_config_path
            .clone()
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));

        if explicit_config_path.is_some() {
            return Self::load_from_file(config_path);
        }

        if config_path.exists() {
            return Self::load_from_file(config_path);
        }

        Self::load_from_env()
    }

    fn load_from_file(config_path: PathBuf) -> Result<Self, String> {
        let raw = fs::read_to_string(&config_path).map_err(|e| {
            format!(
                "Failed to read gateway config file {}: {}",
                config_path.display(),
                e
            )
        })?;

        let file_config = toml::from_str::<FileGatewayConfig>(&raw).map_err(|e| {
            format!(
                "Failed to parse gateway config file {}: {}",
                config_path.display(),
                e
            )
        })?;

        let server = file_config.server.unwrap_or(FileServerConfig {
            host: None,
            port: None,
        });
        let limits = file_config.limits.unwrap_or(FileLimitConfig {
            max_body_bytes: None,
            request_timeout_secs: None,
        });
        let retries = file_config.retries.unwrap_or(FileRetryConfig {
            attempts: None,
            backoff_ms: None,
        });

        let routes = file_config
            .routes
            .into_iter()
            .map(|route| RouteConfig {
                path: route.path,
                target: normalize_target_url(&route.target),
                strip_prefix: route.strip_prefix,
            })
            .collect::<Vec<_>>();

        let config = Self {
            bind_host: server.host.unwrap_or_else(|| "127.0.0.1".to_string()),
            bind_port: server.port.unwrap_or(8080),
            routes,
            max_body_bytes: limits.max_body_bytes.unwrap_or(10_485_760),
            request_timeout_secs: limits.request_timeout_secs.unwrap_or(30),
            retry: RetryConfig {
                attempts: retries.attempts.unwrap_or(3),
                backoff_ms: retries.backoff_ms.unwrap_or(50),
            },
        };

        config.validate()?;
        Ok(config)
    }

    fn load_from_env() -> Result<Self, String> {
        let _ = dotenv().ok();

        let bind_host = env::var("GATEWAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let bind_port_raw = env::var("GATEWAY_PORT").unwrap_or_else(|_| "8080".to_string());
        let bind_port = bind_port_raw
            .parse::<u16>()
            .map_err(|e| format!("Invalid GATEWAY_PORT parameter ({}): {}", bind_port_raw, e))?;

        let django_backend_url =
            env::var("DJANGO_BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());

        let nextjs_frontend_url =
            env::var("NEXTJS_FRONTEND_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());

        let api_prefix = env::var("API_PREFIX").unwrap_or_else(|_| "/api".to_string());

        let strip_api_prefix_raw =
            env::var("STRIP_API_PREFIX").unwrap_or_else(|_| "true".to_string());
        let strip_api_prefix = strip_api_prefix_raw.parse::<bool>().map_err(|e| {
            format!(
                "Invalid STRIP_API_PREFIX parameter ({}): {}",
                strip_api_prefix_raw, e
            )
        })?;

        let max_body_bytes_raw =
            env::var("MAX_BODY_BYTES").unwrap_or_else(|_| "10485760".to_string());
        let max_body_bytes = max_body_bytes_raw.parse::<usize>().map_err(|e| {
            format!(
                "Invalid MAX_BODY_BYTES parameter ({}): {}",
                max_body_bytes_raw, e
            )
        })?;

        let request_timeout_secs_raw =
            env::var("REQUEST_TIMEOUT_SECS").unwrap_or_else(|_| "30".to_string());
        let request_timeout_secs = request_timeout_secs_raw.parse::<u64>().map_err(|e| {
            format!(
                "Invalid REQUEST_TIMEOUT_SECS parameter ({}): {}",
                request_timeout_secs_raw, e
            )
        })?;

        let retry_attempts_raw = env::var("RETRY_ATTEMPTS").unwrap_or_else(|_| "3".to_string());
        let retry_attempts = retry_attempts_raw.parse::<usize>().map_err(|e| {
            format!(
                "Invalid RETRY_ATTEMPTS parameter ({}): {}",
                retry_attempts_raw, e
            )
        })?;

        let retry_backoff_ms_raw =
            env::var("RETRY_BACKOFF_MS").unwrap_or_else(|_| "50".to_string());
        let retry_backoff_ms = retry_backoff_ms_raw.parse::<u64>().map_err(|e| {
            format!(
                "Invalid RETRY_BACKOFF_MS parameter ({}): {}",
                retry_backoff_ms_raw, e
            )
        })?;

        let config = Self {
            bind_host,
            bind_port,
            routes: vec![
                RouteConfig {
                    path: api_prefix,
                    target: normalize_target_url(&django_backend_url),
                    strip_prefix: strip_api_prefix,
                },
                RouteConfig {
                    path: "/".to_string(),
                    target: normalize_target_url(&nextjs_frontend_url),
                    strip_prefix: false,
                },
            ],
            max_body_bytes,
            request_timeout_secs,
            retry: RetryConfig {
                attempts: retry_attempts,
                backoff_ms: retry_backoff_ms,
            },
        };

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if self.routes.is_empty() {
            return Err("At least one route must be configured".to_string());
        }

        if self.retry.attempts == 0 {
            return Err("Retry attempts must be at least 1".to_string());
        }

        for route in &self.routes {
            if !route.path.starts_with('/') {
                return Err(format!(
                    "Route path '{}' is invalid: paths must start with '/'",
                    route.path
                ));
            }

            let uri = route.target.parse::<hyper::Uri>().map_err(|e| {
                format!("Route target '{}' is not a valid URI: {}", route.target, e)
            })?;

            if uri.scheme().is_none() || uri.authority().is_none() {
                return Err(format!(
                    "Route target '{}' must include a scheme and host",
                    route.target
                ));
            }
        }

        Ok(())
    }
}

fn normalize_target_url(target: &str) -> String {
    target.trim_end_matches('/').to_string()
}
