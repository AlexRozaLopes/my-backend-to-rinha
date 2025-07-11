mod endpoint;
mod info;
mod balance_logic;
mod check_health;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crate::endpoint::{proxy_payment, proxy_payments_summary};
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use crate::check_health::{start_health_checker, CheckHealth};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    start_health_checker();
    pub static HEALTH_CHECK: Lazy<Arc<CheckHealth>> = Lazy::new(|| Arc::new(CheckHealth {check_is_failed: AtomicBool::new(false)}));

    dotenv().ok();
    HttpServer::new(|| App::new().service(proxy_payment).service(proxy_payments_summary))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
