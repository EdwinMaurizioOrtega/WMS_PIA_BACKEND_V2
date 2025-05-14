use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ParkenorImagenes {
    pub IMAGEN: String,
    pub ARTICULO: i32,
    pub DESCRIPCION: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ParkenorProducts {
    pub IMAGEN: Option<String>,
    pub ARTICULO: Option<i32>,
    pub DESCRIPCION: Option<String>,
    pub PRE_PAGO_MERCH: Option<i32>,
    pub BTL_MERCH: Option<i32>,
    pub PUBLICIDAD: Option<i32>,
}