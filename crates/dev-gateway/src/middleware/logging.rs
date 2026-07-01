use axum::response::Response;
use hyper::Request;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};
use tower::{Layer, Service};

/// Tower layer that wraps services with request/response logging.
#[derive(Clone)]
pub struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingMiddleware { inner }
    }
}

/// Middleware service that records request start time and logs a single
/// structured line to stdout upon response completion.
#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for LoggingMiddleware<S>
where
    S: Service<Request<B>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: std::fmt::Display + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let method = req.method().clone();
        let original_uri = req.uri().to_string();
        let start = Instant::now();

        // Clone inner service per Tower best practices for async call.
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let result = inner.call(req).await;

            let latency = start.elapsed();
            let timestamp = chrono_now_utc();

            match &result {
                Ok(response) => {
                    let status = response.status().as_u16();
                    println!(
                        "{} {} {} | STATUS: {} | LATENCY: {:.2?}",
                        timestamp, method, original_uri, status, latency,
                    );
                }
                Err(err) => {
                    println!(
                        "{} {} {} | ERROR: {} | LATENCY: {:.2?}",
                        timestamp, method, original_uri, err, latency,
                    );
                }
            }

            result
        })
    }
}

/// Returns the current UTC timestamp in a compact ISO-8601 format without
/// pulling in the `chrono` crate — uses `std::time::SystemTime` directly.
fn chrono_now_utc() -> String {
    use std::time::SystemTime;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();

    let secs = now.as_secs();

    // Compute date/time components from unix timestamp.
    let days = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Civil date from days since epoch (algorithm from Howard Hinnant).
    let z = days as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, m, d, hours, minutes, seconds
    )
}
