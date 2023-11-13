use actix_web::{web, App, HttpServer};

mod models;
mod database;
mod utils;
mod routes;
mod repository;

use actix_cors::Cors;
use actix_web::web::Data;
use crate::repository::mongodb_repo_files::MongoRepo;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let host = "0.0.0.0";
    let port = 80;

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    println!("🚀 Server started successfully on http://{}:{}", host, port);

    //std::env::set_var("RUST_LOG", "actix_web=debug");
    HttpServer::new( move || {
        App::new()
            .app_data(db_data.clone())

            .wrap(
                Cors::permissive()
            )
            .configure(routes::crm_routes::config)
            .configure(routes::crm_upload_files::config)
            .configure(routes::crm_logistica_nacional::config)
            .configure(routes::user_api::config)
            .configure(routes::crm_plantilla_upload::config)

    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
