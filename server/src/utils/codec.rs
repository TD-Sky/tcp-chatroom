use jwt_simple::{prelude::*, Error};
use once_cell::sync::Lazy;
use std::{env, process::exit};

/// 自定义认证信息
#[derive(Serialize, Deserialize)]
pub struct MyClaims {
    pub uid: i64,
}

/// 密钥
static KEY: Lazy<HS256Key> = Lazy::new(|| {
    let Ok(secret) = env::var("SECRET") else {
        tracing::error!("environment variable not found: `{}`", "SECRET");
        exit(1);
    };
    HS256Key::from_bytes(secret.as_bytes())
});

pub fn gen(uid: i64) -> Result<String, Error> {
    let claims = Claims::with_custom_claims(MyClaims { uid }, Duration::from_days(7))
        .with_issuer("tcp-chatroom");

    KEY.authenticate(claims)
}

pub fn parse(token: &str) -> Result<MyClaims, Error> {
    KEY.verify_token::<MyClaims>(token, None)
        .map(|claims| claims.custom)
}
