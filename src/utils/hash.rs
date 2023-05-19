use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

pub fn hash_string(data: String) -> BcryptResult<String> {
    let hashed: String = hash(data, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn validate_hash(unhashed_data: String, hashed_data: &str) -> BcryptResult<bool> {
    let is_valid: bool = verify(unhashed_data, hashed_data)?;
    Ok(is_valid)
}
