use bcrypt::{hash, verify, DEFAULT_COST};
use log::info;
use std::error::Error;

pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    info!("Hashing password");
    let hashed_password = hash(password, DEFAULT_COST)?;
    Ok(hashed_password)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, Box<dyn Error>> {
    info!("Verifying password");
    let result = verify(password, hashed_password)?;
    Ok(result)
}
