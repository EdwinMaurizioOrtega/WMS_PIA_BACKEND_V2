use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct PedidoProv {
    pub PEDIDO_PROV: i32,
    pub FEC_INGRESO: String,
    pub USUARIO: String,
    pub ESTATUS: String,
    pub CLIENTE: String,
    pub PROVEEDOR: String,
    pub DESCRIPCION: String,
    pub DATO1: String,
    pub DATO2: String,
    pub DATO3: String,
    pub DATO4: String,
    pub DATO5: String,
    pub FACTURA: String,
    pub FACTURA_FAB: String,
    pub BULTOS: f64,
    pub VAL1: f64,
    pub VAL2: f64,
    pub PESO: f64,
}


#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub n_pedido: i32,
    pub procedencia: String,
}
