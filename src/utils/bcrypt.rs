use bcrypt::{hash, verify};
use log::{debug, error, info, warn};

pub fn hash_password(password: &str) -> String {
    debug!("Hashing password");
    match hash(password, 4) {
        Err(err) => {
            error!("Password hashing failed: {:?}", err);
            panic!("Hashing failed unexpectedly");
        }
        Ok(hashed_password) => {
            info!("Password hashing succeeded");
            hashed_password
        }
    }
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    debug!("Verifying password");
    match verify(password, hash) {
        Err(err) => {
            error!("Password verification failed: {:?}", err);
            false
        }
        Ok(is_valid) => {
            if !is_valid {
                warn!("Password verification failed: invalid credentials");
            }
            info!("Password verification succeeded");
            is_valid
        }
    }
}
