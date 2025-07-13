use crate::check_health::{HEALTH_CHECK_DEFAULT, HEALTH_CHECK_FALLBACK};
use crate::payment::{PaymentRequest, PaymentResponse};
use crate::queue::{QUEUE, request_to_queue};
use actix_web::{HttpRequest, web};
use once_cell::sync::Lazy;
use reqwest::Client;
use std::env;

pub static PAYMENT_PROCESSOR_DEFAULT: Lazy<String> = Lazy::new(|| {
    env::var("PAYMENT_PROCESSOR_DEFAULT").expect("PAYMENT_PROCESSOR_DEFAULT url not set")
});

pub static PAYMENT_PROCESSOR_FALLBACK: Lazy<String> = Lazy::new(|| {
    env::var("PAYMENT_PROCESSOR_FALLBACK").expect("PAYMENT_PROCESSOR_FALLBACK url not set")
});

pub async fn call_payments(req: HttpRequest, mut body: web::Bytes) {
    let client = Client::new();

    let parsed: Result<PaymentRequest, _> = serde_json::from_slice(&body);

    if let Ok(parsed) = parsed {
        let response = PaymentResponse::new(parsed.correlation_id, parsed.amount);
        body = web::Bytes::from(serde_json::to_string(&response).unwrap());
        println!("{:?}", body);
    }

    let base_url = if HEALTH_CHECK_DEFAULT.is_failed() {
        if HEALTH_CHECK_FALLBACK.is_failed() {
            QUEUE
                .lock()
                .unwrap()
                .push_back(request_to_queue(&req, body.clone()));
            return;
        }
        PAYMENT_PROCESSOR_FALLBACK.as_str()
    } else {
        PAYMENT_PROCESSOR_DEFAULT.as_str()
    };

    let full_url = format!("{}{}", base_url, req.uri());

    let method = req.method().to_string().parse::<reqwest::Method>().unwrap();
    let mut request_builder = client.request(method, full_url);

    for (key, value) in req.headers().iter() {
        if let Ok(val_str) = value.to_str() {
            request_builder = request_builder.header(key.as_str(), val_str);
        }
    }

    match request_builder.body(body.clone()).send().await {
        Ok(res) => {
            println!("{:?}", res)
        }
        Err(_) => {
            println!("error ;-;");
        }
    }
}
