use actix_web::{web, HttpResponse, Responder, get};
use sqlx::Executor;
use sqlx::Row;
use crate::models::pedido_prov::{PedidoProv, PedidoV2, PedidoV3, PedidoV4, PedidoV5, PedidoV6, PedidoV7, QueryDateParams, QueryParams};
use crate::database::connection::establish_connection;

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

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}

#[get("/dn_reporte_pedido_proveedor")]
async fn get_dn_pedido_proveedor(query_params: web::Query<QueryParams>) -> impl Responder {

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
                     AND T0.DATO3 = '{}' AND T0.PROCEDENCIA = '{}'",
        query_params.n_pedido,
        query_params.procedencia
    );

    let pedidos: Vec<PedidoProv> = sqlx::query_as::<_, PedidoProv>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)

}


#[get("/reporte_pedido_proveedor_filtro_fecha")]
async fn get_pedido_proveedor_filtro_fechas(query_params: web::Query<QueryDateParams>) -> impl Responder {
    println!("proced: {}", query_params.proced);
    println!("fec_inicio: {}", query_params.fec_inicio);
    println!("fec_fin: {}", query_params.fec_fin);

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
                        T0.PESO,
                        STUFF((SELECT ', ' + T5.DESCRIPCION + ' (' + CAST(T4.CANTIDAD AS VARCHAR) + ')'
                               FROM dbo.TD_CR_PEDIDO_PROV_DET T4
                                        INNER JOIN dbo.TC_CR_ARTICULO T5
                                                   ON T5.ARTICULO = T4.ARTICULO AND T5.ART_PROCEDE = T4.PROCEDENCIA
                               WHERE T4.PEDIDO_PROV = T0.PEDIDO_PROV
                                 AND T4.PEDIDO_PROV = 31
                                 AND T4.PROCEDENCIA = 9000
                                  FOR XML PATH (''), TYPE ).value('.', 'NVARCHAR(MAX)'), 1, 2, '') AS Articulos
                 FROM dbo.TD_CR_PEDIDO_PROV T0
                          INNER JOIN dbo.TC_SOCIO_NEGOCIO T1 on T1.SOCIO = T0.SOCIO
                          INNER JOIN dbo.TC_CR_PEDIDO_PROV_TIPO T2 ON T2.PEDIDO_PROV_TIPO = T0.PEDIDO_PROV_TIPO
                          INNER JOIN dbo.TC_CR_PEDIDO_PROV_TIPO T3 ON T3.PEDIDO_PROV_TIPO = T0.PEDIDO_PROV_TIPO
                 WHERE T0.PROCEDENCIA = {}
                   AND T0.FEC_INGRESO BETWEEN CAST('{} 00:00:00' AS datetime) AND CAST('{} 23:59:59' AS datetime)",
        query_params.proced,
        query_params.fec_inicio,
        query_params.fec_fin
    );

    let pedidos: Vec<PedidoV5> = sqlx::query_as::<_, PedidoV5>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}

#[get("/reporte_detalle_pedido_proveedor")]
async fn get_detalle_pedido_proveedor(query_params: web::Query<QueryParams>) -> impl Responder {

    println!("n_pedido: {}", query_params.n_pedido);
    println!("procedencia: {}", query_params.procedencia);

    let mut connection = establish_connection().await.unwrap();

    let query = format!(
        "SELECT T0.PEDIDO_PROV,
       T0.PROCEDENCIA,
       T0.ARTICULO,
       T0.SERIE,
       T1.DESCRIPCION,
       T1.PESO
FROM TR_CR_PEDIDO_PROV_SERIE T0
         INNER JOIN TC_CR_ARTICULO T1 ON T1.ARTICULO = T0.ARTICULO AND T1.ART_PROCEDE = T0.PROCEDENCIA
    AND T0.PEDIDO_PROV = {} and T0.PROCEDENCIA = {}",
        query_params.n_pedido,
        query_params.procedencia
    );

    let pedidos: Vec<PedidoV2> = sqlx::query_as::<_, PedidoV2>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)

}

#[get("/dn_reporte_detalle_pedido_proveedor")]
async fn get_dn_detalle_pedido_proveedor(query_params: web::Query<QueryParams>) -> impl Responder {
    println!("n_pedido: {}", query_params.n_pedido);
    println!("procedencia: {}", query_params.procedencia);

    let mut connection = establish_connection().await.unwrap();

    let query = format!(
        "WITH Subquery AS (SELECT TOP 1
        T0.PEDIDO_PROV, T0.PROCEDENCIA
                                   FROM dbo.TD_CR_PEDIDO_PROV T0
                                   WHERE T0.DATO3 = '{}'
                                     AND T0.PROCEDENCIA = {})
                 SELECT T0.PEDIDO_PROV,
                        T0.PROCEDENCIA,
                        T0.ARTICULO,
                        T0.SERIE,
                        T1.DESCRIPCION,
                        T1.PESO
                 FROM TR_CR_PEDIDO_PROV_SERIE T0
                          INNER JOIN
                      TC_CR_ARTICULO T1 ON T1.ARTICULO = T0.ARTICULO AND T1.ART_PROCEDE = T0.PROCEDENCIA
                          INNER JOIN
                      Subquery ON T0.PEDIDO_PROV = Subquery.PEDIDO_PROV AND T0.PROCEDENCIA = Subquery.PROCEDENCIA
                 WHERE T0.PROCEDENCIA = Subquery.PROCEDENCIA;",
        query_params.n_pedido,
        query_params.procedencia
    );

    let pedidos: Vec<PedidoV2> = sqlx::query_as::<_, PedidoV2>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)

}

#[get("/reporte_cantidad_detalle_pedido_proveedor")]
async fn get_cantidad_detalle_pedido_proveedor(query_params: web::Query<QueryParams>) -> impl Responder {
    println!("n_pedido: {}", query_params.n_pedido);
    println!("procedencia: {}", query_params.procedencia);

    let mut connection = establish_connection().await.unwrap();

    let query = format!(
        "SELECT T0.PEDIDO_PROV,
       T0.PROCEDENCIA,
       T0.ARTICULO,
       T0.ART_PROCEDE,
       T0.CANTIDAD,
       T0.DATA_DET1,
       T1.DESCRIPCION,
       T1.PESO
FROM WMS_EC.dbo.TD_CR_PEDIDO_PROV_DET T0
         INNER JOIN TC_CR_ARTICULO T1 ON T1.ARTICULO = T0.ARTICULO AND T1.ART_PROCEDE = T0.ART_PROCEDE
WHERE PEDIDO_PROV = {}
  AND PROCEDENCIA = {}
ORDER BY PEDIDO_PROV;",
        query_params.n_pedido,
        query_params.procedencia
    );

    let pedidos: Vec<PedidoV3> = sqlx::query_as::<_, PedidoV3>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)

}

#[get("/dn_reporte_cantidad_detalle_pedido_proveedor")]
async fn get_dn_cantidad_detalle_pedido_proveedor(query_params: web::Query<QueryParams>) -> impl Responder {
    println!("n_pedido: {}", query_params.n_pedido);
    println!("procedencia: {}", query_params.procedencia);

    let mut connection = establish_connection().await.unwrap();

    let query = format!(
        "WITH Subquery AS (SELECT TOP 1
        T0.PEDIDO_PROV, T0.PROCEDENCIA
                                   FROM dbo.TD_CR_PEDIDO_PROV T0
                                   WHERE T0.DATO3 = '{}'
                                     AND T0.PROCEDENCIA = {})
                 SELECT T0.PEDIDO_PROV,
                        T0.PROCEDENCIA,
                        T0.ARTICULO,
                        T0.ART_PROCEDE,
                        T0.CANTIDAD,
                        T0.DATA_DET1,
                        T1.DESCRIPCION,
                        T1.PESO
                 FROM WMS_EC.dbo.TD_CR_PEDIDO_PROV_DET T0
                          INNER JOIN
                      TC_CR_ARTICULO T1 ON T1.ARTICULO = T0.ARTICULO AND T1.ART_PROCEDE = T0.ART_PROCEDE
                          INNER JOIN
                      Subquery ON T0.PEDIDO_PROV = Subquery.PEDIDO_PROV AND T0.PROCEDENCIA = Subquery.PROCEDENCIA
                 WHERE T0.PROCEDENCIA = Subquery.PROCEDENCIA
                 ORDER BY T0.PEDIDO_PROV;",
        query_params.n_pedido,
        query_params.procedencia
    );

    let pedidos: Vec<PedidoV3> = sqlx::query_as::<_, PedidoV3>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}

#[get("/rango_fecha_creacion_pedido_proveedor")]
async fn get_rango_fecha_creacion_pedido_proveedor(query_params: web::Query<QueryDateParams>) -> impl Responder {
    println!("proced: {}", query_params.proced);
    println!("fec_inicio: {}", query_params.fec_inicio);
    println!("fec_fin: {}", query_params.fec_fin);

    let mut connection = establish_connection().await.unwrap();

    let query = format!(
        "SELECT T0.PEDIDO_PROV,
               CONVERT(NVARCHAR(30), T0.FEC_INGRESO, 120) AS FEC_INGRESO,
               CONVERT(NVARCHAR(30), T0.FEC_ALTA, 120) AS FEC_ALTA,
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
               T0.VAL1,
               T0.VAL2,
               T0.PESO
        FROM dbo.TD_CR_PEDIDO_PROV T0
                 INNER JOIN dbo.TC_SOCIO_NEGOCIO T1 on T1.SOCIO = T0.SOCIO
                 INNER JOIN dbo.TC_CR_PEDIDO_PROV_TIPO T2 ON T2.PEDIDO_PROV_TIPO = T0.PEDIDO_PROV_TIPO
                 INNER JOIN dbo.TC_CR_PEDIDO_PROV_TIPO T3 ON T3.PEDIDO_PROV_TIPO = T0.PEDIDO_PROV_TIPO
        WHERE T0.PROCEDENCIA = {} AND T0.FEC_ALTA BETWEEN CAST('{} 00:00:00' AS datetime )  AND  CAST('{} 23:59:59' AS datetime)",
        query_params.proced,
        query_params.fec_inicio,
        query_params.fec_fin
    );

    let pedidos: Vec<PedidoV4> = sqlx::query_as::<_, PedidoV4>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}


#[get("/rango_fecha_llegada_pedido_proveedor_bodega")]
async fn get_rango_fecha_llegada_pedido_proveedor_bodega(query_params: web::Query<QueryDateParams>) -> impl Responder {
    println!("proced: {}", query_params.proced);
    println!("fec_inicio: {}", query_params.fec_inicio);
    println!("fec_fin: {}", query_params.fec_fin);

    let mut connection = establish_connection().await.unwrap();

    let query = format!(
        "SELECT T0.PEDIDO_PROV,
               CONVERT(NVARCHAR(30), T0.FEC_INGRESO, 120) AS FEC_INGRESO,
               CONVERT(NVARCHAR(30), T0.FEC_ALTA, 120) AS FEC_ALTA,
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
               T0.VAL1,
               T0.VAL2,
               T0.PESO
        FROM dbo.TD_CR_PEDIDO_PROV T0
                 INNER JOIN dbo.TC_SOCIO_NEGOCIO T1 on T1.SOCIO = T0.SOCIO
                 INNER JOIN dbo.TC_CR_PEDIDO_PROV_TIPO T2 ON T2.PEDIDO_PROV_TIPO = T0.PEDIDO_PROV_TIPO
                 INNER JOIN dbo.TC_CR_PEDIDO_PROV_TIPO T3 ON T3.PEDIDO_PROV_TIPO = T0.PEDIDO_PROV_TIPO
        WHERE T0.PROCEDENCIA = {} AND T0.FEC_INGRESO BETWEEN CAST('{} 00:00:00' AS datetime )  AND  CAST('{} 23:59:59' AS datetime)",
        query_params.proced,
        query_params.fec_inicio,
        query_params.fec_fin
    );

    let pedidos: Vec<PedidoV4> = sqlx::query_as::<_, PedidoV4>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}



#[get("/reporte_despacho_pedido_proveedor")]
async fn get_despacho_pedido_proveedor(query_params: web::Query<QueryParams>) -> impl Responder {
    println!("n_pedido: {}", query_params.n_pedido);
    println!("procedencia: {}", query_params.procedencia);

    let mut connection = establish_connection().await.unwrap();

    let query = format!(
        "SELECT T0.NUM_PEDIDO,
       T0.PROCEDENCIA,
       CONVERT(NVARCHAR(30), T0.FECHA, 120) AS FECHA,
       T0.CONTACTO,
       T0.TEL_CONTACTO,
       T0.CANTIDAD,
       T0.TOTAL,
       T0.CANTON,
       T0.PROVINCIA,
       T1.DESCRIPCION,
       T2.CONTRATO,
       T3.BULTOS
FROM dbo.TD_CR_PEDIDO T0
         INNER JOIN dbo.TC_CR_CLIENTE T1 ON T1.CTE = T0.CTE and T1.CTE_PROCEDE = T0.CTE_PROCEDE
         INNER JOIN dbo.TD_CR_PEDIDO_CONTRATO T2 ON T0.NUM_PEDIDO = T2.NUM_PEDIDO and T0.PROCEDENCIA = T2.PROCEDENCIA
         INNER JOIN dbo.TD_CR_PEDIDO_TRANSPORTE T3 ON T0.NUM_PEDIDO = T3.NUM_PEDIDO and T0.PROCEDENCIA = T3.PROCEDENCIA
WHERE T0.NUM_PEDIDO = {}
  AND T0.PROCEDENCIA = {}",
        query_params.n_pedido,
        query_params.procedencia
    );

    let pedidos: Vec<PedidoV6> = sqlx::query_as::<_, PedidoV6>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}


#[get("/reporte_despacho_detalle_pedido_proveedor")]
async fn get_despacho_detalle_pedido_proveedor(query_params: web::Query<QueryParams>) -> impl Responder {
    println!("n_pedido: {}", query_params.n_pedido);
    println!("procedencia: {}", query_params.procedencia);

    let mut connection = establish_connection().await.unwrap();

    let query = format!(
        "SELECT T0.NUM_PEDIDO,
       T0.PROCEDENCIA,
       T0.ARTICULO,
       T0.CANTIDAD,
       T0.TOTAL,
       T1.DESCRIPCION,
       T1.ART_TIPO,
       T2.DESCRIPCION AS DESCRIPCION_2
FROM dbo.TD_CR_PEDIDO_DET T0
         INNER JOIN dbo.TC_CR_ARTICULO T1 ON T1.ARTICULO = T0.ARTICULO AND T1.ART_PROCEDE = T0.ART_PROCEDE
         INNER JOIN TC_CR_ARTICULO_TIPO T2 ON T1.ART_TIPO = T2.ART_TIPO
WHERE NUM_PEDIDO = {}
  AND PROCEDENCIA = {}",
        query_params.n_pedido,
        query_params.procedencia
    );

    let pedidos: Vec<PedidoV7> = sqlx::query_as::<_, PedidoV7>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}


// =======Reporteria WMS - PIA=====
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/wms")
        //Por número de pedido
        .service(get_pedido_proveedor)
        //Por DN del pedido
        .service(get_dn_pedido_proveedor)
        .service(get_pedido_proveedor_filtro_fechas)
        .service(get_detalle_pedido_proveedor)
        .service(get_dn_detalle_pedido_proveedor)
        .service(get_cantidad_detalle_pedido_proveedor)
        .service(get_dn_cantidad_detalle_pedido_proveedor)
        .service(get_rango_fecha_creacion_pedido_proveedor)
        .service(get_rango_fecha_llegada_pedido_proveedor_bodega)
        .service(get_despacho_pedido_proveedor)
        .service(get_despacho_detalle_pedido_proveedor);

    conf.service(scope);
}