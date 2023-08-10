use actix_web::{web, App, HttpServer};

mod models;
mod database;
mod utils;
mod routes;

use actix_cors::Cors;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let host = "0.0.0.0";
    let port = 80;

    println!("🚀 Server started successfully on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::permissive()
            )
            .configure(routes::crm_routes::config)
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
