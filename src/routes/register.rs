use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::utils::hash::hash_string;

#[derive(Deserialize)]
pub struct RegisterReqBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn hello(user_info: web::Json<RegisterReqBody>) -> impl Responder {
    // hashing the password
    match hash_string(&user_info.password) {
        Ok(hashed_password) => {
            HttpResponse::Ok().body(hashed_password)
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Error while hashing the password")
        }
    }
}
