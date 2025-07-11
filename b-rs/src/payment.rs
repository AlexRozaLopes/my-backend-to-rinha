use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    #[serde(rename = "totalRequests")]
    pub total_requests: u64,
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub default: Stats,
    pub fallback: Stats,
}

#[derive(Deserialize, Debug)]
pub struct SummaryQuery {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
