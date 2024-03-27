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


// V4
#[derive(Serialize, FromRow)]
pub struct PedidoV4 {
    pub PEDIDO_PROV: i32,
    pub FEC_INGRESO: String,
    pub FEC_ALTA: String,
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
    pub VAL1: f64,
    pub VAL2: f64,
    pub PESO: f64,
}

#[derive(Serialize, FromRow)]
pub struct PedidoV5 {
    pub PEDIDO_PROV: i32,
    pub FEC_INGRESO: String,
    pub FEC_ALTA: String,
    pub USUARIO: String,
    pub ESTATUS: String,
    pub CLIENTE: String,
    pub PROVEEDOR: String,
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
    pub DESCRIPCION_V2: String,
    pub CANTIDAD: f64,
    pub DATA_DET1: String,
    pub COSTO: f64,
    pub ARTICULO: i32,
}

#[derive(Serialize, FromRow)]
pub struct PedidoV6 {
    pub NUM_PEDIDO: i32,
    pub PROCEDENCIA: i32,
    pub FECHA: String,
    pub CONTACTO: String,
    pub TEL_CONTACTO: String,
    pub CANTIDAD: f64,
    pub TOTAL: f64,
    pub CANTON: String,
    pub PROVINCIA: String,
    pub DESCRIPCION: String,
    pub CONTRATO: String,
    pub BULTOS: i32
}

#[derive(Serialize, FromRow)]
pub struct PedidoV7 {
    pub NUM_PEDIDO: i32,
    pub PROCEDENCIA: i32,
    pub ARTICULO: i32,
    pub CANTIDAD: f64,
    pub TOTAL: f64,
    pub DESCRIPCION: String,
    pub ART_TIPO: i32,
    pub DESCRIPCION_2: String,
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


#[derive(serde::Deserialize)]
pub struct QueryParamsPedidoAndDN {
    pub n_pedido: String,
    pub procedencia: String,
    pub dn: String,
}


#[derive(serde::Deserialize)]
pub struct QueryParamsDeleteImage {
    pub id: String,
    pub file_name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ParamsUpdateGuiaPDF {
    pub n_pedido: String,
    pub num_guia: String,
    pub url_guia: String
}
