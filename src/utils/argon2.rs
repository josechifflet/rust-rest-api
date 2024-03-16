use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[tracing::instrument(name = "Hashing user password", skip(password))]
pub fn hash(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password, &salt)
        .expect("Unable to hash password.")
        .to_string()
}

#[tracing::instrument(name = "Verifying user password", skip(password, hash))]
pub fn verify_password(hash: &str, password: &[u8]) -> bool {
    let parsed_hash = PasswordHash::new(hash).expect("Unable to parse hash.");
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .map_or(false, |_| true)
}