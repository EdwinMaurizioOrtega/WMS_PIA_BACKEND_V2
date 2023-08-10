use actix_web::{web, HttpResponse, Responder};
use sqlx::Executor;
use sqlx::Row;
use crate::models::pedido_prov::PedidoProv;
use crate::database::connection::establish_connection;
use crate::utils::json_utils::convert_to_json;


pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hola, mundo")
}
pub async fn get_pedidos() -> impl Responder {
    let mut connection = establish_connection().await.unwrap();

    let query = "SELECT * FROM dbo.TD_CR_PEDIDO_PROV T0";
    let pedidos: Vec<PedidoProv> = sqlx::query_as::<_, PedidoProv>(query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let json_results = convert_to_json(pedidos);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string_pretty(&json_results).unwrap())
}


