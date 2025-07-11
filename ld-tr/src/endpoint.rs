use actix_web::{HttpRequest, HttpResponse, Responder, post, web, get};
use crate::balance_logic::call_payments;

#[post("/payments")]
pub async fn proxy_payment(req: HttpRequest, body: web::Bytes) -> impl Responder {
    println!("{:?}", req);
    println!("{:?}", body);
    call_payments(req, body).await;
    HttpResponse::Ok()
}

#[get("/payments-summary")]
pub async fn proxy_payments_summary(req: HttpRequest, body: web::Bytes) -> impl Responder {
    println!("{:?}", req);
    println!("{:?}", body);
    HttpResponse::Ok()
}
