
use serde::{Serialize, Deserialize};
use tracing::{error, info, instrument};
use uuid::Uuid;
use actix_web::{Result, client::Client, get, web::{self, Json}};
use actix_web_opentelemetry::ClientExt;
use futures::try_join;


#[derive(Serialize, Deserialize)]
pub struct Fib {
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
    if num <= 1 {
        Ok(Fib { fib: 1 })
    } else {
        let f1 = call_fib(num-1);
        let f2 = call_fib(num-2);
        let (fib1, fib2) = try_join!(f1, f2)?;
        Ok(fib1 + fib2)
    }
}

#[get("/fib/{num}")]
#[instrument(fields(request_id=?Uuid::new_v4()))]
pub async fn fib(web::Path(num): web::Path<u32>) -> Result<Json<Fib>> {

    let body = calculate_fib(num).await?;

    Ok(web::Json(body))

}