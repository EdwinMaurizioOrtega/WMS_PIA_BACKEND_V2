extern crate dotenv;

use sqlx::Row;
use crate::database::connection::establish_connection;
use crate::models::user_model::{Access, User};

pub async fn get_user_by_email(email: &String) -> Option<User> {
    let mut connection = establish_connection().await.unwrap();

    // Realiza la consulta SQL
    let query = "select * from MC_WEB_USER where EMAIL = @p1";
    let email_param = email.to_string();

    // Ejecuta la consulta y obtén el resultado
    if let Some(row) = sqlx::query(query)
        .bind(&email_param)
        .fetch_optional(&mut connection)
        .await
        .unwrap()
    {
        // Mapea los resultados de la consulta a la estructura UserDetail
        let user_detail = User {
            id: row.get("ID"),
            username: row.get("USERNAME"),
            email: row.get("EMAIL"),
            password: row.get("PASSWORD"),
            created_at: None,
        };
        Some(user_detail)
    } else {
        None
    }
}

pub async fn get_user_by_id(id: i32) -> Option<User> {
    let mut connection = establish_connection().await.unwrap();

    // Realiza la consulta SQL
    let query = "SELECT * FROM WMS_EC.dbo.MC_WEB_USER WHERE ID = @p1";
    let id_param = id;

    // Ejecuta la consulta y obtén el resultado
    if let Some(row) = sqlx::query(query)
        .bind(id_param)
        .fetch_optional(&mut connection)
        .await
        .unwrap()
    {
        // Mapea los resultados de la consulta a la estructura UserDetail
        let user_detail = User {
            id: row.get("ID"),
            username: row.get("USERNAME"),
            email: row.get("EMAIL"),
            password: row.get("PASSWORD"),
            created_at: None,
        };
        Some(user_detail)
    } else {
        None
    }
}

pub async fn get_user_access(id: i32) -> Option<Vec<Access>> {
    let mut connection = establish_connection().await.unwrap();

    // Realiza la consulta SQL
    //let query = "SELECT * FROM WMS_EC.dbo.MC_WEB_USER WHERE ID = @p1";
    let query = "select * from MC_WEB_ACCESS where USER_ID = $1";
    let id_param = id;

    // Ejecuta la consulta y obtén el resultado
    if let Ok(rows) = sqlx::query(query)
        .bind(id_param)
        .fetch_all(&mut connection)
        .await
    {
        // Mapea los resultados de la consulta a la estructura Access y devuelve el vector
        let access_list: Vec<Access> = rows
            .into_iter()
            .map(|row| Access {
                USER_ID: row.get("USER_ID"),
                SUBHEADER: row.get("SUBHEADER"),
                PAGE: row.get("PAGE"),
            })
            .collect();

        Some(access_list)
    } else {
        None
    }
}


