use actix_web::{web, HttpResponse, Responder, get};
use sqlx::Executor;
use sqlx::Row;
use crate::models::pedido_prov::{PedidoProv, QueryParams};
use crate::database::connection::establish_connection;
use crate::utils::json_utils::convert_to_json;


// pub async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hola, mundo")
// }


#[get("/reporte_pedido_proveedor")]
async fn get_pedido_proveedor(query_params: web::Query<QueryParams>) -> impl Responder {

    println!("n_pedido: {}", query_params.n_pedido);
    println!("procedencia: {}", query_params.procedencia);

    let mut connection = establish_connection().await.unwrap();

    let query = format!(
        "SELECT T0.PEDIDO_PROV,
                CONVERT(NVARCHAR(30), T0.FEC_INGRESO, 120) AS FEC_INGRESO,
                T0.USUARIO,
                T0.ESTATUS,
                T3.DESCRIPCION AS CLIENTE,
                T1.DESCRIPCION AS PROVEEDOR,
                T2.DESCRIPCION,
                T0.DATO1,
                T0.DATO2,
                T0.DATO3,
                T0.DATO4,
                T0.DATO5,
                T0.FACTURA,
                T0.FACTURA_FAB,
                T0.BULTOS,
                T0.VAL1,
                T0.VAL2,
                T0.PESO
        FROM dbo.TD_CR_PEDIDO_PROV T0
                INNER JOIN dbo.TC_SOCIO_NEGOCIO T1 on T1.SOCIO = T0.SOCIO
                INNER JOIN dbo.TC_CR_PEDIDO_PROV_TIPO T2 ON T2.PEDIDO_PROV_TIPO = T0.PEDIDO_PROV_TIPO
                INNER JOIN dbo.TC_CR_PEDIDO_PROV_TIPO T3 ON T3.PEDIDO_PROV_TIPO = T0.PEDIDO_PROV_TIPO
        WHERE T0.PEDIDO_PROV = {} AND T0.PROCEDENCIA = '{}'",
        query_params.n_pedido,
        query_params.procedencia
    );

    let pedidos: Vec<PedidoProv> = sqlx::query_as::<_, PedidoProv>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let json_results = convert_to_json(pedidos);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string_pretty(&json_results).unwrap())
}


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/wms")
        .service(get_pedido_proveedor);
        // .service(register_user_handler)
        // .service(login_user_handler)
        // .service(logout_handler)
        // .service(get_me_handler);

    conf.service(scope);
}


