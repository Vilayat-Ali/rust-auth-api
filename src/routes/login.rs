use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::db::get_user_by_email;
use crate::utils::hash::validate_hash;
use crate::utils::{generate_access_token, generate_refresh_token};
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

    let user_data: (String, String, String) = get_user_by_email(&user_info.email).await;

    if &user_data.0 == &String::default() && &user_data.1 == &String::default() {
        return HttpResponse::Forbidden().body("User doesn't exists");
    }

    match validate_hash(&user_info.password, &user_data.clone().2) {
        Ok(is_valid) => match is_valid {
            true => {
                let access_token: String =
                    generate_access_token(&user_data.0, &user_data.1).unwrap();

                let refresh_token: String =
                    generate_refresh_token(&user_data.0, &user_data.1).unwrap();

                let response: String = res_body
                    .set_status(true)
                    .set_message("user logged in successfully!")
                    .set_data(LoginResBody {
                        access_token,
                        refresh_token,
                    })
                    .build();

                HttpResponse::Ok().body(response)
            }
            false => {
                let response: String = res_body
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
