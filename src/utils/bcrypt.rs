use bcrypt::{hash, verify};
use log::{debug, error, info, warn};

pub fn hash_password(password: &str) -> String {
    debug!("Starting password hashing");

    match hash(password, 4) {
        Ok(hashed_password) => {
            info!("Password hashing succeeded");
            hashed_password
        }
        Err(err) => {
            error!("Password hashing failed: {:?}", err);
            panic!("Hashing failed unexpectedly");
        }
    }
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    debug!("Starting password verification");

    match verify(password, hash) {
        Ok(is_valid) => {
            if is_valid {
                info!("Password verification succeeded");
            } else {
                warn!("Password verification failed: invalid credentials");
            }
            is_valid
        }
        Err(err) => {
            error!("Password verification failed: {:?}", err);
            false
        }
    }
}
