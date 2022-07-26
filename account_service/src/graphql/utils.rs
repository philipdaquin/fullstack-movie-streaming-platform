use actix_web::{Result};
use lazy_static::lazy_static;
use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};

lazy_static! {
    static ref PASSWORD_SECRET_KEY: String =
        std::env::var("PASSWORD_SECRET_KEY").expect("Can't read PASSWORD_SECRET_KEY");
}
pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}
pub fn verify_password(hash: &str, password: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}