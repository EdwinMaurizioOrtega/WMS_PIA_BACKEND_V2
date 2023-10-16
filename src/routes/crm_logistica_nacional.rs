use actix_web::{delete, Error, get, HttpResponse, post, put, Responder, web};
use chrono::{DateTime, Utc};
use serde_json::json;


use crate::database::connection::establish_connection;
use crate::models::mc_cliente_cnt::{DeleteRequest, McClienteCnt, McClienteCntResult, MCParroquia};

//Listamos todas las imágenes y PDFs
#[get("/clientes_cnt")]
async fn get_all_clientes_cnt() -> impl Responder {
    let mut connection = establish_connection().await.unwrap();

    let query = format!("SELECT cve,
       open_smartflex,
       cl_sap,
       almacen_sap,
       fecha_creacion,
       fecha_cierre,
       estado,
       regional,
       canal,
       descripcion_almacen,
       direccion,
       provincia,
       ciudad,
       nombre_contacto,
       telefono_contacto,
       fecha_modificacion
FROM MC_CLIENTE_CNT
ORDER BY cve DESC");

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
    println!("open_smartflex: {}", cliente_data.open_smartflex);
    println!("cl_sap: {}", &cliente_data.cl_sap);
    println!("almacen_sap: {}", cliente_data.almacen_sap);

    let mut connection = establish_connection().await.unwrap();

    //Insertar y retornar lo insertado
    let query = "INSERT INTO MC_CLIENTE_CNT (
    open_smartflex,
    cl_sap,
    almacen_sap,
    fecha_creacion,

    estado,
    regional,
    canal,
    descripcion_almacen,
    direccion,
    provincia,
    nombre_contacto,
    telefono_contacto )
                 OUTPUT INSERTED.cve,
                 INSERTED.open_smartflex,
                 INSERTED.cl_sap,
                 INSERTED.almacen_sap,
                 INSERTED.fecha_creacion,

                 INSERTED.estado,
                 INSERTED.regional,
                 INSERTED.canal,
                 INSERTED.descripcion_almacen,
                 INSERTED.direccion,
                 INSERTED.provincia,
                 INSERTED.nombre_contacto,
                 INSERTED.telefono_contacto
                 VALUES (@p1, @p2, @p3, @p4, @p5, @p6, @p7, @p8, @p9, @p10, @p11, @p12)";

    // Obtener la fecha y hora actual
    let fecha_actual: DateTime<Utc> = Utc::now();
    // Formatear la fecha y hora
    let fecha_hora_formateada = fecha_actual.format("%Y-%m-%d %H:%M:%S").to_string();

    let cli: Result<McClienteCntResult, sqlx::Error> = sqlx::query_as(query)
        .bind(&cliente_data.open_smartflex)
        .bind(&cliente_data.cl_sap)
        .bind(&cliente_data.almacen_sap)
        //Fecha y hora actual del sistema para la creacion del registro
        .bind(&fecha_hora_formateada)
        //Cuando el punto de venta cierra el local físico.
        // .bind(&cliente_data.fecha_cierre.as_deref())
        .bind(&cliente_data.estado)
        .bind(&cliente_data.regional)
        .bind(&cliente_data.canal)
        .bind(&cliente_data.descripcion_almacen)
        .bind(&cliente_data.direccion)
        .bind(&cliente_data.provincia)
        .bind(&cliente_data.nombre_contacto)
        .bind(&cliente_data.telefono_contacto)
        .fetch_one(&mut connection)
        .await;

    match cli {
        Ok(clientes) => HttpResponse::Ok().json(json!({"data": clientes})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}



#[delete("/cliente_cnt")]
async fn delete_cliente_cnt(cliente_data: web::Json<DeleteRequest>) -> impl Responder {
    println!("CVE: {:?}", cliente_data.cve);

    let mut connection = establish_connection().await.unwrap();

    let query = format!("DELETE FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CVE = {:?};", cliente_data.cve);

    let result = sqlx::query(&query)
        .execute(&mut connection)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Record deleted successfully"})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/cliente_cnt")]
async fn update_cliente_cnt(cliente_data: web::Json<McClienteCnt>) -> impl Responder {
    println!("CVE: {:?}", cliente_data.cve);

    let cve = cliente_data.cve.unwrap(); // Extrayendo el valor del Option

    // Formatear fecha_cierre correctamente eliminando comillas dobles adicionales
    // let fecha_cierre = cliente_data.fecha_cierre
    //     .as_ref()
    //     .map(|date| {
    //         let cleaned_date = date.trim_matches(|c| c == '\"'); // Eliminar comillas dobles
    //         format!("{}", cleaned_date)
    //     })
    //     .unwrap_or("NULL".to_string());

    // Obtener la fecha y hora actual
    let fecha_actual: DateTime<Utc> = Utc::now();
    // Formatear la fecha y hora
    let fecha_hora_formateada = fecha_actual.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut connection = establish_connection().await.unwrap();

    let query = format!("UPDATE WMS_EC.dbo.MC_CLIENTE_CNT
    SET OPEN_SMARTFLEX = {},
    CL_SAP         = '{}',
    ALMACEN_SAP    = {},
    ESTADO         = {},
    REGIONAL       = '{}',
    CANAL          = {},
    DESCRIPCION_ALMACEN = {},
    DIRECCION = {},
    PROVINCIA = {},
    NOMBRE_CONTACTO = {},
    TELEFONO_CONTACTO = {},
    FECHA_MODIFICACION = '{}'
    WHERE CVE = {:?};",
                        cliente_data.open_smartflex,
                        cliente_data.cl_sap,
                        cliente_data.almacen_sap,
                        cliente_data.estado,
                        cliente_data.regional,
                        cliente_data.canal.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.descripcion_almacen.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.direccion.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.provincia.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.nombre_contacto.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.telefono_contacto.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        fecha_hora_formateada,
                        cve);

    println!("Generated SQL query: {}", query); // Imprimir la consulta SQL generada

    let result = sqlx::query(&query)
        .execute(&mut connection)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Record updated successfully"})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}



#[get("/parroquias")]
async fn get_all_parroquias() -> impl Responder {
    let mut connection = establish_connection().await.unwrap();

    let query = format!("SELECT * FROM MC_WEB_PARROQUIA ORDER BY id asc;");

    let cli: Result<Vec<MCParroquia>, sqlx::Error> = sqlx::query_as(&query)
        .fetch_all(&mut connection)
        .await;

    match cli {
        Ok(clientes) => HttpResponse::Ok().json(json!({"data": clientes})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }




}


#[put("/close_local_cnt")]
async fn close_local_cnt(cliente_data: web::Json<DeleteRequest>) -> impl Responder {
    println!("CVE: {:?}", cliente_data.cve);

    let cve = cliente_data.cve; // Extrayendo el valor del Option

    // Obtener la fecha y hora actual
    let fecha_actual: DateTime<Utc> = Utc::now();
    // Formatear la fecha y hora
    let fecha_hora_formateada = fecha_actual.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut connection = establish_connection().await.unwrap();

    let query = format!("UPDATE WMS_EC.dbo.MC_CLIENTE_CNT
    SET
    FECHA_CIERRE = '{}'
    WHERE CVE = {:?};",
                        fecha_hora_formateada,
                        cve);

    println!("Generated SQL query: {}", query); // Imprimir la consulta SQL generada

    let result = sqlx::query(&query)
        .execute(&mut connection)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "FECHA_CIERRE registrada correctamente."})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}



pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/logistica-nacional")
        .service(get_all_clientes_cnt)
        .service(create_cliente_cnt)
        .service(update_cliente_cnt)
        .service(delete_cliente_cnt)
        .service(get_all_parroquias)
        .service(close_local_cnt)
        ;

    conf.service(scope);
}