use crate::logic::{self, weather::get_weather};
use actix_web::{dev::Response, get, web, HttpResponse, Responder};
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
    // Logic
    let response: logic::weather::WeatherReport = get_weather();

    // Returns
    println!("{}", response);
    HttpResponse::Ok().body(format!("{}", response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(root).service(weather);
}
