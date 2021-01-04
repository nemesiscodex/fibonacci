use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

pub fn configure_tracing() {
    // Get filter config from environment RUST_LOG
    let env = EnvFilter::from_default_env();

    let fmt = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(env)
        .with(fmt)
        .init();
}