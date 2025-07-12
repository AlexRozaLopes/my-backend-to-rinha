use crate::balance_logic::call_payments;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use crate::info::call_payments_summary;

pub async fn proxy_payment(req: HttpRequest, body: web::Bytes) -> impl Responder {
    println!("{:?}", req);
    println!("{:?}", body);
    call_payments(req, body).await;
    HttpResponse::Ok()
}

#[get("/payments-summary")]
pub async fn proxy_payments_summary(req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    call_payments_summary(req).await
}
