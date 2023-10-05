use crate::repository::mssql_repo::{get_user_access, get_user_by_email, get_user_by_id};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, TokenData, DecodingKey, Validation, decode};

use serde_json::{json, Value};

use chrono::{prelude::*, Duration};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, HttpRequest};

use crate::models::user_model::{LoginUserSchema, TokenClaims};

#[post("/login")]
async fn signin(new_user: web::Json<LoginUserSchema>) -> impl Responder {
    println!("Contenido de data: {:?}", new_user);

    if new_user.email.is_empty() {
        return HttpResponse::BadRequest().body("invalid Email");
    }

    let query_result = get_user_by_email(&new_user.email).await;

    match query_result {
        Some(user) => {
            println!("Contenido de data: {:?}", user);

            // Compare the passwords directly
            let is_valid = new_user.password == user.password;

            if !is_valid {
                return HttpResponse::BadRequest()
                    .json(json!({"status": "fail", "message": "Correo electrónico o contraseña no válidos"}));
            }

            // Generate JWT token and set it in the cookie
            let now = Utc::now();
            let iat = now.timestamp() as usize;
            let exp = (now + Duration::minutes(60)).timestamp() as usize;

            let claims = TokenClaims {
                _id: user.id.unwrap().to_string(),
                username: user.username.to_string(),
                exp,
                iat,
            };

            let secret = "@#2023MovilCelistic";

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
            )
                .unwrap();

            HttpResponse::Ok()
                .json(json!({"status": "success", "user": claims, "accessToken": token}))
        }
        None => {
            HttpResponse::NotFound().json(json!({"status": "fail", "message": "El usuario no existe"}))
        }
    }
}


#[get("/my-account")]
async fn my_account(req: HttpRequest) -> impl Responder {

    println!("Authorization: {:?}", req.headers().get("authorization"));

    // Extrae el token de autorización del encabezado
    let access_token = match req.headers().get("authorization") {

        Some(header) => match header.to_str() {
            Ok(token) => {
                // Quitar "Bearer " del token, si está presente
                let token = if token.starts_with("Bearer ") {
                    &token[7..]
                } else {
                    token
                };

                token
            },
            Err(_) => {
                return HttpResponse::Unauthorized().json(json!({
                    "message": "Error extracting authorization token"
                }));
            }
        },
        None => {
            return HttpResponse::Unauthorized().json(json!({
                "message": "Authorization token missing"
            }));
        }
    };

    // Decodifica el token JWT
    let token_data = match decode::<TokenClaims>(
        access_token,
        &DecodingKey::from_secret("@#2023MovilCelistic".as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "message": "Invalid authorization token"
            }));
        }
    };

    // Parsea el ID de usuario
    let user_id = match token_data.claims._id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "message": "Invalid user ID in token"
            }));
        }
    };

    let query_result = get_user_by_id(user_id).await;

    match query_result {
        Some(user) => {
            println!("Contenido de data: {:?}", user);

            HttpResponse::Ok()
                .json(json!({"status": "success", "user": user}))
        }
        None => {
            HttpResponse::NotFound().json(json!({"status": "fail", "message": "El usuario no existe"}))
        }
    }
}

#[get("/my-access")]
async fn my_access(req: HttpRequest) -> impl Responder {


    println!("Authorization: {:?}", req.headers().get("authorization"));

    // Extrae el token de autorización del encabezado
    let access_token = match req.headers().get("authorization") {

        Some(header) => match header.to_str() {
            Ok(token) => {
                // Quitar "Bearer " del token, si está presente
                let token = if token.starts_with("Bearer ") {
                    &token[7..]
                } else {
                    token
                };

                token
            },
            Err(_) => {
                return HttpResponse::Unauthorized().json(json!({
                    "message": "Error extracting authorization token"
                }));
            }
        },
        None => {
            return HttpResponse::Unauthorized().json(json!({
                "message": "Authorization token missing"
            }));
        }
    };

    // Decodifica el token JWT
    let token_data = match decode::<TokenClaims>(
        access_token,
        &DecodingKey::from_secret("@#2023MovilCelistic".as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "message": "Invalid authorization token"
            }));
        }
    };

    // Parsea el ID de usuario
    let user_id = match token_data.claims._id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "message": "Invalid user ID in token"
            }));
        }
    };

    let query_result = get_user_access(user_id).await;

    match query_result {
        Some(acc) => {
            println!("String de data: {:?}", acc);

            HttpResponse::Ok()
                .json(json!({"status": "success", "data": acc}))
        }
        None => {
            HttpResponse::NotFound().json(json!({"status": "fail", "message": "No tiene permisos."}))
        }
    }

}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/account")
        .service(my_account)
        .service(my_access)
        .service(signin);

    conf.service(scope);
}


