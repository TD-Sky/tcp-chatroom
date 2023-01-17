use once_cell::sync::Lazy;
use sha2::{Digest, Sha256};
use std::{env, process::exit};

static PASSWORD_HASHER: Lazy<Sha256> = Lazy::new(|| {
    let Ok(salt) = env::var("SALT") else {
        tracing::error!("environment variable not found: `{}`", "SALT");
        exit(1);
    };
    Sha256::new_with_prefix(salt)
});

pub fn hash(password: &str) -> String {
    format!(
        "{:x}",
        PASSWORD_HASHER.clone().chain_update(password).finalize()
    )
}
