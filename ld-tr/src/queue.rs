use crate::payment::PaymentResponse;
use once_cell::sync::Lazy;
use redis::AsyncTypedCommands;
use reqwest::Client;
use std::env;
use tokio::time::sleep;
use crate::check_health::HEALTH_CHECK_DEFAULT;

static QUEUE_NAME: &str = "payment_queue";
static REDIS_CLIENT: Lazy<redis::Client> = Lazy::new(|| {
    let redis_url = env::var("REDIS").unwrap_or_else(|_| "redis://localhost:6379/".to_string());
    redis::Client::open(redis_url).expect("Erro ao criar cliente Redis")
});

pub async fn enqueue_payment(response: PaymentResponse) -> redis::RedisResult<()> {
    let mut conn = REDIS_CLIENT.get_multiplexed_tokio_connection().await?;
    let json = serde_json::to_string(&response).unwrap();
    conn.lpush(QUEUE_NAME, json).await?;
    Ok(())
}

use tokio::time::{timeout, Duration};

async fn dequeue_and_post(endpoint: String) -> redis::RedisResult<()> {
    if HEALTH_CHECK_DEFAULT.is_failed() {
        println!("⚠️ Payment Process off");
        return Ok(());
    }
    let mut conn = REDIS_CLIENT.get_multiplexed_tokio_connection().await?;
    let maybe_json: Option<String> = conn.rpop(QUEUE_NAME, None).await?;

    if let Some(json) = maybe_json {
        let parsed: PaymentResponse = serde_json::from_str(&json).unwrap();


        let http_client = Client::new();

        let result = timeout(Duration::from_secs(1), async {
            http_client.post(endpoint).json(&parsed).send().await
        }).await;

        match result {
            Ok(Ok(response)) => {
                println!("✅ POST feito com status: {}", response.status());
                if !response.status().is_success() {
                    eprintln!("❌ Erro no processamento da fila");
                    conn.rpush(QUEUE_NAME, json).await?;
                }
            }
            Ok(Err(err)) => {
                eprintln!("❌ Erro na requisição: {}", err);
                conn.rpush(QUEUE_NAME, json).await?;
            }
            Err(_) => {
                eprintln!("⏰ Timeout! POST demorou mais de 1 segundo");
                conn.rpush(QUEUE_NAME, json).await?;
            }
        }
    } else {
        println!("⚠️ Fila vazia, nada para processar.");
    }

    Ok(())
}

pub fn start_payment_worker(endpoint: String) {
    tokio::spawn(async move {
        loop {
            if let Err(e) = dequeue_and_post(endpoint.clone()).await {
                eprintln!("Erro no worker: {}", e);
            }
            sleep(Duration::from_secs(1)).await;
        }
                
    });
}
