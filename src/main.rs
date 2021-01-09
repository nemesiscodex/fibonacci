mod config;
mod service;

use actix_web::{App, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use service::fib;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let _uninstall = config::configure_tracing();

    HttpServer::new(|| {
        App::new()
            .wrap(RequestTracing::new())
            .service(fib)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await

}
