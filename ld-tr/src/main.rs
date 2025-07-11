mod endpoint;
mod info;
mod use_case;

use crate::endpoint::{proxy_payment, proxy_payments_summary};
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new().service(proxy_payment).service(proxy_payments_summary))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
