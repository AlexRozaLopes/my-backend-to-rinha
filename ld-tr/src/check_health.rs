use crate::balance_logic::PAYMENT_PROCESSOR_DEFAULT;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::spawn;
use tokio::time::{sleep, timeout};

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

pub static HEALTH_CHECK_DEFAULT: Lazy<Arc<CheckHealth>> =
    Lazy::new(|| Arc::new(CheckHealth::new()));
pub static HEALTH_CHECK_FALLBACK: Lazy<Arc<CheckHealth>> =
    Lazy::new(|| Arc::new(CheckHealth::new()));

#[derive(Serialize, Debug, Deserialize)]
pub struct HealthResponse {
    pub failing: bool,
}

pub async fn verify_health(srv: String) {
    let url = format!("{}/payments/service-health", srv);
    let client = Client::new();

    let result = timeout(Duration::from_secs(3), client.get(&url).send()).await;

    match result {
        Ok(Ok(res)) => match res.json::<HealthResponse>().await {
            Ok(health) => {
                if srv.eq(PAYMENT_PROCESSOR_DEFAULT.as_str()) {
                    HEALTH_CHECK_DEFAULT.set_failed(health.failing);
                } else {
                    HEALTH_CHECK_FALLBACK.set_failed(health.failing);
                }
                println!("🔍 Health status atualizado: {:?}", health);
            }
            Err(e) => {
                eprintln!("❌ Erro ao parsear JSON do health: {:?}", e);
                if srv.eq(PAYMENT_PROCESSOR_DEFAULT.as_str()) {
                    HEALTH_CHECK_DEFAULT.set_failed(true);
                } else {
                    HEALTH_CHECK_FALLBACK.set_failed(true);
                }
            }
        },
        Ok(Err(e)) => {
            eprintln!("❌ Erro na requisição de health: {:?}", e);
            if srv.eq(PAYMENT_PROCESSOR_DEFAULT.as_str()) {
                HEALTH_CHECK_DEFAULT.set_failed(true);
            } else {
                HEALTH_CHECK_FALLBACK.set_failed(true);
            }
        }
        Err(_) => {
            eprintln!("⏰ Timeout: serviço {} demorou mais de 3s para responder", srv);
            if srv.eq(PAYMENT_PROCESSOR_DEFAULT.as_str()) {
                HEALTH_CHECK_DEFAULT.set_failed(true);
            } else {
                HEALTH_CHECK_FALLBACK.set_failed(true);
            }
        }
    }
}

pub fn start_health_checker(srv: String) {
    spawn(async move {
        loop {
            verify_health(srv.clone()).await;
            sleep(Duration::from_secs(5)).await;
        }
    });
}
