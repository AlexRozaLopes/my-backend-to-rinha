use crate::balance_logic::{PAYMENT_PROCESSOR_DEFAULT, PAYMENT_PROCESSOR_FALLBACK};
use crate::check_health::{HEALTH_CHECK_DEFAULT, HEALTH_CHECK_FALLBACK};
use actix_web::{HttpRequest, web};
use once_cell::sync::Lazy;
use reqwest::Client;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

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

pub fn request_to_queue(req: &HttpRequest, body: web::Bytes) -> QueueRequest {
    QueueRequest {
        method: req.method().to_string(),
        path: req.path().to_string(),
        body,
    }
}

pub async fn call_payments_from_queue(queue_req: QueueRequest) {
    let client = Client::new();

    let base_url = match (
        HEALTH_CHECK_DEFAULT.is_failed(),
        HEALTH_CHECK_FALLBACK.is_failed(),
    ) {
        (true, true) => {
            enqueue(queue_req);
            return;
        }
        (true, false) => PAYMENT_PROCESSOR_FALLBACK.as_str(),
        _ => PAYMENT_PROCESSOR_DEFAULT.as_str(),
    };

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

    let response = client
        .request(method.clone(), &full_url)
        .body(queue_req.body.clone())
        .send()
        .await;

    match response {
        Ok(res) => println!("✅ Request enviada: {} {}", method, res.status()),
        Err(e) => {
            eprintln!("❌ Falha ao enviar request: {:?}", e);
            enqueue(queue_req);
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
    let (tx, rx) = mpsc::channel::<QueueRequest>(100);
    *QUEUE_SENDER.lock().unwrap() = Some(tx);
    start_queue_worker(rx);
}

pub fn enqueue(req: QueueRequest) {
    if let Some(sender) = &*QUEUE_SENDER.lock().unwrap() {
        if let Err(e) = sender.try_send(req) {
            eprintln!("❌ Fila cheia! Não foi possível enfileirar: {:?}", e);
        }
    } else {
        eprintln!("❌ Sender ainda não foi inicializado.");
    }
}
