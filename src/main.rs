mod config;
mod service;

use actix_web::{App, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use service::fib;
use tracing::instrument;

#[actix_web::main]
#[instrument(fields(service.name="fibonacci"))]
async fn main() -> std::io::Result<()> {

    let _uninstall = config::configure_tracing();

    let exporter = opentelemetry_prometheus::exporter().init();
    let request_metrics = actix_web_opentelemetry::RequestMetrics::new(
        opentelemetry::global::meter("fibonacci"),
        Some(|req: &actix_web::dev::ServiceRequest| {
            req.path() == "/metrics" && req.method() == actix_web::http::Method::GET
        }),
        Some(exporter),
    );

    HttpServer::new(move || {
        App::new()
            .wrap(RequestTracing::new())
            .wrap(request_metrics.clone())
            .service(fib)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await

}
