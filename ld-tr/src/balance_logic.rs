use std::env;
use once_cell::sync::Lazy;

pub static PAYMENT_PROCESSOR_DEFAULT: Lazy<String> =
    Lazy::new(|| env::var("PAYMENT_PROCESSOR_DEFAULT").expect("PAYMENT_PROCESSOR_DEFAULT url not set"));

pub static PAYMENT_PROCESSOR_FALLBACK: Lazy<String> =
    Lazy::new(|| env::var("PAYMENT_PROCESSOR_FALLBACK").expect("PAYMENT_PROCESSOR_FALLBACK url not set"));

