use actix_web::{HttpRequest, HttpResponse, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::balance_logic::{PAYMENT_PROCESSOR_DEFAULT, PAYMENT_PROCESSOR_FALLBACK};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    #[serde(rename = "totalRequests")]
    pub total_requests: u64,
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
}

impl Stats {
    pub fn new(total_requests: u64, total_amount: f64) -> Self {
        Self { total_requests, total_amount }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub default: Stats,
    pub fallback: Stats,
}

impl Report {
    pub fn new(default: Stats, fallback: Stats) -> Self {
        Self { default, fallback }
    }
}

pub async fn call_payments_summary(req: HttpRequest) -> impl Responder {
    let client = Client::new();

    let default_url = format!("{}/admin{}", *PAYMENT_PROCESSOR_DEFAULT, req.uri());
    let fallback_url = format!("{}/admin{}", *PAYMENT_PROCESSOR_FALLBACK, req.uri());

    let method = req.method().to_string().parse::<reqwest::Method>().unwrap();
    let mut request_builder_df = client.request(method.clone(), default_url);
    let mut request_builder_fb = client.request(method, fallback_url);

    for (key, value) in req.headers().iter() {
        if let Ok(val_str) = value.to_str() {
            request_builder_df = request_builder_df.header(key.as_str(), val_str);
            request_builder_fb = request_builder_fb.header(key.as_str(), val_str);
        }
    }

    request_builder_df = request_builder_df.header("X-Rinha-Token","123");
    request_builder_fb = request_builder_fb.header("X-Rinha-Token","123");

    let info_df = request_builder_df.send().await.unwrap().json::<Stats>().await.unwrap_or_else(|_| Stats::new(0, 0f64));
    let info_fb = request_builder_fb.send().await.unwrap().json::<Stats>().await.unwrap_or_else(|_| Stats::new(0, 0f64));

    let report = Report::new(info_df, info_fb);

    HttpResponse::Ok().json(report)
}
