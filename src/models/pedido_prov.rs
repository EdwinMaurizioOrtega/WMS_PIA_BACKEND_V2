use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tiberius::time::DateTime;

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

#[derive(Debug, serde::Deserialize)]
pub struct ParamsUpdateKgOrden {
    pub num_orden: String,
    pub peso: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ParamsInsertPedidoContrato {
    pub num_orden: String,
    pub peso: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DespachoPedidosFuxionSend {
    pub id: i64,
    pub FECHA_FORMATEADA: Option<String>,
    pub COURIER: Option<String>,
    pub DESCRIPCION: Option<String>,
    pub NUM_PEDIDO: i32,
    pub NUM_CORTE: Option<String>,
    pub GUIA: Option<String>,
    pub PESO: Option<String>,
    pub PESO_REF: Option<String>,
    pub RESPONSABLE: Option<String>,
    pub ESTATUS: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InventarioReporteFuxionSend {
    pub id: i64,
    pub COD_ART: i32,
    pub UNI_MEDIDA: Option<String>,
    pub NOM_PRODUCTO: Option<String>,
    pub CANTIDAD: i32,
    pub OBS: Option<String>,
    pub NOTAS: Option<String>,
    pub ALMACEN: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FullReporteDespachosConsolidados {
    pub TIPO_ART: Option<String>,
    pub ALMACEN: Option<String>,
    pub PROPIEDAD: Option<String>,
    pub TIPO: Option<String>,
    pub TECNOLOGIA: Option<String>,
    pub COD_OPEN: Option<i32>,
    pub COD_SAP: Option<String>,
    pub DESCRIPCION: Option<String>,
    pub SERIE: Option<String>,
    pub ICC: Option<String>,
    pub MIN: Option<String>,
    pub IMSI: Option<String>,
    pub FECHA: Option<String>,
    pub BODEGA_OPEN: Option<String>,
    pub COD_DIRECTO: Option<String>,
    pub ALMACEN_SAP: Option<String>,
    pub BOD_SAP_ALM_SAP: Option<String>,
    pub DESCR_BODEGA: Option<String>,
    pub CANAL: Option<String>,
    pub GUIA: Option<String>,
    pub PEDIDO_SAP: Option<String>,
    pub COD_INTERNO: Option<i32>,
    pub SALIDA_SAP: Option<String>,
    pub OBSERVACION: Option<String>,
    pub PEDIDO_PIA: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FullReporteDespachosSinSeries {
    pub TIPO_ANIDADO_ARTICULO: Option<String>,
    pub GUIA_REMISION: Option<String>,
    pub FECHA: Option<String>,
    pub BODEGA_OPEN: Option<String>,
    pub COD_DIRECTO: Option<String>,
    pub ALMACEN_SAP: Option<String>,
    pub BOD_SAP_ALM_SAP: Option<String>,
    pub DESCR_BODEGA: Option<String>,
    pub ANIDADO_PESO_TOTAL: Option<f64>,
    pub ARTICULO_ANIDADO: Option<i32>,
    pub ANIDADO_ARTICULO: Option<String>,
    pub TOTAL_DESPACHADO: Option<f64>,
    pub PIA: Option<i32>,
    pub ANIDADO_COD_SAP: Option<String>,
    pub PEDIDO_SAP: Option<String>,
    pub PESO: Option<f64>,
    pub COD_INTERNO: Option<i32>,
    pub ARTICULO: Option<String>,
    pub COD_SAP: Option<String>,
    pub TIPO_ARTICULO: Option<String>,

}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FullReporteFormatoGuias {
    pub NUMERO_GUIA: Option<String>,
    pub DETALLE_ENVIO_1: Option<i32>,
    pub DETALLE_ENVIO_2: Option<String>,
    pub DETALLE_ENVIO_3: Option<String>,
    pub CIUDAD_DESTINO: Option<String>,
    pub SECTOR: Option<String>,
    pub CODIGO_DESTINATARIO: Option<String>,
    pub RAZON_SOCIAL_DESTINATARIO: Option<String>,
    pub NOMBRE_DESTINATARIO: Option<String>,
    pub APELLIDO_DESTINATARIO: Option<String>,
    pub DIRECCION1_DESTINATARIO: Option<String>,
    pub TELEFONO1_DESTINATARIO: Option<String>,
    pub TELEFONO2_DESTINATARIO: Option<String>,
    pub CODIGO_POSTAL_DESTINATARIO: Option<String>,
    pub PRODUCTO: Option<String>,
    pub ART_TIPO: Option<String>,
    pub CANTIDAD: Option<f64>,
    pub PIEZAS: Option<String>,
    pub TOTAL: Option<f64>,
    pub VALOR_ASEGURADO: Option<String>,
    pub LARGO: Option<String>,
    pub ANCHO: Option<String>,
    pub ALTO: Option<String>,
    pub PESO: Option<String>,
    pub NUMERO_GUIA_SOBRERETORNO: Option<String>,
    pub FECHA_FACTURA: Option<String>,
    pub NUMERO_FACTURA: Option<String>,
    pub VALOR_FACTURA: Option<String>,
    pub DETALLE_ITEMS_FACTURA: Option<String>,
    pub VERIFICAR_CONTENIDO_RECAUDO: Option<String>,
    pub VALOR_FLETE_RECAUDO: Option<String>,
    pub VALOR_COMISION_RECAUDO: Option<String>,
    pub VALOR_SEGURO_RECAUDO: Option<String>,
    pub VALOR_IMPUESTO_RECAUDO: Option<String>,
    pub VALOR_OTROS_RECAUDO: Option<String>,

}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FullReporteInventarioInicialBodega {
    pub COD_OPEN: Option<i32>,
    pub COD_SAP: Option<String>,
    pub DESCRIPCION: Option<String>,
    pub CANTIDAD: Option<i32>,
    pub TIPO: Option<String>,
    pub PROPIEDAD: Option<String>,
    pub ESTADO: Option<String>,
    pub DESCR_UBICACION: Option<String>,
    pub TECNOLOGIA: Option<String>,
    pub MARCA: Option<String>,
    pub GAMA: Option<String>,
    pub UBICACION: Option<String>,
    pub TIPO_ARTICULO: Option<String>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FullReporteInventarioInicialInterno {
    pub COD_OPEN: Option<i32>,
    pub COD_SAP: Option<String>,
    pub DESCRIPCION: Option<String>,
    pub CANTIDAD: Option<i32>,
    pub TIPO: Option<String>,
    pub PROPIEDAD: Option<String>,
    pub ESTADO: Option<String>,
    pub DESCR_UBICACION: Option<String>,
    pub TECNOLOGIA: Option<String>,
    pub MARCA: Option<String>,
    pub GAMA: Option<String>,
    pub UBICACION: Option<String>,
    pub COD_PIA: Option<i32>,
    pub TIPO_ARTICULO: Option<String>
}