use crate::balance_logic::{PAYMENT_PROCESSOR_DEFAULT, PAYMENT_PROCESSOR_FALLBACK};
use crate::check_health::{HEALTH_CHECK_DEFAULT, HEALTH_CHECK_FALLBACK};
use actix_web::{HttpRequest, web};
use once_cell::sync::Lazy;
use reqwest::Client;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::spawn;
use tokio::time::sleep;

#[derive(Clone,Debug)]
pub struct QueueRequest {
    pub method: String,
    pub path: String,
    pub body: web::Bytes,
}

impl QueueRequest {
    pub fn new(method: String, path: String, body: web::Bytes) -> Self {
        Self { method, path, body }
    }
}

pub static QUEUE: Lazy<Arc<Mutex<VecDeque<QueueRequest>>>> =
    Lazy::new(|| Arc::new(Mutex::new(VecDeque::new())));

pub fn request_to_queue(req: &HttpRequest, body: web::Bytes) -> QueueRequest {
    QueueRequest {
        method: req.method().to_string(),
        path: req.path().to_string(),
        body,
    }
}

pub async fn call_payments_from_queue(queue_req: &QueueRequest) {
    let client = Client::new();

    let base_url = if HEALTH_CHECK_DEFAULT.is_failed() {
        if HEALTH_CHECK_FALLBACK.is_failed() {
            QUEUE.lock().unwrap().push_back(queue_req.clone());
            return;
        }
        PAYMENT_PROCESSOR_FALLBACK.as_str()
    } else {
        PAYMENT_PROCESSOR_DEFAULT.as_str()
    };

    let full_url = format!("{}{}", base_url, queue_req.path);
    let method = queue_req
        .method
        .parse::<reqwest::Method>()
        .unwrap_or(reqwest::Method::GET);

    let request_builder = client
        .request(method, full_url)
        .body(queue_req.body.clone());

    match request_builder.send().await {
        Ok(res) => println!("✅ Sent queued request: {:?}", res.status()),
        Err(e) => eprintln!("❌ Error sending queued request: {:?}", e),
    }
}

pub fn start_queue_worker() {
    spawn(async move {
        loop {
            let maybe_req = {
                let mut queue = QUEUE.lock().unwrap();
                queue.pop_front()
            };

            if let Some(queue_req) = maybe_req {
                println!("executando queue --> {:?}", queue_req);
                call_payments_from_queue(&queue_req).await;
            } else {
                sleep(Duration::from_secs(15)).await;
            }
        }
    });
}
