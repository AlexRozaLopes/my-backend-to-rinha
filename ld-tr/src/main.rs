mod balance_logic;
mod check_health;
mod endpoint;
mod info;

use crate::check_health::start_health_checker;
use crate::endpoint::{proxy_payment, proxy_payments_summary};
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_health_checker();
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(proxy_payments_summary)
            .route("/{tail:.*}", web::to(proxy_payment))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
