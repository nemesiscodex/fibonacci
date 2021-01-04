use actix_web::{App, HttpServer, client::Client, get, web::{self, Json}, Result};
use log::debug;
use serde::{Serialize, Deserialize};

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

async fn call_fib(num: u32) -> Result<Fib> {
    let mut result = Client::new()
        .get(format!("http://localhost:3000/fib/{}", num))
        .send()
        .await?;

    let fib_result: Fib = result.json().await?;

    Ok(fib_result)
}
 
async fn calculate_fib(num: u32) -> Result<Fib> {
    debug!("Calculating fib {}", num);
    if num <= 1 {
        Ok(Fib { fib: 1 })
    } else {
        Ok(call_fib(num-1).await? + call_fib(num-2).await?)
    }
}

#[get("/fib/{num}")]
async fn fib(web::Path(num): web::Path<u32>) -> Result<Json<Fib>> {

    let body = calculate_fib(num).await?;

    Ok(web::Json(body))

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
