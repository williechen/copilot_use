//! 本機 OTLP traces/metrics/logs：嚴格 loopback，無 payload/secret/私人 path。
use opentelemetry::{
    KeyValue, global,
    logs::{LogRecord, Logger, LoggerProvider, Severity},
    propagation::{Extractor, Injector},
    trace::{FutureExt, Span, Status, TraceContextExt, Tracer},
};
use opentelemetry_otlp::{Protocol, WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::{
    Resource, logs::SdkLoggerProvider, metrics::SdkMeterProvider,
    propagation::TraceContextPropagator, trace::SdkTracerProvider,
};
use std::{collections::HashMap, sync::OnceLock, time::Duration};
static LOGS: OnceLock<SdkLoggerProvider> = OnceLock::new();
pub struct Telemetry {
    traces: Option<SdkTracerProvider>,
    metrics: Option<SdkMeterProvider>,
    logs: Option<SdkLoggerProvider>,
}
impl Drop for Telemetry {
    fn drop(&mut self) {
        if let Some(p) = &self.traces {
            let _ = p.shutdown();
        }
        if let Some(p) = &self.metrics {
            let _ = p.shutdown();
        }
        if let Some(p) = &self.logs {
            let _ = p.shutdown();
        }
    }
}
fn local_endpoint(value: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let url = url::Url::parse(value)?;
    let local = match url.host() {
        Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
        Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
        _ => false,
    };
    if !local
        || !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("觀測目的地只接受不含憑證的本機 loopback URL".into());
    }
    Ok(value.trim_end_matches('/').into())
}
pub fn init(service: &'static str) -> Result<Telemetry, Box<dyn std::error::Error + Send + Sync>> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let Ok(base) = std::env::var("COPILOT_LOCAL_OTLP_ENDPOINT") else {
        return Ok(Telemetry {
            traces: None,
            metrics: None,
            logs: None,
        });
    };
    let base = local_endpoint(&base)?;
    let resource = Resource::builder_empty()
        .with_service_name(service)
        .with_attributes([KeyValue::new("service.version", env!("CARGO_PKG_VERSION"))])
        .build();
    // 明確 endpoint/空 headers 覆寫 OTEL_*；外部 exporter 設定不會被採用。
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpJson)
        .with_endpoint(format!("{base}/v1/traces"))
        .with_headers(HashMap::new())
        .with_timeout(Duration::from_secs(3))
        .build()?;
    let traces = SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(exporter)
        .build();
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpJson)
        .with_endpoint(format!("{base}/v1/metrics"))
        .with_headers(HashMap::new())
        .with_timeout(Duration::from_secs(3))
        .build()?;
    let metrics = SdkMeterProvider::builder()
        .with_resource(resource.clone())
        .with_periodic_exporter(exporter)
        .build();
    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpJson)
        .with_endpoint(format!("{base}/v1/logs"))
        .with_headers(HashMap::new())
        .with_timeout(Duration::from_secs(3))
        .build()?;
    let logs = SdkLoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build();
    global::set_tracer_provider(traces.clone());
    global::set_meter_provider(metrics.clone());
    let _ = LOGS.set(logs.clone());
    Ok(Telemetry {
        traces: Some(traces),
        metrics: Some(metrics),
        logs: Some(logs),
    })
}
struct Headers<'a>(&'a axum::http::HeaderMap);
impl Extractor for Headers<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.to_str().ok())
    }
    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}
struct Carrier(HashMap<String, String>);
impl Injector for Carrier {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(key.into(), value);
    }
}
pub fn outgoing_headers() -> HashMap<String, String> {
    let mut carrier = Carrier(HashMap::new());
    global::get_text_map_propagator(|p| {
        p.inject_context(&opentelemetry::Context::current(), &mut carrier)
    });
    carrier.0
}
pub fn event(operation: &'static str, status: u16) {
    let attributes = [
        KeyValue::new("operation", operation),
        KeyValue::new("status", i64::from(status)),
    ];
    global::meter("copilot")
        .u64_counter("copilot.operations")
        .build()
        .add(1, &attributes);
    if let Some(provider) = LOGS.get() {
        let logger = provider.logger("copilot");
        let mut record = logger.create_log_record();
        record.set_body(operation.into());
        record.set_severity_number(Severity::Info);
        record.add_attribute("status", i64::from(status));
        logger.emit(record);
    }
}
pub async fn http(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let route = request
        .extensions()
        .get::<axum::extract::MatchedPath>()
        .map(|p| p.as_str().to_owned())
        .unwrap_or_else(|| "unmatched".into());
    let parent = global::get_text_map_propagator(|p| p.extract(&Headers(request.headers())));
    let mut span = global::tracer("copilot.http").start_with_context("http.request", &parent);
    span.set_attribute(KeyValue::new("http.route", route));
    span.set_attribute(KeyValue::new(
        "http.request.method",
        request.method().to_string(),
    ));
    let context = parent.with_span(span);
    let response = next.run(request).with_context(context.clone()).await;
    let status = response.status().as_u16();
    context.span().set_attribute(KeyValue::new(
        "http.response.status_code",
        i64::from(status),
    ));
    if status >= 500 {
        context.span().set_status(Status::error("service_error"));
    }
    context.span().end();
    let _guard = context.attach();
    event("http.request", status);
    response
}
pub async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("signal handler");
        tokio::select! {_=tokio::signal::ctrl_c()=>{},_=terminate.recv()=>{}}
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}
pub async fn observe<T, E>(
    operation: &'static str,
    future: impl std::future::Future<Output = Result<T, E>>,
) -> Result<T, E> {
    let span = global::tracer("copilot.operations").start(operation);
    let context = opentelemetry::Context::current().with_span(span);
    let result = future.with_context(context.clone()).await;
    let status = if result.is_ok() { 200 } else { 500 };
    if status >= 500 {
        context.span().set_status(Status::error("operation_failed"));
    }
    context.span().end();
    event(operation, status);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_remote_or_secret_endpoint() {
        assert!(local_endpoint("https://example.com").is_err());
        assert!(local_endpoint("http://key@127.0.0.1:4318").is_err());
        assert!(local_endpoint("http://127.0.0.1:4318").is_ok());
    }
}
