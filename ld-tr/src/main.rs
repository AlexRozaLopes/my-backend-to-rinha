mod balance_logic;
mod check_health;
mod endpoint;
mod info;
mod payment;
mod queue;

use crate::balance_logic::{PAYMENT_PROCESSOR_DEFAULT, PAYMENT_PROCESSOR_FALLBACK};
use crate::check_health::start_health_checker;
use crate::endpoint::{post_payment, post_purge_payment, proxy_payments_summary};
use crate::queue::init_queue;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_health_checker(PAYMENT_PROCESSOR_DEFAULT.to_string());
    start_health_checker(PAYMENT_PROCESSOR_FALLBACK.to_string());
    init_queue();
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(proxy_payments_summary)
            .service(post_payment)
            .service(post_purge_payment)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
