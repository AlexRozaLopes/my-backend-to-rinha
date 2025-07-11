use crate::balance_logic::PAYMENT_PROCESSOR_DEFAULT;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::spawn;
use tokio::time::sleep;

pub struct CheckHealth {
    pub check_is_failed: AtomicBool,
}

impl CheckHealth {
    pub fn new() -> Self {
        Self {
            check_is_failed: AtomicBool::new(false),
        }
    }

    pub fn set_failed(&self, failed: bool) {
        self.check_is_failed.store(failed, Ordering::Relaxed);
    }

    pub fn is_failed(&self) -> bool {
        self.check_is_failed.load(Ordering::Relaxed)
    }
}

pub static HEALTH_CHECK: Lazy<Arc<CheckHealth>> = Lazy::new(|| Arc::new(CheckHealth::new()));

#[derive(Serialize, Debug, Deserialize)]
pub struct HealthResponse {
    pub failing: bool,
}

pub async fn verify_health() {
    let url = format!("{}/payments/service-health", *PAYMENT_PROCESSOR_DEFAULT);
    let client = Client::new();

    match client.get(&url).send().await {
        Ok(res) => match res.json::<HealthResponse>().await {
            Ok(health) => {
                HEALTH_CHECK.set_failed(health.failing);
                println!("🔍 Health status atualizado: {:?}", health);
            }
            Err(e) => {
                eprintln!("❌ Erro ao parsear JSON do health: {:?}", e);
                HEALTH_CHECK.set_failed(true);
            }
        },
        Err(e) => {
            eprintln!("❌ Erro na requisição de health: {:?}", e);
            HEALTH_CHECK.set_failed(true);
        }
    }
}

pub fn start_health_checker() {
    spawn(async move {
        loop {
            verify_health().await;
            sleep(Duration::from_secs(5)).await;
        }
    });
}
