use actix_web::{App, HttpServer};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://localhost:8080");

    HttpServer::new(|| {
        App::new().configure(routes::configure) // Import routes from module
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
