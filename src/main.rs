mod models;
mod setting;

use dotenv::dotenv;
use models::Status;
use setting::Setting;
use actix_web::{web, App, HttpServer, Responder};

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "Ok".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let config = Setting::from_env().unwrap();

    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}