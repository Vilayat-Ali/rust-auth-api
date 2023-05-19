use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

pub fn hash_string(data: String) -> BcryptResult<String> {
    let hashed: String = hash("hunter2", DEFAULT_COST)?;
    Ok(hash)
}

pub fn validate_hash(unhashed_data: &str, hashed_data: String) -> BcryptResult<bool> {
    let is_valid: bool = verify("hunter2", unhashed_data)?;
    Ok(is_valid)
}
