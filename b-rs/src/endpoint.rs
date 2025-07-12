use crate::payment::{PaymentRequest, SummaryQuery};
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder, get, post};
use crate::use_case::{payment, summary_payment};

#[post("/payments")]
pub async fn post_payment(req_payment: Json<PaymentRequest>) -> impl Responder {
    println!("{:?}", req_payment.clone());
    payment(req_payment.into_inner()).await;
    HttpResponse::Ok()
}

#[post("/purge-payments")]
pub async fn post_purge_payment() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/payments-summary")]
pub async fn get_payments_summary(path_params: Option<Query<SummaryQuery>>) -> impl Responder {
    match path_params {
        None => {summary_payment(None)}
        Some(params) => {
            println!("{:?}",params);
            summary_payment(params.into())
        }
    }.await
}
