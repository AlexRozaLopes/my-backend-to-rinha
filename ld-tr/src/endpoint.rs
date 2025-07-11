use actix_web::{HttpRequest, HttpResponse, Responder, post, web, get};

#[post("/payments")]
pub async fn proxy_payment(req: HttpRequest, body: web::Bytes) -> impl Responder {
    println!("{:?}", req);
    println!("{:?}", body);
    HttpResponse::Ok()
}

#[get("/payments-summary")]
pub async fn proxy_payments_summary(req: HttpRequest, body: web::Bytes) -> impl Responder {
    println!("{:?}", req);
    println!("{:?}", body);
    HttpResponse::Ok()
}
