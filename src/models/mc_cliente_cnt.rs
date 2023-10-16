use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct McClienteCnt {
    pub cve: Option<i32>,
    pub open_smartflex: i32,
    pub cl_sap: String,
    pub almacen_sap: i32,
    pub fecha_creacion: Option<String>,
    pub fecha_cierre: Option<String>,
    pub estado: i32,
    pub regional: String,
    pub canal: Option<String>,
    pub descripcion_almacen: Option<String>,
    pub direccion: Option<String>,
    pub provincia: Option<String>,
    pub ciudad: Option<String>,
    pub nombre_contacto: Option<String>,
    pub telefono_contacto: Option<String>,
    pub fecha_modificacion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct McClienteCntResult {
    pub cve: Option<i32>,
    pub open_smartflex: i32,
    pub cl_sap: String,
    pub almacen_sap: i32,
    pub fecha_creacion: Option<String>,
    pub estado: i32,
    pub regional: String,
    pub canal: Option<String>,
    pub descripcion_almacen: Option<String>,
    pub direccion: Option<String>,
    pub provincia: Option<String>,
    pub nombre_contacto: Option<String>,
    pub telefono_contacto: Option<String>,
}

#[derive(Debug, Deserialize)] // Only deserialize the 'cve' field
pub struct DeleteRequest {
    pub cve: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MCParroquia {
    pub id: i64,
    pub descripcioncanton: String,
    pub descripcionparroquia: String,
    pub provincia: String
}