use actix_web::{delete, Error, get, HttpResponse, post, put, Responder, web};
use chrono::{DateTime, Utc};
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
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
       nombre_contacto,
       telefono_contacto,
       fecha_modificacion,
        cl_sap_indirecto,
        correo,
        tiempo_entrega
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
    println!("open_smartflex: {}", cliente_data.open_smartflex.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()));
    println!("cl_sap: {}", &cliente_data.cl_sap);
    println!("almacen_sap: {}", cliente_data.almacen_sap.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()));

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
    telefono_contacto,
    cl_sap_indirecto,
    correo,
    tiempo_entrega)
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
                 INSERTED.telefono_contacto,
                 INSERTED.cl_sap_indirecto,
                 INSERTED.correo,
                 INSERTED.tiempo_entrega
                 VALUES (@p1, @p2, @p3, @p4, @p5, @p6, @p7, @p8, @p9, @p10, @p11, @p12, @p13, @p14, @p15)";

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
        .bind(&cliente_data.cl_sap_indirecto)
        .bind(&cliente_data.correo)
        .bind(&cliente_data.tiempo_entrega)
        .fetch_one(&mut connection)
        .await;

    match cli {
        Ok(clientes) => {

            //1.Enviamos la notificacion por correo
            let creds = Credentials::new("sistemas@movilcelistic.com".to_owned(), "gt5P4&M#C74c".to_owned());

            // Open a remote connection to gmail
            let mailer = SmtpTransport::starttls_relay("smtp.office365.com")
                .unwrap()
                .port(587)
                .credentials(creds)
                .build();

            // Llamar a la función notificar_correos
            let correos = notificar_correos();

            // Iterar sobre el vector y mostrar cada dirección de correo
            for correo in correos {
                println!("Notificando a: {}", correo);

                let asusnto: String;  // Declarar la variable aquí para que sea válida en todo el bloque

                asusnto = format!("CREACIÓN PUNTO DE VENTA: {:?}", clientes.descripcion_almacen.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()));

                let mensaje = format!("Estimados,\n Se confirma la creación del siguiente punto de venta:\n\n CLIENTE: {} ", clientes.descripcion_almacen.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()));

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

            //2.Response de los datos creados
            HttpResponse::Ok().json(json!({"data": clientes}))
        }
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

//Actualizar el punto de venta CNT
#[put("/cliente_cnt")]
async fn update_cliente_cnt(cliente_data: web::Json<McClienteCnt>) -> impl Responder {
    println!("CVE: {:?}", cliente_data.cve);

    //Abrimos la conexión a la base de datos
    let mut connection = establish_connection().await.unwrap();

    let cve = cliente_data.cve.unwrap(); // Extrayendo el valor del Option

    //Lógica para comparar que datos se actualizaron

    //let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CVE = {:?};", cve);

    //println!("Generated SQL query: {}", query); // Imprimir la consulta SQL generada
    //
    // let result: Result<McClienteCntResult, sqlx::Error> = sqlx::query_as(&query)
    //     .fetch_one(&mut connection)
    //     .await;
    //
    // match result {
    //     Ok(rows) => {
    //         println!("Error al encontrar el registro: {:?}", rows);
    //     },
    //     Err(err) => {
    //         println!("Error al encontrar el registro.");
    //     },
    // }




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
    FECHA_MODIFICACION = '{}',
    CL_SAP_INDIRECTO = '{}',
    CORREO = '{}',
    TIEMPO_ENTREGA = '{}'
    WHERE CVE = {:?};",
                        cliente_data.open_smartflex.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()),
                        cliente_data.cl_sap,
                        cliente_data.almacen_sap.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()),
                        cliente_data.estado,
                        cliente_data.regional,
                        cliente_data.canal.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.descripcion_almacen.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.direccion.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.provincia.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.nombre_contacto.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.telefono_contacto.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        fecha_hora_formateada,
                        cliente_data.cl_sap_indirecto.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.correo.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                        cliente_data.tiempo_entrega.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>

                        cve);

    println!("Generated SQL query: {}", query); // Imprimir la consulta SQL generada

    let result = sqlx::query(&query)
        .execute(&mut connection)
        .await;

    match result {
        Ok(_) => {

            //1.Enviamos la notificacion por correo
            let creds = Credentials::new("sistemas@movilcelistic.com".to_owned(), "gt5P4&M#C74c".to_owned());

            // Open a remote connection to gmail
            let mailer = SmtpTransport::starttls_relay("smtp.office365.com")
                .unwrap()
                .port(587)
                .credentials(creds)
                .build();

            // Llamar a la función notificar_correos
            let correos = notificar_correos();

            // Iterar sobre el vector y mostrar cada dirección de correo
            for correo in correos {
                println!("Notificando a: {}", correo);

                let asusnto: String;  // Declarar la variable aquí para que sea válida en todo el bloque

                asusnto = format!("ACTUALIZACIÓN PUNTO DE VENTA: {:?}", cliente_data.descripcion_almacen.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                );

                let mensaje = format!("Estimados,\n Se confirma la actualización del siguiente punto de venta:\n\n CLIENTE: {} ", cliente_data.descripcion_almacen.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string()), // Manejar Option<String>
                );

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

            //2. Respuesta JSON
            HttpResponse::Ok().json(json!({"message": "Record updated successfully"}))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[get("/parroquias")]
async fn get_all_parroquias() -> impl Responder {
    let mut connection = establish_connection().await.unwrap();

    let query = format!("SELECT * FROM MC_WEB_PROVINCIAS_CIUDADES ORDER BY ID_CIUDAD asc;");

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

//Correos activos para enviar las notificaciones
fn notificar_correos() -> Vec<String> {
    let correo1 = "aibarram@movilcelistic.com".to_string();
    //let correo2 = "sistemas@hipertronics.us".to_string();

    // Devolver un vector con las direcciones de correo
    vec![correo1]
}