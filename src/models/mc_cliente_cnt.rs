use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct McClienteCnt {
    pub cve: Option<i32>,
    pub open_smartflex: Option<String>,
    pub cl_sap: String,
    pub almacen_sap: Option<String>,
    pub fecha_creacion: Option<String>,
    pub fecha_cierre: Option<String>,
    pub estado: i32,
    pub regional: String,
    pub canal: Option<String>,
    pub descripcion_almacen: Option<String>,
    pub direccion: Option<String>,
    pub provincia: Option<String>,
    pub nombre_contacto: Option<String>,
    pub telefono_contacto: Option<String>,
    pub fecha_modificacion: Option<String>,
    pub cl_sap_indirecto: Option<String>,
    pub correo: Option<String>,
    pub tiempo_entrega: Option<String>,
    pub user_update: Option<String>,
    pub temp_loc_fecha_cierre: Option<String>,
    pub temp_loc_fecha_apertura: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct McClienteCntResult {
    pub cve: Option<i32>,
    pub open_smartflex: Option<String>,
    pub cl_sap: String,
    pub almacen_sap: Option<String>,
    pub fecha_creacion: Option<String>,
    pub estado: i32,
    pub regional: String,
    pub canal: Option<String>,
    pub descripcion_almacen: Option<String>,
    pub direccion: Option<String>,
    pub provincia: Option<String>,
    pub nombre_contacto: Option<String>,
    pub telefono_contacto: Option<String>,
    pub cl_sap_indirecto: Option<String>,
    pub correo: Option<String>,
    pub tiempo_entrega: Option<String>,
}

#[derive(Debug, Deserialize)] // Only deserialize the 'cve' field
pub struct DeleteRequest {
    pub cve: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MCParroquia {
    pub ID_CIUDAD: i64,
    pub NOMBRE_CIUDAD: String,
    pub NOMBRE_PROVINCIA: String,

}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct McClienteCntAux {
    pub CVE: i32,
    pub ESTADO: i32,
    pub CANAL: String,
    pub DESCRIPCION_ALMACEN: String,
    pub REGIONAL: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MC_WEB_PROVINCIAS_CIUDADES {
    pub ID_CIUDAD: i64,
    pub NOMBRE_CIUDAD: String,
    pub NOMBRE_PROVINCIA: String,
}