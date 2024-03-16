use bcrypt::{hash, verify, DEFAULT_COST};
use std::error::Error;

pub fn _hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    let hashed_password = hash(password, DEFAULT_COST)?;
    Ok(hashed_password)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, Box<dyn Error>> {
    let result = verify(password, hashed_password)?;
    Ok(result)
}
