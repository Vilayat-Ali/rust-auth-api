use jsonwebtoken::{
    decode, encode, errors, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub exp: usize,
}

pub fn generate_access_token<'a>(username: &String, email: &String) -> errors::Result<String> {
    let claims: Claims = Claims {
        username,
        email,
        exp: 2,
    };

    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;

    Ok(token)
}

pub fn generate_refresh_token(username: &String, email: &String) -> errors::Result<String> {
    let claims: Claims = Claims {
        username,
        email,
        exp: 2,
    };

    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;

    Ok(token)
}

// pub fn validate_jwt_token(jwt_token: &str) -> errors::Result<TokenData<Claims>> {
//     let token = decode::<Claims>(
//         jwt_token,
//         &DecodingKey::from_secret("secret".as_ref()),
//         &Validation::default(),
//     )?;

//     Ok(token)
// }
