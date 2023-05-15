use actix_web::{HttpResponse, Responder};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
