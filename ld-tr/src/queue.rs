use crate::balance_logic::PAYMENT_PROCESSOR_DEFAULT;
use actix_web::web;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::{Duration, timeout};
#[derive(Clone, Debug)]
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

static QUEUE_SENDER: Lazy<Mutex<Option<Sender<QueueRequest>>>> = Lazy::new(|| Mutex::new(None));

pub async fn call_payments_from_queue(queue_req: QueueRequest) {
    let client = Client::new();

    let base_url = PAYMENT_PROCESSOR_DEFAULT.as_str();
    let full_url = format!("{}{}", base_url, queue_req.path);

    let method = queue_req
        .method
        .parse::<reqwest::Method>()
        .unwrap_or_else(|_| {
            eprintln!(
                "❌ Método HTTP inválido '{}', usando GET como fallback",
                queue_req.method
            );
            reqwest::Method::GET
        });

    let request = client
        .request(method.clone(), &full_url)
        .header("Content-Type", "application/json")
        .body(queue_req.body.clone());

    let result = timeout(Duration::from_secs(2), request.send()).await;

    match result {
        Ok(Ok(res)) => {
            println!("✅ Request enviada: {} {}", method, res.status());

            if !res.status().is_success() {
                eprintln!("⚠️ Response com status inesperado: {}", res.status());

                if let Err(e) = enqueue(queue_req.clone()).await {
                    eprintln!("❌ Falha ao reenfileirar request: {:?}", e);
                }
            }
        }
        Ok(Err(e)) => {
            eprintln!("❌ Erro na requisição: {:?}", e);
            if let Err(e) = enqueue(queue_req).await {
                eprintln!("❌ Falha ao reenfileirar request: {:?}", e);
            }
        }
        Err(_) => {
            eprintln!(
                "⏱ Timeout! A chamada demorou mais que {}s",
                Duration::from_secs(3).as_secs()
            );
            if let Err(e) = enqueue(queue_req).await {
                eprintln!("❌ Falha ao reenfileirar request: {:?}", e);
            }
        }
    }
}
fn start_queue_worker(mut rx: Receiver<QueueRequest>) {
    tokio::spawn(async move {
        println!("Queue iniciada com sucesso!");

        while let Some(queue_req) = rx.recv().await {
            println!("👷 Executando queue --> {:?}", queue_req);
            call_payments_from_queue(queue_req).await;
        }
    });
}

pub fn init_queue() {
    let (tx, rx) = mpsc::channel::<QueueRequest>(2000);
    *QUEUE_SENDER.lock().unwrap() = Some(tx);
    start_queue_worker(rx);
}

pub async fn enqueue(req: QueueRequest) -> Result<(), mpsc::error::SendError<QueueRequest>> {
    let sender_opt = {
        let guard = QUEUE_SENDER.lock().unwrap();
        guard.clone()
    };

    if let Some(sender) = sender_opt {
        sender.send(req).await
    } else {
        Err(mpsc::error::SendError(req))
    }
}
