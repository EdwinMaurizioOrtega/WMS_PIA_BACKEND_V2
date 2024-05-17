use actix_web::{web, HttpResponse, Responder, get};
use actix_web::error::ErrorInternalServerError;
use actix_web::error::ParseError::Status;
use sqlx::Executor;
use sqlx::Row;
use crate::models::pedido_prov::{FullReporteDespachosConsolidados, FullReporteDespachosSinSeries, PedidoProv, PedidoV2, PedidoV3, PedidoV4, PedidoV5, PedidoV6, PedidoV7, QueryDateParams, QueryParams, QueryParamsPedidoAndDN};
use crate::database::connection::establish_connection;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::extension::ClientId;

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
       CONVERT(NVARCHAR(30), T0.FEC_ALTA, 120)    AS FEC_ALTA,
       T0.USUARIO,
       T0.ESTATUS,
       T2.DESCRIPCION                             AS CLIENTE,
       T1.DESCRIPCION                             AS PROVEEDOR,
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
       T5.DESCRIPCION                             AS DESCRIPCION_V2,
       T4.CANTIDAD,
       T4.DATA_DET1,
       T4.COSTO,
       T4.ARTICULO
FROM dbo.TD_CR_PEDIDO_PROV T0
         INNER JOIN dbo.TC_SOCIO_NEGOCIO T1 on T1.SOCIO = T0.SOCIO
         INNER JOIN dbo.TC_CR_PEDIDO_PROV_TIPO T2 ON T2.PEDIDO_PROV_TIPO = T0.PEDIDO_PROV_TIPO
         INNER JOIN dbo.TD_CR_PEDIDO_PROV_DET T4 ON T0.PEDIDO_PROV = T4.PEDIDO_PROV and T0.PROCEDENCIA = T4.PROCEDENCIA
         INNER JOIN dbo.TC_CR_ARTICULO T5 ON T5.ARTICULO = T4.ARTICULO AND T5.ART_PROCEDE = T4.PROCEDENCIA
WHERE T0.PROCEDENCIA = {} AND T0.FEC_ALTA BETWEEN CAST('{} 00:00:00' AS datetime) AND CAST('{} 23:59:59' AS datetime);",
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


#[get("/send_email")]
async fn send_email_microsoft(query_params: web::Query<QueryParamsPedidoAndDN>) -> Result<HttpResponse, actix_web::Error> {
    println!("pedidoProveedor: {}", query_params.n_pedido);
    println!("procedencia: {}", query_params.procedencia);
    println!("dn: {}", query_params.dn);


//1.Buscar la fecha de ingreso

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


    let creds = Credentials::new("sistemas@movilcelistic.com".to_owned(), "gt5P4&M#C74c".to_owned());

    let correos = vec![
        "allerena@movilcelistic.com",
        "mbarahona@movilcelistic.com",
        "sales@ht-bit.com",
        "aibarram@movilcelistic.com",
    ];

    // Open a remote connection to gmail
    let mailer = SmtpTransport::starttls_relay("smtp.office365.com")
        .unwrap()
        .port(587)
        .credentials(creds)
        .build();

    for correo in correos {

        let asusnto: String;  // Declarar la variable aquí para que sea válida en todo el bloque

        if let Some(primer_pedido) = pedidos.first() {
            let fec_ingreso_primer_pedido = &primer_pedido.FEC_INGRESO;
            // Ahora fec_ingreso_primer_pedido contiene el valor de FEC_INGRESO del primer pedido
            asusnto = format!("RECEPCIÓN DE MERCADERÍA BODEGA HT {}", fec_ingreso_primer_pedido);
        } else {
            println!("No se encontraron pedidos.");
            asusnto = String::from("RECEPCIÓN DE MERCADERÍA BODEGA HT");  // Asunto predeterminado en caso de que no haya pedidos
        }

        let mensaje = format!("Estimados,\n Se confirma el ingreso a bodega de la mercadería correspondiente a:\n\n PEDIDO PROVEEDOR: {} \nDN: {}", query_params.n_pedido, query_params.dn);

        let email = Message::builder()
            .from("sistemas@movilcelistic.com".parse().unwrap())
            .to(correo.parse().unwrap())
            .subject(asusnto)
            .header(ContentType::TEXT_PLAIN)
            .body(mensaje)
            .unwrap();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent to: {}", correo),
            Err(e) => eprintln!("Could not send email to {}: {:?}", correo, e),
        }
    }

    Ok(HttpResponse::Ok().body("Emails sent successfully!"))
}

// =======Reporteria WMS - PIA=====

#[get("/full_reporte_despachos_consolidados")]
async fn get_full_reporte_despachos_consolidados(query_params: web::Query<QueryDateParams>) -> impl Responder {

    println!("procedencia: {}", query_params.proced);

    let mut connection = establish_connection().await.unwrap();

    let query = format!("SELECT * FROM FullDespachosConsolidados( {}, '{}', '{}');",
        query_params.proced,
        query_params.fec_inicio,
        query_params.fec_fin
    );

    let pedidos: Vec<FullReporteDespachosConsolidados> = sqlx::query_as::<_, FullReporteDespachosConsolidados>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}

#[get("/full_reporte_despachos_sin_series")]
async fn get_full_reporte_despachos_sin_series(query_params: web::Query<QueryDateParams>) -> impl Responder {

    println!("procedencia: {}", query_params.proced);

    let mut connection = establish_connection().await.unwrap();

    let query = format!("SELECT * FROM FullDespachosSinSeries( {}, '{}', '{}');",
                        query_params.proced,
                        query_params.fec_inicio,
                        query_params.fec_fin
    );

    let pedidos: Vec<FullReporteDespachosSinSeries> = sqlx::query_as::<_, FullReporteDespachosSinSeries>(&query)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    let user_response = serde_json::json!({"data": pedidos});

    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}

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
        .service(get_despacho_detalle_pedido_proveedor)
        .service(send_email_microsoft)
        .service(get_full_reporte_despachos_consolidados)
        .service(get_full_reporte_despachos_sin_series)
        ;

    conf.service(scope);
}