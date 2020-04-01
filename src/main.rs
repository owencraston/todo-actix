mod models;
mod config;

use crate::models::Status;
use crate::config::Config;
use actix_web::{HttpServer, App, web, Responder};
use std::io;
use dotenv::dotenv;

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "Ok".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = Config::from_env().unwrap();

    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
