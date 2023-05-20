use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::utils::hash::hash_string;
use crate::utils::jwt::{generate_access_token, generate_refresh_token};
use crate::APIResponseBuilder;

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterReqBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterResBody {
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn register(user_info: web::Json<RegisterReqBody>) -> impl Responder {
    // hashing the password
    match hash_string(&user_info.password) {
        Ok(_hashed_password) => {
            let access_token: String = generate_access_token(
                user_info.username.clone(),
                user_info.email.clone(),
                user_info.password.clone(),
            )
            .unwrap();

            let refresh_token: String = generate_refresh_token(
                user_info.username.clone(),
                user_info.email.clone(),
                user_info.password.clone(),
            )
            .unwrap();

            let res_body: String = APIResponseBuilder::default()
                .set_status(true)
                .set_message("User registered successfully!")
                .set_data(&RegisterResBody {
                    access_token,
                    refresh_token,
                })
                .build();

            HttpResponse::Ok().body(res_body)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error while hashing the password"),
    }
}
