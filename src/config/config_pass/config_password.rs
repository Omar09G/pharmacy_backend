use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, SaltString};

pub fn get_password_salt() -> String {
    std::env::var("PASSWORD_SALT").unwrap_or_else(|_| "default_salt".to_string())
}
pub fn generate_hash(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;
    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}
