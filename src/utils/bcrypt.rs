use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> String {
    hash(password, 4).unwrap()
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}
