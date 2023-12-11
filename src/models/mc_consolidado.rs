use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MC_WEB_CONSOLIDADO_CARGA_PEDIDOS {
    pub FECHA: String,
    pub PEDIDO_TRASLADO: String,
    pub CENTRO_SUMINISTRADOR: String,
    pub ALMACEN_EMISOR: String,
    pub MATERIAL: String,
    pub DESCRIPCION_MATERIAL: String,
    pub CANTIDAD: String,
    pub CENTRO: String,
    pub DESCRIPCION_CENTRO: String,
    pub CIUDAD: String,
    pub GUIA_COURIER: String,
    pub ALMACEN_RECEPTOR: String,
    pub COURIER: String,
    pub CANAL: String,
    pub COD_CLIENTE: String,
    pub OBSERVACION: String,
    pub CATEGORIA: String,
    pub ARTICULO: String,
    pub ALMACEN: String,
    pub PEDIDO: String,
}