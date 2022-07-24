use actix_web::{get, HttpResponse, Responder};


#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}
