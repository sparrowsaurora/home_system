use actix_web::{get, web, HttpResponse, Responder};
use tokio::fs;

#[get("/")]
async fn root() -> impl Responder {
    println!("Root");
    match fs::read_to_string("static/index.html").await {
        Ok(contents) => HttpResponse::Ok().content_type("text/html").body(contents),
        Err(_) => HttpResponse::InternalServerError().body("Could not load HTML file"),
    }
}

#[get("/weather")]
async fn weather() -> impl Responder {
    println!("/weather");
    HttpResponse::Ok().body("Hello from /weather")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(root).service(weather);
}
