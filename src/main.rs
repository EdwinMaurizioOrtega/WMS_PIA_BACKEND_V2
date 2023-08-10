use actix_web::{web, App, HttpServer};
use crate::routes::pedido_routes::{get_pedidos, hello};

mod models;
mod database;
mod utils;
mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let host = "0.0.0.0";
    let port = 8080;

    println!("🚀 Server started successfully on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(hello))) // Agrega la ruta /
            .service(web::resource("/pedidos").route(web::get().to(get_pedidos)))
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
