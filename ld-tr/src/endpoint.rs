use actix_web::{post, HttpResponse, Responder};
use actix_web::web::Json;
use crate::payment::PaymentRequest;

#[post("/payments")]
pub async fn post_payment(req_payment: Json<PaymentRequest>) -> impl Responder {
    println!("{:?}", req_payment.clone());
    HttpResponse::Ok()
}

