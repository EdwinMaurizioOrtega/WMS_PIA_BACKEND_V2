use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct PedidoProv {
    pub PEDIDO_PROV: i32,
    pub DATO3: String,
}
