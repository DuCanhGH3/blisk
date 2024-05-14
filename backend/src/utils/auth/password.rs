use argon2::{
    password_hash::{rand_core::OsRng, Error, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(password_hash) => Ok( password_hash.to_string()),
        Err(err) => Err(err)
    }
}

pub fn verify(hash: String, password: String) -> Result<(), argon2::password_hash::Error> {
    let password_hash = PasswordHash::new(&hash)?;
    Argon2::default().verify_password(password.as_bytes(), &password_hash)
}
