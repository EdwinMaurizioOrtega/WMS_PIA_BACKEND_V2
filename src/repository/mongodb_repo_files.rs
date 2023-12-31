use std::env;
use std::fs::File;

extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use mongodb::bson::from_document;
use crate::models::files::{Files, SelectedFile};

use mongodb::error::{Error as MongoError, ErrorKind};
use crate::database::connection::establish_connection;
use crate::models::mc_cliente_cnt::McClienteCnt;
use crate::models::user_model::User;


pub struct MongoRepo {
    col: Collection<Files>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("rustDB");
        let col: Collection<Files> = db.collection("Files");
        MongoRepo { col }
    }

    pub async fn create_registro(&self, new_pre_registro: Files) -> Result<InsertOneResult, Error> {
        println!("{:?}", new_pre_registro);

        let selected_files = new_pre_registro
            .selected_file
            .into_iter()
            .map(|sf| SelectedFile {
                file_name: sf.file_name,
                file_type: sf.file_type,
                file_url: sf.file_url,
            })
            .collect::<Vec<_>>();

        let new_doc = Files {
            id: None,
            pedido_proveedor: new_pre_registro.pedido_proveedor,
            procedencia: new_pre_registro.procedencia,
            dn: new_pre_registro.dn,
            description: new_pre_registro.description,
            selected_file: selected_files,
            created_at: new_pre_registro.created_at,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(user)
    }


    pub async fn get_pedido_files(&self, n_pedido: &String, procedencia: &String) -> Result<Vec<Files>, Error> {
        let filter = doc! {"pedido_proveedor": n_pedido, "procedencia": procedencia};
        let mut cursors = self
            .col
            .find(filter, None)
            .await
            .ok()
            .expect("Error getting list of users");

        let mut users: Vec<Files> = Vec::new();

        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }

    pub async fn get_dn_files(&self, n_pedido: &String, procedencia: &String) -> Result<Vec<Files>, Error> {
        let filter = doc! {"dn": n_pedido, "procedencia": procedencia};
        let mut cursors = self
            .col
            .find(filter, None)
            .await
            .ok()
            .expect("Error getting list of users");

        let mut users: Vec<Files> = Vec::new();

        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }


    pub async fn get_pedido_and_dn_files(&self, n_pedido: &String, dn: &String, procedencia: &String) -> Result<Vec<Files>, Error> {
        let filter = doc! {"pedido_proveedor": n_pedido, "dn": dn, "procedencia": procedencia};
        let mut cursors = self
            .col
            .find(filter, None)
            .await
            .ok()
            .expect("Error getting list of users");

        let mut users: Vec<Files> = Vec::new();

        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }


    pub async fn delete_image_one(&self, id: &String, file_name: &String) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };

        let update = doc! { "$pull": { "selected_file": { "file_name": file_name } } };

        let result = self
            .col
            .update_one(filter, update, None)
            .await
            .ok()
            .expect("Error updating user");

        Ok(result)
    }


    pub async fn get_all_clientes_cnt(&self) -> Result<Vec<McClienteCnt>, Error> {

        let mut connection = establish_connection().await.unwrap();

        let query = format!(
            "SELECT * FROM McClienteCnt ORDER BY cve ASC");

        let pedidos: Vec<McClienteCnt> = sqlx::query_as::<_, McClienteCnt>(&query)
            .fetch_all(&mut connection)
            .await
            .unwrap();

        Ok(pedidos)
    }

}