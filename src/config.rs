use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

pub fn configure_tracing() {
    // Get filter config from environment RUST_LOG
    let env = EnvFilter::from_default_env();

    let formatting_layer = 
        BunyanFormattingLayer::new("fibonacci".into(), std::io::stdout);

    tracing_subscriber::registry()
        .with(env)
        // .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();
}