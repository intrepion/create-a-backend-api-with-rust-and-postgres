mod employees;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(employees::init_routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
