use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn root() -> impl Responder {
    println!("Root");
    HttpResponse::Ok().body("Hello from /")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    println!("/Hello");
    HttpResponse::Ok().body("Hello from /hello")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(root).service(hello);
}
