use argon2::{
    password_hash::{rand_core::OsRng, Error, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::utils::errors::AppError;

pub fn hash(password: &String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(hashed.to_string())
}

pub fn verify(hash: String, password: String) -> Result<bool, AppError> {
    let password_hash = PasswordHash::new(&hash)?;
    match Argon2::default().verify_password(password.as_bytes(), &password_hash) {
        Ok(()) => Ok(true),
        Err(Error::Password) => Ok(false),
        Err(err) => Err(AppError::from(err)),
    }
}
