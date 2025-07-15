use crate::info::payments_summary;
use crate::payment::{PaymentRequest, payment};
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, Responder, get, post};

#[get("/payments-summary")]
pub async fn proxy_payments_summary(req: HttpRequest) -> impl Responder {
    payments_summary(req).await
}

#[post("/payments")]
pub async fn post_payment(req_payment: Json<PaymentRequest>) -> impl Responder {
    payment(req_payment.into_inner()).await;
    HttpResponse::Ok()
}


#[post("/purge-payments")]
pub async fn post_purge_payment() -> impl Responder {
    HttpResponse::Ok()
}
