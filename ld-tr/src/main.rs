mod endpoint;
mod payment;
mod use_case;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use crate::endpoint::post_payment;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new().service(post_payment))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}