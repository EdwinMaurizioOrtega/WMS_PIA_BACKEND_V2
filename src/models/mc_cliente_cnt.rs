use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct McClienteCnt {
    pub CVE: Option<i32>,
    pub OPEN_SMARTFLEX: i32,
    pub CL_SAP: String,
    pub ALMACEN_SAP: i32,
    pub FECHA_CREACION: String,
    pub FECHA_CIERRE: String,
    pub ESTADO: i32,
    pub REGIONAL_CANAL: String,

}