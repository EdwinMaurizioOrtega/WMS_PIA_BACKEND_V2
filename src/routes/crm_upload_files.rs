use std::{env, fs};
use std::fs::{File, read};
use std::io::Write;
use std::path::{Path, PathBuf};
use actix_multipart::Multipart;
use actix_web::{Error, get, HttpResponse, post, web};
use actix_web::web::{Data, Json};
use chrono::{Datelike, Utc};
use futures::StreamExt;
use mongodb::bson::DateTime;
use crate::{models::files::Files};
use crate::repository::mongodb_repo_files::MongoRepo;

//Guardar los detalles de las imajenes mas los nombres de los archivos
#[post("/cargar_imagenes")]
async fn create_registro_imagen(db: Data<MongoRepo>, new_pre_registro: Json<Files>) -> HttpResponse  {
    // Utilizamos println! para imprimir el contenido de new_pre_registro
    println!("Llega: {:?}", new_pre_registro);

    let selected_file_cloned = new_pre_registro.selected_file.iter().cloned().collect::<Vec<_>>();

    let mut data = Files {
        id: None,
        pedido_proveedor: new_pre_registro.pedido_proveedor.to_owned(),
        procedencia: new_pre_registro.procedencia.to_owned(),
        description: new_pre_registro.description.to_owned(),
        selected_file: selected_file_cloned,
        created_at: None,

    };

    // Asignar la fecha y hora actual antes de guardar el documento
    data.created_at = Some(DateTime::now());

    let pre_files = db.create_registro(data).await;

    match pre_files {
        Ok(preregistro) => HttpResponse::Ok().json(preregistro),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}

//Guardar los archivos en una ruta especifica en el almacenamiento local
#[post("/upload_web_files")]
async fn upload_file(mut payload: Multipart,) -> HttpResponse  {
    // Utilizamos println! para imprimir el contenido de new_pre_registro
    //println!("Llega: {:?}", new_pre_registro);

    let current_time = Utc::now();
    let year = current_time.year().to_string();
    let month = format!("{:02}", current_time.month());

    let uploads_path = env::var("UPLOADS_PATH").unwrap();
    let base_dir = Path::new(&uploads_path);

    // Crea la estructura de carpetas si no existen
    let year_dir = base_dir.join(&year);
    let month_dir = year_dir.join(&month);
    fs::create_dir_all(&month_dir).unwrap();

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap_or("unknown");

        let file_path = month_dir.join(filename);

        let mut file = File::create(file_path).unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file.write_all(&data).unwrap();
        }
    }

    HttpResponse::Ok().json("Archivos recibidos y guardados correctamente")


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

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/mogo-db-wms")
        .service(create_registro_imagen)
        .service(upload_file)
        .service(serve_file);

    conf.service(scope);
}