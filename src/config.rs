use opentelemetry_jaeger::Uninstall;
use tracing_opentelemetry::{OpenTelemetryLayer};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use opentelemetry::{global, sdk::{propagation::TraceContextPropagator}};

pub fn configure_tracing() -> Uninstall {
    // Get filter config from environment RUST_LOG
    let env = EnvFilter::from_default_env();

    global::set_text_map_propagator(TraceContextPropagator::new());
    let (tracer, uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name("fibonacci")
        .install()
        .expect("pipeline install error");
    

    // Create a tracing layer with the configured tracer
    let telemetry = OpenTelemetryLayer::new(tracer);

    let formatting_layer = 
        BunyanFormattingLayer::new("fibonacci".into(), std::io::stdout);

    tracing_subscriber::registry()
        .with(env)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();

    uninstall
}