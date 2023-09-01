use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct McClienteCnt {
    pub cve: Option<i32>,
    pub open_smartflex: i32,
    pub cl_sap: String,
    pub almacen_sap: i32,
    pub fecha_creacion: String,
    pub fecha_cierre: Option<String>,
    pub estado: i32,
    pub regional_canal: String,
}

#[derive(Debug, Deserialize)] // Only deserialize the 'cve' field
pub struct DeleteRequest {
    pub cve: i32,
}