mod models;

use models::Status;
use actix_web::{web, App, HttpServer, Responder};

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "Ok".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}