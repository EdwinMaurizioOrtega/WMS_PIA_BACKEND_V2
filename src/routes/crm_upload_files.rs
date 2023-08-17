use std::{env, fs};
use std::fs::{File, read};
use std::io::Write;
use std::path::{Path, PathBuf};
use actix_multipart::{Field, Multipart};
use actix_web::{Error, get, HttpResponse, post, Responder, web};
use actix_web::web::{Data, Json};
use chrono::{Datelike, Utc};
use dotenv::dotenv;
use futures::StreamExt;
use mongodb::bson::{DateTime, to_bson};
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::AsyncWriteExt;
use crate::{models::files::Files};
use crate::models::files::SelectedFile;
use crate::models::pedido_prov::QueryParams;
use crate::repository::mongodb_repo_files::MongoRepo;

//Guardar los detalles de las imajenes mas los nombres de los archivos
// #[post("/cargar_imagenes")]
// async fn create_registro_imagen(db: Data<MongoRepo>, new_pre_registro: Json<Files>) -> HttpResponse  {
//     // Utilizamos println! para imprimir el contenido de new_pre_registro
//     println!("Llega: {:?}", new_pre_registro);
//
//     let selected_file_cloned = new_pre_registro.selected_file.iter().cloned().collect::<Vec<_>>();
//
//     let mut data = Files {
//         id: None,
//         pedido_proveedor: new_pre_registro.pedido_proveedor.to_owned(),
//         procedencia: new_pre_registro.procedencia.to_owned(),
//         description: new_pre_registro.description.to_owned(),
//         selected_file: selected_file_cloned,
//         created_at: None,
//
//     };
//
//     // Asignar la fecha y hora actual antes de guardar el documento
//     data.created_at = Some(DateTime::now());
//
//     let pre_files = db.create_registro(data).await;
//
//     match pre_files {
//         Ok(preregistro) => HttpResponse::Ok().json(preregistro),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
//
// }

//Guardar los archivos en una ruta especifica en el almacenamiento local
#[post("/upload_web_files")]
async fn upload_file(mut payload: Multipart, db: Data<MongoRepo>) -> Result<HttpResponse, actix_web::Error> {
    dotenv().ok(); // Cargar las variables de entorno desde el archivo .env

    let mut form = Files {
        id: None,
        pedido_proveedor: String::new(),
        procedencia: String::new(),
        dn: String::new(),
        description: String::new(),
        selected_file: Vec::new(),
        created_at: None,
    };


    form.created_at = Some(DateTime::now());

    let current_time = Utc::now();
    let year = current_time.year().to_string();
    let month = format!("{:02}", current_time.month());

    let uploads_path = match env::var("UPLOADS_PATH") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };

    println!("Ruta: {:?}", uploads_path);

    let base_dir = Path::new(&uploads_path);

    // Crea la estructura de carpetas si no existen
    let year_dir = base_dir.join(&year);
    let month_dir = year_dir.join(&month);
    fs::create_dir_all(&month_dir).unwrap();

    // Validamos la ruta
    let url = match env::var("HOST") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();

        if let Some(name) = content_type.get_name() {
            match name {
                "pedidoProveedor" => {
                    let bytes = field.next().await.unwrap().unwrap(); // Obtiene Bytes
                    form.pedido_proveedor = String::from_utf8_lossy(&bytes).to_string(); // Convierte a String
                }
                "procedencia" => {
                    let bytes = field.next().await.unwrap().unwrap();
                    form.procedencia = String::from_utf8_lossy(&bytes).to_string();
                }
                "dn" => {
                    let bytes = field.next().await.unwrap().unwrap();
                    form.dn = String::from_utf8_lossy(&bytes).to_string();
                }
                "description" => {
                    let bytes = field.next().await.unwrap().unwrap();
                    form.description = String::from_utf8_lossy(&bytes).to_string();
                }
                "selectedFile" => {
                    let filename = content_type.get_filename().unwrap();

                    // Obtener la extensión del archivo
                    let file_ext = Path::new(filename)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("");

                    // Generar un nombre de archivo aleatorio
                    let rand_filename: String = thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(20) // Cambia este número a la longitud deseada del nombre
                        .map(char::from)
                        .collect();

                    // Concatenar el nombre de archivo aleatorio con la extensión
                    let full_filename = if file_ext.is_empty() {
                        rand_filename
                    } else {
                        format!("{}.{}", rand_filename, file_ext)
                    };

                    //No mover de aqui -_-
                    let selected_file = SelectedFile {
                        file_name: full_filename.to_string(),
                        file_type: file_ext.to_string(),
                        file_url: format!("{}/api/mogo-db-wms/uploads/{}/{}/{}", url, year, month, full_filename),
                    };

                    let file_path = month_dir.join(full_filename);

                    let mut file = File::create(file_path).unwrap();
                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        file.write_all(&data).unwrap();
                    }
                    //Aquí
                    form.selected_file.push(selected_file);
                }
                _ => (),
            }
        }
    }

    // // Imprime los valores de los campos por consola
    println!("Valor de pedidoProveedor: {}", form.pedido_proveedor);
    println!("Valor de procedencia: {}", form.procedencia);
    println!("Valor de description: {}", form.description);


    // Crear un documento con los datos
    let mut new_document = Files {
        id: None,
        pedido_proveedor: form.pedido_proveedor,
        procedencia: form.procedencia,
        dn: form.dn,
        description: form.description,
        selected_file: form.selected_file,
        created_at: None,
    };

    // Asignar la fecha y hora actual antes de guardar el documento
    new_document.created_at = Some(DateTime::now());

    // Convertir el documento a BSON
    let new_document_bson = new_document;
    // Imprimir el contenido del documento BSON en la consola
    println!("Contenido del documento BSON: {:?}", new_document_bson);

    let pre_files = db.create_registro(new_document_bson).await;

    // match pre_files {
    //     Ok(preregistro) => HttpResponse::Ok().json(preregistro),
    //     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    // }


    Ok(HttpResponse::Ok().json("Archivos recibidos y guardados correctamente"))
}

//Servir las imagenes en navegador web
#[get("/uploads/{year}/{month}/{filename:.+}")]
async fn serve_file(path: web::Path<(String, String, String)>) -> Result<HttpResponse, Error> {
    let (year, month, filename) = path.into_inner();

    let uploads_path = env::var("UPLOADS_PATH").unwrap();
    let base_path = Path::new(&uploads_path);

    let folder_path = base_path.join(&year).join(&month);
    let file_path = folder_path.join(&filename);

    if let Ok(file_data) = read(file_path.clone()) {
        let content_type = match file_path.extension() {
            Some(ext) if ext == "jpg" => "image/jpeg",
            Some(ext) if ext == "png" => "image/png",
            Some(ext) if ext == "pdf" => "application/pdf",
            _ => "application/octet-stream", // Tipo genérico si no se reconoce la extensión
        };

        Ok(HttpResponse::Ok()
            .content_type(content_type)
            .body(file_data))
    } else {
        Ok(HttpResponse::NotFound().body("Archivo no encontrado"))
    }
}

//Listamos todas las imágenes y PDFs
#[get("/lista_imagenes")]
async fn get_list_image_by_proveedor_and_procedencia(query_params: web::Query<QueryParams>, db: Data<MongoRepo>) -> impl Responder {
    println!("pedidoProveedor: {}", query_params.n_pedido);
    println!("procedencia: {}", query_params.procedencia);


    let user_detail_result = db.get_files(&query_params.n_pedido, &query_params.procedencia).await;

    // Crear la respuesta JSON
    let user_response = match user_detail_result {
        Ok(users) => json!({"data": users}),
        Err(err) => json!({"error": err.to_string()}), // Convertir el error a una cadena
    };

    // Enviar la respuesta
    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}


//Listamos todas las imágenes y PDFs
#[get("/dn_lista_imagenes")]
async fn get_list_image_by_proveedor_and_dn(query_params: web::Query<QueryParams>, db: Data<MongoRepo>) -> impl Responder {
    println!("pedidoProveedor: {}", query_params.n_pedido);
    println!("procedencia: {}", query_params.procedencia);


    let user_detail_result = db.get_files(&query_params.n_pedido, &query_params.procedencia).await;

    // Crear la respuesta JSON
    let user_response = match user_detail_result {
        Ok(users) => json!({"data": users}),
        Err(err) => json!({"error": err.to_string()}), // Convertir el error a una cadena
    };

    // Enviar la respuesta
    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/mogo-db-wms")
        // .service(create_registro_imagen)
        .service(upload_file)
        .service(serve_file)
        .service(get_list_image_by_proveedor_and_procedencia)
        .service(get_list_image_by_proveedor_and_dn);

    conf.service(scope);
}