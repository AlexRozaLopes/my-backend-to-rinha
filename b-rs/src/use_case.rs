use crate::payment::{PaymentRequest, PaymentResponse, Report, SummaryQuery};
use actix_web::web::Query;
use actix_web::{HttpResponse, Responder};
use once_cell::sync::Lazy;
use reqwest::Client;
use std::env;

static LOAD_BALANCE: Lazy<String> =
    Lazy::new(|| env::var("LOAD_BALANCE").expect("LOAD_BALANCE url not set"));

pub async fn payment(payment_req: PaymentRequest) {
    let response = PaymentResponse::new(payment_req.correlation_id, payment_req.amount);
    let url = format!("{}/payments", *LOAD_BALANCE);
    let client = Client::new();
    client.post(&url).json(&response).send().await.unwrap();
}

pub async fn summary_payment(query: Option<Query<SummaryQuery>>) -> impl Responder {
    let url = format!("{}/payments-summary", *LOAD_BALANCE);
    let client = Client::new();

    match query {
        None => {
            let report = client
                .get(url)
                .send()
                .await
                .unwrap()
                .json::<Report>()
                .await
                .unwrap();
            HttpResponse::Ok().json(report)
        }
        Some(q) => {
            let report = client
                .get(url)
                .query(&q.into_inner())
                .send()
                .await
                .unwrap()
                .json::<Report>()
                .await
                .unwrap();
            HttpResponse::Ok().json(report)
        }
    }
}
