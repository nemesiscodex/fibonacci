use actix_web::{App, HttpServer, get, web::{self, Json}};
use log::debug;
use serde::Serialize;

#[derive(Serialize)]
struct Fib {
    fib: u32
}

fn calculate_fib(num: u32) -> u32 {
    debug!("Calculating fib {}", num);
    if num <= 1 {
        1
    } else {
        calculate_fib(num-1) + calculate_fib(num-2)
    }
}

#[get("/fib/{num}")]
async fn fib(web::Path(num): web::Path<u32>) -> Json<Fib> {

    let body = Fib { fib: calculate_fib(num) };

    web::Json(body)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(fib)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await

}
