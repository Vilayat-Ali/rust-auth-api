use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub email: String,
    pub password: String,
    pub exp: usize,
}

pub fn generate_jwt_token(
    username: String,
    email: String,
    password: String,
) -> errors::Result<String> {
    let claims: Claims = Claims {
        username,
        email,
        password,
        exp: 2,
    };

    let token: String = encode(
        &Header::new(Algorithm::RS512),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;

    Ok(token)
}

pub fn validate_jwt_token(jwt_token: &str) -> errors::Result<TokenData<Claims>> {
    let token = decode::<Claims>(
        jwt_token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::RS512),
    )?;

    Ok(token)
}