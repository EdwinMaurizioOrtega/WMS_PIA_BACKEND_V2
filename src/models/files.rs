use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use mongodb::bson::DateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub pedido_proveedor: String,
    pub procedencia: String,
    pub description: String,
    pub selected_file: Vec<SelectedFile>,
    //Fecha en la que se agrego el registro a la DB
    pub created_at: Option<DateTime>,

}

#[derive(Debug, Serialize, Deserialize, Clone)]// Agrega "Clone" aquí
pub struct SelectedFile {
    pub file_name: String,
    pub file_type: String,
    pub file_url: String
}