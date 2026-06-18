use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

fn get_argon2<'a>() -> Argon2<'a> {
    Argon2::default()
}

pub fn password_hash(password: &[u8]) -> String {
    let argon2 = get_argon2();
    let salt = SaltString::generate(&mut OsRng);

    return argon2
        .hash_password(password, &salt)
        .expect("failed tp hash password ")
        .to_string();
}

// Fixed: Accepts a &str hash from your DB, parses it, and returns a boolean Result
pub fn password_verify(password: &[u8], stored_hash_str: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(stored_hash_str) else {
        tracing::error!(
            "Password verifiaction failed because the password stored in the DB is corrupted."
        );
        return false; // Returns false if the hash string format in DB is corrupted
    };

    get_argon2().verify_password(password, &parsed_hash).is_ok() // Returns true on success, false on invalid password
}
