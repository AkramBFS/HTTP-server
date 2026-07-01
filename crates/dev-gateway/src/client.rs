use std::time::Duration;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;

pub type HttpClient = Client<HttpConnector, axum::body::Body>;

/// Constructs a pre-configured connection pool client for upstream services.
pub fn build_client() -> HttpClient {
    let mut connector = HttpConnector::new();
    connector.set_nodelay(true);
    connector.enforce_http(false);
    connector.set_keepalive(Some(Duration::from_secs(60)));
    connector.set_connect_timeout(Some(Duration::from_secs(10)));

    Client::builder(TokioExecutor::new())
        .pool_max_idle_per_host(32)
        .pool_idle_timeout(Duration::from_secs(60))
        .build(connector)
}
