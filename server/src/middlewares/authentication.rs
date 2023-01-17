use crate::utils::codec::{self, MyClaims};
use protocol::short::{response::Status, Response};

pub fn auth(token: &str) -> Result<i64, Response> {
    match codec::parse(token) {
        Ok(MyClaims { uid }) => Ok(uid),
        Err(_) => Err(Response::from(Status::Unauthorized)),
    }
}
