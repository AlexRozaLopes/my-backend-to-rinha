use crate::balance_logic::call_payments;
use crate::info::call_payments_summary;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web, post};
use actix_web::web::Json;
use crate::payment::{payment, PaymentRequest};

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

#[post("/payments")]
pub async fn post_payment(req_payment: Json<PaymentRequest>) -> impl Responder {
    payment(req_payment.into_inner()).await;
    HttpResponse::Ok()
}
