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
    pub COD_ANTIGUO: Option<String>,
    pub DESCRIPCION: Option<String>,
    pub PRE_PAGO_MERCH: Option<i32>,
    pub BTL_MERCH: Option<i32>,
    pub PUBLICIDAD: Option<i32>,
}


#[derive(Deserialize, Clone)]
pub struct QueryParamsSaveUrlImgProduct {
    pub COD_PROD: String,
    pub URL: String,

}

#[derive(serde::Deserialize)]
pub struct QueryParamImageURL {
    pub URL: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ParkenorProductoDetalle {
    pub IMAGEN: Option<String>,
    pub DESCRIPCION: Option<String>,
    pub COD_PIA: Option<i32>,
    pub COD_ANTIGUO: Option<String>,
    pub PRE_PAGO_MERCH: Option<i32>,
    pub BTL_MERCH: Option<i32>,
    pub PUBLICIDAD: Option<i32>,
    pub UBICACION: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct QueryParamCodArticulo {
    pub cod_articulo: String,
}