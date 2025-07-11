use crate::payment::{PaymentRequest, PaymentResponse};
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
