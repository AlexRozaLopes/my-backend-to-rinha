use crate::balance_logic::PAYMENT_PROCESSOR_DEFAULT;
use crate::queue::{QueueRequest, enqueue};
use actix_web::web;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, timeout};
use uuid::Uuid;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct PaymentRequest {
    #[serde(rename = "correlationId")]
    pub correlation_id: Uuid,
    pub amount: f64,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct PaymentResponse {
    #[serde(rename = "correlationId")]
    pub correlation_id: Uuid,
    pub amount: f64,
    #[serde(rename = "requestedAt")]
    pub requested_at: DateTime<Utc>,
}

impl PaymentResponse {
    pub fn new(correlation_id: Uuid, amount: f64) -> Self {
        Self {
            correlation_id,
            amount,
            requested_at: Utc::now(),
        }
    }
}

pub async fn payment(payment_req: PaymentRequest) {
    let response = PaymentResponse::new(payment_req.correlation_id, payment_req.amount);
    let client = Client::new();

    let full_url = format!("{}/payments", PAYMENT_PROCESSOR_DEFAULT.as_str());

    let result = timeout(
        Duration::from_secs(2),
        client.post(&full_url).json(&response).send(),
    )
    .await;

    match result {
        Ok(Ok(res)) => {
            println!("✅ Pagamento enviado com status: {}", res.status());
        }
        Ok(Err(e)) => {
            eprintln!("❌ Erro na requisição: {:?}", e);

            let body: web::Bytes = web::Bytes::from(serde_json::to_string(&response).unwrap());
            enqueue(QueueRequest::new(
                "POST".to_string(),
                "/payments".to_string(),
                body,
            ))
            .await
            .unwrap();
        }
        Err(_) => {
            eprintln!("⏱ Timeout! A chamada demorou mais que 6 segundos.");
            let body: web::Bytes = web::Bytes::from(serde_json::to_string(&response).unwrap());
            enqueue(QueueRequest::new(
                "POST".to_string(),
                "/payments".to_string(),
                body,
            ))
            .await
            .unwrap();
        }
    }
}
