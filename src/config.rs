use std::net::SocketAddr;

use opentelemetry_otlp::Uninstall;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use opentelemetry::{KeyValue, global, sdk::{Resource, propagation::TraceContextPropagator, trace}};
use tracing_gelf::Logger;


pub fn configure_tracing() -> Uninstall 
    {
    // Get filter config from environment RUST_LOG
    let env = EnvFilter::from_default_env();

    global::set_text_map_propagator(TraceContextPropagator::new());
    let (tracer, uninstall) = opentelemetry_otlp::new_pipeline()
        .with_endpoint("http://localhost:4317")
        .with_trace_config(
            trace::config()
                .with_resource(
                    Resource::new(vec![KeyValue::new("service.name", "fibonacci")])
                )
        )
        .install()
        .expect("pipeline install error");
    

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer()
        .with_tracer(tracer);

    let formatting_layer = 
        BunyanFormattingLayer::new("fibonacci".into(), std::io::stdout);

    // Graylog address
    let address: SocketAddr = "127.0.0.1:12201".parse().unwrap();
    // Start tracing
    let (gelf_logger, gelf_task) = Logger::builder()
        .connect_udp(address).unwrap();

    actix_web::rt::spawn(gelf_task);

    tracing_subscriber::registry()
        .with(env)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(gelf_logger)
        .init();

    uninstall
}