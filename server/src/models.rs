use protocol::models::{GroupMessage, PrivateMessage, PublicMessage};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MaybeNewUser {
    pub id: Option<i64>,
    pub nickname: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Public(PublicMessage),
    Private(PrivateMessage),
    Group(GroupMessage),
}
