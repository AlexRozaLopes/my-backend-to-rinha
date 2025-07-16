use crate::check_health::{HEALTH_CHECK_DEFAULT, HEALTH_CHECK_FALLBACK};
use crate::info::{PAYMENT_PROCESSOR_DEFAULT, PAYMENT_PROCESSOR_FALLBACK};
use crate::queue::enqueue_payment;
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

    let base_url = if HEALTH_CHECK_DEFAULT.is_failed() {
        if HEALTH_CHECK_FALLBACK.is_failed() {
            let _ = enqueue_payment(response.clone()).await;
            return;
        }
        PAYMENT_PROCESSOR_FALLBACK.as_str()
    } else {
        PAYMENT_PROCESSOR_DEFAULT.as_str()
    };

    let full_url = format!("{}/payments", base_url);

    let result = timeout(
        Duration::from_secs(1),
        client.post(&full_url).json(&response).send(),
    )
    .await;

    match result {
        Ok(Ok(res)) => {
            println!("✅ Pagamento enviado com status: {}", res.status());
            if !res.status().is_success() {
                let _ = enqueue_payment(response.clone()).await;
            }
        }
        Ok(Err(e)) => {
            eprintln!("❌ Erro na requisição: {:?}", e);

            let _ = enqueue_payment(response.clone()).await;
        }
        Err(_) => {
            eprintln!("⏱ Timeout! A chamada demorou mais que 1 segundos.");
            let _ = enqueue_payment(response.clone()).await;
        }
    }
}
