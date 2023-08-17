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

use mongodb::error::Error as MongoError;


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
            created_at:  new_pre_registro.created_at,
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


}