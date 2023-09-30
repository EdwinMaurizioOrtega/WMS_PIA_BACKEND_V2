use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

//Muy importante
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub _id: String,
    pub username: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct UserIdentifier {
    pub user_id: String,
    pub user_name: String,
}

