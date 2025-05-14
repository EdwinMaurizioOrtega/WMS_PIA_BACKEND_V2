use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use crate::database::connection::establish_connection;
use crate::models::mc_cliente_cnt::MCParroquia;
use crate::models::mc_parkenor::{ParkenorImagenes, ParkenorProducts};
use crate::models::user_model::TokenClaims;
use crate::repository::mssql_repo::get_user_by_id;
use crate::routes::user_api::{my_access, my_account, signin};

#[get("/get_imagenes")]
async fn my_imagenes(req: HttpRequest) -> impl Responder {

    let mut connection = establish_connection().await.unwrap();

    let query = "SELECT T3.URL                                                                    AS IMAGEN,
       T0.ARTICULO,
       T1.DESCRIPCION
FROM WMS_EC.dbo.TD_CR_ARTICULO_SIN_SERIE T0
         INNER JOIN WMS_EC.dbo.TC_CR_ARTICULO T1 ON T1.ARTICULO = T0.ARTICULO AND T1.ART_PROCEDE = T0.ART_PROCEDE
         INNER JOIN WMS_EC.dbo.MC_WEB_IMAGEN T3 ON T3.COD_ARTICULO = T0.ARTICULO
WHERE T0.ART_PROCEDE = 7183
  AND T0.CVECIUDAD LIKE 'PARK_UIO_TME';".to_string();

    let cli: Result<Vec<ParkenorImagenes>, sqlx::Error> = sqlx::query_as(&query)
        .fetch_all(&mut connection)
        .await;

    match cli {
        Ok(clientes) => HttpResponse::Ok().json(json!({"data": clientes})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}





#[get("/products")]
async fn get_all_a_movil_celistic_products() -> impl Responder {

    let mut connection = establish_connection().await.unwrap();

    let query = "SELECT T3.URL                                                                    AS IMAGEN,
       T0.ARTICULO,
       T1.DESCRIPCION,
--        T2.CVEALMACEN,
--        T2.DESCRIPCION,
--        T0.CANTIDAD,
       CASE WHEN T2.DESCRIPCION = 'PRE-PAGO MERCH' THEN T0.CANTIDAD ELSE '0' END AS PRE_PAGO_MERCH,
       CASE WHEN T2.DESCRIPCION = 'BTL MERCH' THEN T0.CANTIDAD ELSE '0' END      AS BTL_MERCH,
       CASE WHEN T2.DESCRIPCION = 'PUBLICIDAD' THEN T0.CANTIDAD ELSE '0' END     AS PUBLICIDAD
FROM WMS_EC.dbo.TD_CR_ARTICULO_SIN_SERIE T0
         INNER JOIN WMS_EC.dbo.TC_CR_ARTICULO T1 ON T1.ARTICULO = T0.ARTICULO AND T1.ART_PROCEDE = T0.ART_PROCEDE
         INNER JOIN WMS_EC.dbo.TC_CR_ALMACEN T2 ON T2.CVEALMACEN = T0.CVEALMACEN AND T2.CVECIUDAD = T0.CVECIUDAD
         LEFT JOIN WMS_EC.dbo.MC_WEB_IMAGEN T3 ON T3.COD_ARTICULO = T0.ARTICULO
WHERE T0.ART_PROCEDE = 7183
  AND T0.CVECIUDAD LIKE 'PARK_UIO_TME';".to_string();

    let cli: Result<Vec<ParkenorProducts>, sqlx::Error> = sqlx::query_as(&query)
        .fetch_all(&mut connection)
        .await;

    match cli {
        Ok(clientes) => HttpResponse::Ok().json(json!({"data": clientes})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/parkenor")
        .service(my_imagenes)
        .service(get_all_a_movil_celistic_products)

        ;

    conf.service(scope);
}
