use crate::endpoint::{get_payments_summary, post_payment, post_purge_payment};
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod endpoint;
mod payment;
mod use_case;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(post_payment)
            .service(get_payments_summary)
            .service(post_purge_payment)
    })
    .bind(("0.0.0.0", 9999))?
    .run()
    .await
}
