mod balance_logic;
mod endpoint;
mod info;
mod payment;
mod queue;

use crate::endpoint::{post_payment, proxy_payment, proxy_payments_summary};
use crate::queue::init_queue;
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_queue();
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(proxy_payments_summary)
            .service(post_payment)
            .route("/{tail:.*}", web::to(proxy_payment))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
