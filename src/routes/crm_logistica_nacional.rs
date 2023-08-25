use actix_web::{Error, get, HttpResponse, post, Responder, web};
use chrono::NaiveDate;
use serde_json::json;
use sqlx::{Decode, Mssql};
use sqlx::error::BoxDynError;

use crate::database::connection::establish_connection;
use crate::models::mc_cliente_cnt::McClienteCnt;

//Listamos todas las imágenes y PDFs
#[get("/clientes_cnt")]
async fn get_all_clientes_cnt() -> impl Responder {

    let mut connection = establish_connection().await.unwrap();

    let query = format!("SELECT CVE,
    OPEN_SMARTFLEX,
    CL_SAP,
    ALMACEN_SAP,
    FECHA_CREACION,
    FECHA_CIERRE,
    ESTADO,
    REGIONAL_CANAL
    FROM MC_CLIENTE_CNT ORDER BY CVE ASC");

    let cli: Result<Vec<McClienteCnt>, sqlx::Error> = sqlx::query_as(&query)
        .fetch_all(&mut connection)
        .await;

    match cli {
        Ok(clientes) => HttpResponse::Ok().json(json!({"data": clientes})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/cliente_cnt")]
async fn create_cliente_cnt(cliente_data: web::Json<McClienteCnt>) -> impl Responder {

    let mut connection = establish_connection().await.unwrap();

    let query = "INSERT INTO MC_CLIENTE_CNT (OPEN_SMARTFLEX, CL_SAP, ALMACEN_SAP, FECHA_CREACION, FECHA_CIERRE, ESTADO, REGIONAL_CANAL)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)";

    let cli: Result<McClienteCnt, sqlx::Error> = sqlx::query_as(query)
        .bind(cliente_data.OPEN_SMARTFLEX)
        .bind(&cliente_data.CL_SAP)
        .bind(cliente_data.ALMACEN_SAP)
        .bind(&cliente_data.FECHA_CREACION)
        .bind(&cliente_data.FECHA_CIERRE)
        .bind(cliente_data.ESTADO)
        .bind(&cliente_data.REGIONAL_CANAL)
        .fetch_one(&mut connection)
        .await;

    match cli {
        Ok(clientes) => HttpResponse::Ok().json(json!({"data": clientes})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/logistica-nacional")
        .service(get_all_clientes_cnt)
        .service(create_cliente_cnt);

    conf.service(scope);
}