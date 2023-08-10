use serde::Serialize;
use sqlx::FromRow;

// V1
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

// V2
#[derive(Serialize, FromRow)]
pub struct PedidoV2 {
    pub PEDIDO_PROV: i32,
    pub PROCEDENCIA: i32,
    pub ARTICULO: i32,
    pub SERIE: String,
    pub DESCRIPCION: String,
    pub PESO: f64,
}

// V3
#[derive(Serialize, FromRow)]
pub struct PedidoV3 {
    pub PEDIDO_PROV: i32,
    pub PROCEDENCIA: i32,
    pub ARTICULO: i32,
    pub ART_PROCEDE: i32,
    pub CANTIDAD: f64,
    pub DESCRIPCION: String,
    pub PESO: f64,
}


//Sin Fecha
#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub n_pedido: String,
    pub procedencia: String,
}

//Con Fecha
#[derive(serde::Deserialize)]
pub struct QueryDateParams {
    pub proced: i32,
    pub fec_inicio: String,
    pub fec_fin: String,
}
