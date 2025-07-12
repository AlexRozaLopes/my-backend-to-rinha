use crate::endpoint::{get_payments_summary, post_payment};
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod endpoint;
mod payment;
mod use_case;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new().service(post_payment).service(get_payments_summary))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
