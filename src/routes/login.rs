use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::utils::hash::validate_hash;
// use crate::utils::jwt::{generate_access_token, generate_refresh_token};
use crate::APIResponseBuilder;

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginReqBody {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginResBody {
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn login(user_info: web::Json<LoginReqBody>) -> impl Responder {
    // response
    let mut res_body: APIResponseBuilder<LoginResBody> = APIResponseBuilder::default();

    match validate_hash(
        &user_info.password,
        "$2b$12$FWhQEQtfoRFlw0MIvjtf3uz8olATCs3NegAZbNGVjBxyV6uJfMVAq",
    ) {
        Ok(is_valid) => match is_valid {
            true => {
                let response = res_body
                    .set_status(true)
                    .set_message("user logged in successfully!")
                    .set_data(LoginResBody {
                        access_token: String::new(),
                        refresh_token: String::new(),
                    })
                    .build();

                HttpResponse::Ok().body(response)
            }
            false => {
                let response = res_body
                    .set_status(false)
                    .set_message("Invalid password")
                    .build();

                HttpResponse::Forbidden().body(response)
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Failed to run validations for password")
        }
    }
}
