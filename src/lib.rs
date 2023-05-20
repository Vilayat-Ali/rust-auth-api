extern crate bcrypt;

pub mod db;
pub mod routes;
pub mod utils;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct APIResponseWithData<'a, T> {
    pub status: bool,
    pub message: &'a str,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct APIResponseWithoutData<'a> {
    pub status: bool,
    pub message: &'a str,
}

pub struct APIResponseBuilder<'a, T> {
    pub status: Option<bool>,
    pub message: Option<&'a str>,
    pub data: Option<T>,
}

impl<'a, T> APIResponseBuilder<'a, T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self {
            status: None,
            message: None,
            data: None,
        }
    }

    pub fn set_status(&mut self, status: bool) -> &mut APIResponseBuilder<'a, T> {
        self.status = Some(status);
        self
    }

    pub fn set_message(&mut self, message: &'a str) -> &mut APIResponseBuilder<'a, T> {
        self.message = Some(message);
        self
    }

    pub fn set_data(&mut self, data: T) -> &mut APIResponseBuilder<'a, T> {
        self.data = Some(data);
        self
    }

    pub fn build(&mut self) -> String
    where
        T: Clone,
    {
        match &self.data {
            Some(res_data) => {
                let res_payload: APIResponseWithData<T> = APIResponseWithData {
                    status: self
                        .status
                        .expect("'status' not defined in APIResponseWithData in response builder"),
                    message: self
                        .message
                        .expect("'message' not defined in APIResponseWithData in response builder"),
                    data: res_data.clone(),
                };

                match serde_json::to_string_pretty(&res_payload) {
                    Ok(response) => response,
                    Err(_) => String::from("Failed to generate response"),
                }
            }
            None => {
                let res_payload: APIResponseWithoutData = APIResponseWithoutData {
                    status: self
                        .status
                        .expect("'status' not defined in APIResponseWithData in response builder"),
                    message: self
                        .message
                        .expect("'message' not defined in APIResponseWithData in response builder"),
                };

                match serde_json::to_string_pretty(&res_payload) {
                    Ok(response) => response,
                    Err(_) => String::from("Failed to generate response"),
                }
            }
        }
    }
}
