mod config;

use actix_web::{App, HttpServer, Result, client::Client, get, web::{self, Json}};
use serde::{Serialize, Deserialize};
use tracing::{error, info, instrument};
use actix_web_opentelemetry::{ClientExt, RequestTracing};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Fib {
    fib: u32
}

impl std::ops::Add for Fib {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Fib { fib: self.fib + rhs.fib }
    }
}

#[instrument]
async fn call_fib(num: u32) -> Result<Fib> {
    let mut result = Client::new()
        .get(format!("http://localhost:3000/fib/{}", num))
        .trace_request()
        .send()
        .await?;

    let fib_result: Fib = result.json().await?;

    Ok(fib_result)
}
 
#[instrument]
async fn calculate_fib(num: u32) -> Result<Fib> {
    info!("Calculating fib {}", num);
    if num == 4 {
        error!("Cannot calculate fibonacci 4");
        Err(actix_web::error::ErrorRequestTimeout(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")))
    } else if num <= 1 {
        Ok(Fib { fib: 1 })
    } else {
        Ok(call_fib(num-1).await? + call_fib(num-2).await?)
    }
}

#[get("/fib/{num}")]
#[instrument(fields(request_id=?Uuid::new_v4()))]
async fn fib(web::Path(num): web::Path<u32>) -> Result<Json<Fib>> {

    let body = calculate_fib(num).await?;

    Ok(web::Json(body))

}

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
